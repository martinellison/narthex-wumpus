#!/bin/bash
RUSTBUILDTYPE=${1:-release}
if [[ "$RUSTBUILDTYPE" == "release" ]]; then
    RUSTBUILD="--release"
else
    RUSTBUILD=""
fi
export BASE=$(git rev-parse --show-toplevel)
if [[ "$BASE" == "" ]]; then
    echo "need to be in the git repository"
    exit 1
fi
echo "building rust..."
WUMPUS_C_DIR=$BASE/wumpus-c
cd $WUMPUS_C_DIR
# cargo build --$RUSTBUILDTYPE

echo "cbindgen..."
CBINDGEN_TARG=$WUMPUS_C_DIR/wumpus-c.h
cbindgen $WUMPUS_C_DIR/src/lib.rs --output $CBINDGEN_TARG --lang c

echo "swig..."
export SWIGDIR=$WUMPUS_C_DIR/swig
if [[ ! -d $SWIGDIR ]]; then mkdir -p $SWIGDIR; fi
rm -rf $SWIGDIR/*
swig -outdir $SWIGDIR -java -package wumpus $WUMPUS_C_DIR/wumpus.i

echo "finding java..."
JAVAINCL=${JAVA_HOME}/include
if [[ ! -d ${JAVAINCL} ]]; then
    echo "cannot find java includes" $JAVAINCL
    exit 9
fi

export ANDROID_HOME=$HOME/work/android/sdk
if [[ ! -d ${ANDROID_HOME} ]]; then
    echo "cannot find android SDK" $ANDROID_HOME
    exit 9
fi
NDK_VERSION=23.0.7599858
export NDK_HOME=$ANDROID_HOME/ndk/$NDK_VERSION
if [[ ! -d ${NDK_HOME} ]]; then
    echo "cannot find android NDK" $NDK_HOME
    ls $ANDROID_HOME/ndk/
    exit 9
fi

export ANDLEV=26
HOST_TAG=linux-x86_64
PKGNAME=wumpus-c
PKGNAMEMOD=${PKGNAME//-/_}

for ARCH in aarch64 x86_64 armv7; do
    echo "---- building for" $ARCH " ----------"

    case $ARCH in
    aarch64)
        # export ARCHID=arm64-v8a
        export RUST_TARGET=aarch64-linux-android
        export ANDR_CLANG_ARCH=aarch64-linux-android
        export JNIDIR=arm64-v8a
        # export TOOLARCH=aarch64-linux-android
        ;;
    armv7)
        # export ARCHID=armeabi-v7a
        export RUST_TARGET=armv7-linux-androideabi
        export ANDR_CLANG_ARCH=armv7a-linux-androideabi
        export JNIDIR=armeabi-v7a
        # export TOOLARCH=arm-linux-androideabi
        ;;
    x86_64)
        # export ARCHID=x86
        export RUST_TARGET=x86_64-linux-android
        export ANDR_CLANG_ARCH=x86_64-linux-android
        export JNIDIR=x86_64
        # export TOOLARCH=x86_64-linux-android
        ;;
    esac
    cd $WUMPUS_C_DIR
    echo "building rust for '" $RUST_TARGET "', package '" $PKGNAME "'..."
    cargo build --target $RUST_TARGET -p $PKGNAME $RUSTBUILD
    if [[ $? != 0 ]]; then
        echo "---- build failed for $ARCH ----"
        exit 1
    fi
    export TOOLCHAIN=$NDK_HOME/toolchains/llvm/prebuilt/$HOST_TAG
    if [[ ! -d "$TOOLCHAIN" ]]; then
        echo "toolchain does not exist" $TOOLCHAIN "for" $ARCH
        exit 1
    fi
    echo "TOOLCHAIN is" $TOOLCHAIN
    export CC=$TOOLCHAIN/bin/$ANDR_CLANG_ARCH$ANDLEV-clang
    # export CXX=$TOOLCHAIN/bin/$ANDR_CLANG_ARCH$ANDLEV-clang++
    export ARDIR=$TOOLCHAIN/bin
    export AR=$ARDIR/llvm-ar
    echo "cc" $CC "ar" $AR
    if [[ ! -f "$CC" ]]; then
        echo "c compiler does not exist" $CC "for" $ARCH
        exit 1
    fi
    if [[ ! -f "$AR" ]]; then
        echo "c archiver does not exist" $AR "for" $ARCH "TOOLCHAIN" $TOOLCHAIN "TOOLARCH" $TOOLARCH "ANDR CLANG ARCH" $ANDR_CLANG_ARCH "ANDLEV" $ANDLEV
        ls $ARDIR
        exit 1
    fi
    LIBNAME=wumpus
    RUSTEX_ANDR="$WUMPUS_C_DIR/target/$RUST_TARGET/$RUSTBUILDTYPE/lib$PKGNAMEMOD.a"
    if [[ ! -f $RUSTEX_ANDR ]]; then
        echo "no archive file for " $PKGNAME "path" $RUSTEX_ANDR
        exit 0
    fi
    cp $RUSTEX_ANDR $SWIGDIR
    echo "android c compile... "
    rm -f $SWIGDIR/*.so
    SONAME=lib$LIBNAME.so
    $CC -shared $WUMPUS_C_DIR/wumpus_wrap.c $RUSTEX_ANDR -lm -llog -lz -o $SWIGDIR/$SONAME -I"${JAVALOC}/include" -I"${JAVALOC}/include/linux" -I $BASE -fPIC
    if [[ $? != 0 ]]; then exit 1; fi
    ANDDIR=$BASE/Wumpus/app/src/main
    ANDLIBDIR=$ANDDIR/jniLibs
    ANDLIBDIR64=$ANDLIBDIR/$JNIDIR
    ANDJAVADIR=$ANDDIR/java/$LIBNAME
    if [[ ! -d $ANDJAVADIR ]]; then mkdir -p $ANDJAVADIR; fi
    if [[ ! -d $ANDLIBDIR64 ]]; then mkdir -p $ANDLIBDIR64; fi
    echo "copying" $SONAME "to" $ANDLIBDIR64
    rm $ANDLIBDIR64/*
    cp $SWIGDIR/$SONAME $ANDLIBDIR64
    echo "copying generated java to" $ANDJAVADIR
    rm $ANDJAVADIR/*
    cp $SWIGDIR/*.java $ANDJAVADIR/
    cd $BASE/Wumpus
    echo $ARCH "built"
done
