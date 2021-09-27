/*! This file contains a C interface for an `Engine`. This can then be wrapped by `cbindgen` to create a C interface, and then by SWIG to create a Java interface.

Hopefully, this code should not require many changes to be used with any engine that satisfies the [EngineTrait]. What is important is to ensure that `WumpusUserHandle` points to a [UserHandle] of the required `Engine`.

For further information, see the documentation for [narthex_engine_trait], starting with [narthex_engine_trait::EngineTrait].
*/
use libc::c_char;
extern crate android_log;
extern crate log;
use anyhow::Result;
use narthex_engine_trait::{
    ActionTrait, ConfigTrait, EngineTrait, Event, InterfaceType, ResponseTrait,
};
use serde::Serialize;
use std::{
    ffi::{CStr, CString},
    fmt::Debug,
    ops::DerefMut,
    panic,
    panic::AssertUnwindSafe,
    sync::{Arc, Mutex},
};

// -- generic code ---
/// A `UserHandle` is an opaque pointer an `UserData`, and so to an `Engine`.
/// This will be passed to C (and Java).
pub struct UserHandle<Engine: EngineTrait> {
    ptr: Arc<Mutex<UserData<Engine>>>,
}
/** A `UserData` is the data that persists. This must be explicitly deleted. */
#[derive(Debug, Default)]
pub struct UserData<Engine: EngineTrait> {
    engine: Engine,
    last_response: Engine::Response,
    last_string: CString,
}
impl<Engine> UserData<Engine>
where
    Engine: EngineTrait + Default,
    Engine::Response: Default,
{
    /** `new` creates a new UserData */
    pub fn new(engine: Engine) -> Result<Self> {
        Ok(Self {
            engine,
            last_response: Engine::Response::default(),
            last_string: CString::new("")?,
        })
    }
}
fn string_from_c(s: *const c_char) -> String {
    unsafe { CStr::from_ptr(s).to_string_lossy().into_owned() }
}
macro_rules! data_from_handle {
    ($data:expr) => {
        (*$data)
            .ptr
            .lock()
            .expect("bad mut wumpus data")
            .deref_mut()
    };
}
macro_rules! do_with_data {
    ($doit:expr) => {{
        let r3 = AssertUnwindSafe(|| $doit);
        panic::catch_unwind(r3).unwrap();
    }};
}

// --- this bit for [[Engine]] ---

/// **This is the line that will need to be changed.**
type WumpusUserHandle = UserHandle<engine::Engine>;
// ****

/// create a [[UserData]]. This code is dependent on the app.
/// # Safety
/// data pointer must be valid
#[no_mangle]
pub unsafe extern "C" fn new_engine(config_json: *const c_char) -> *mut WumpusUserHandle {
    internal_new_engine(config_json)
}
fn internal_new_engine<Engine>(config_json: *const c_char) -> *mut UserHandle<Engine>
where
    Engine: EngineTrait + Default,
    // UserData<Engine>: Default,
    Engine::Config: ConfigTrait + Debug,
    Engine::Response: Default,
{
    android_log::init("wumpus").expect("could not init log");
    log::debug!("creating engine for user data");
    let config_str = string_from_c(config_json);
    let config = Engine::Config::from_json(&config_str).unwrap_or_else(|e| {
        log::error!("config error: {:?}", &e);
        panic!("bad Config")
    });
    log::debug!("have config {:?}", &config);
    let data = panic::catch_unwind(AssertUnwindSafe(|| {
        let engine = Engine::new(&config, InterfaceType::Android).unwrap_or_else(|e| {
            log::error!("error creating engine {:?}", &e);
            Engine::default()
        });
        log::debug!(
            "have engine for user data {:?}",
            engine.get_interface_type()
        );
        let user_data = UserData::new(engine).unwrap_or_else(|e| {
            log::error!("user data error {:?}", &e);
            UserData::new(Engine::default()).expect("more bad user data")
        });
        log::debug!("user data generated");
        user_data
    }))
    .unwrap_or_else(|e| {
        log::error!("panic! {:?}", &e);
        panic!("bad engine!");
    });
    log::trace!("created engine");
    Box::into_raw(Box::new(UserHandle {
        ptr: (Arc::new(Mutex::new(data))),
    }))
}
#[no_mangle]
/// deletes the main data structure. . This code is dependent on the app.
/// # Safety
/// data pointer must be valid
pub unsafe extern "C" fn delete_engine(data: *mut WumpusUserHandle) {
    log::debug!("deleting engine...");
    let _b = Box::from_raw(data);
}
// pub unsafe extern "C" fn delete_engine<Engine>(data: *mut UserHandle<Engine>)
// where
//     Engine: EngineTrait,
// {
//     let _b = Box::from_raw(data);
// }

// --- this bit for [[EngineTrait]] ---

#[no_mangle]
/// get the most recent string
/// # Safety
/// data pointer must be valid
pub unsafe extern "C" fn last_string(data: *mut WumpusUserHandle) -> *const c_char {
    internal_last_string(data)
}
unsafe fn internal_last_string<Engine>(data: *mut UserHandle<Engine>) -> *const c_char
where
    Engine: EngineTrait,
{
    data_from_handle!(data).last_string.as_ptr()
}
//---- execute ----
#[no_mangle]
/// execute an action (just wraps the engine call)
/// # Safety
/// data pointer must be valid
pub unsafe extern "C" fn execute(data: *mut WumpusUserHandle, body: *const c_char) {
    internal_execute(data, body)
}
unsafe fn internal_execute<Engine>(data: *mut UserHandle<Engine>, body: *const c_char)
where
    Engine: EngineTrait,
    Engine::Action: ActionTrait,
{
    execute_inner(data_from_handle!(data), string_from_c(body));
}
/// safe calculations for execute
fn execute_inner<Engine>(d: &mut UserData<Engine>, bs: String)
where
    Engine: EngineTrait,
    Engine::Action: ActionTrait,
{
    log::debug!("executing {}", &bs);
    do_with_data!({
        d.last_response = d
            .engine
            .execute(Engine::Action::from_json(&bs).unwrap_or_else(|err| {
                log::error!("bad action {}", &err);
                panic!("bad action {}", &err);
            }))
            .unwrap()
    });
}
#[no_mangle]
//---- handle_event ------
/// handle an event (just wraps the engine call)
/// # Safety
/// data pointer must be valid
pub unsafe extern "C" fn handle_event(data: *mut WumpusUserHandle, body: *const c_char) {
    //pub unsafe extern "C" fn handle_event(data: *mut WumpusUserHandle, event:
    // Event) {
    internal_handle_event(data, body)
}
unsafe fn internal_handle_event<Engine>(data: *mut UserHandle<Engine>, body: *const c_char)
where
    Engine: EngineTrait,
    Engine::Response: Default,
{
    handle_event_inner(data_from_handle!(data), string_from_c(body));
}
/// safe calculations for handle_event
fn handle_event_inner<Engine>(d: &mut UserData<Engine>, bs: String)
where
    Engine: EngineTrait,
    Engine::Response: Default,
{
    log::debug!("handling '{:?}'", &bs);
    do_with_data!({
        // log::debug!("handle event inner");
        match Event::from_json(&bs) {
            Err(e) => {
                log::error!("bad event {:?}!", &e);
            }
            Ok(ev) => {
                d.last_response = d.engine.handle_event(&ev).unwrap_or_else(|e| {
                    ("bad handle event {:?}: {:?}", &ev, &e);
                    Engine::Response::default()
                })
            }
        }
    });
}

#[no_mangle]
/// creates the initial HTML  (just wraps the engine call)
/// # Safety
/// data pointer must be valid
pub unsafe extern "C" fn initial_html(data: *mut WumpusUserHandle) -> *const c_char {
    internal_initial_html(data)
}
unsafe fn internal_initial_html<Engine>(data: *mut UserHandle<Engine>) -> *const c_char
where
    Engine: EngineTrait,
{
    initial_html_inner(data_from_handle!(data))
}
/// safe calculations for initial_html
fn initial_html_inner<Engine>(d: &mut UserData<Engine>) -> *const c_char
where
    Engine: EngineTrait,
{
    log::debug!("getting init html");
    do_with_data!({
        let html = d.engine.initial_html().unwrap_or_else(|e| {
            log::error!("error {:?}", &e);
            "".to_string()
        });
        log::debug!("got init html");
        d.last_string = CString::new(html).unwrap();
    });
    d.last_string.as_ptr()
}
#[no_mangle]
/// whether the response requires the application to be shut down
/// # Safety
/// data pointer must be valid
pub unsafe extern "C" fn is_shutdown_required(data: *const WumpusUserHandle) -> bool {
    internal_is_shutdown_required(data)
}
unsafe fn internal_is_shutdown_required<Engine>(data: *const UserHandle<Engine>) -> bool
where
    Engine: EngineTrait,
    Engine::Response: ResponseTrait + Default + Serialize,
{
    data_from_handle!(data).last_response.shutdown_required()
}
#[no_mangle]
/// creates the JSON-encoded response  (just wraps the engine call)
/// # Safety
/// data pointer must be valid
pub unsafe extern "C" fn last_response_json(data: *mut WumpusUserHandle) -> *const c_char {
    log::debug!("getting last response...");
    internal_last_response_json(data)
}
unsafe fn internal_last_response_json<Engine>(data: *mut UserHandle<Engine>) -> *const c_char
where
    Engine: EngineTrait,
    Engine::Response: ResponseTrait + Default + Serialize,
{
    last_response_inner(data_from_handle!(data))
}
/// safe calculations for last_response_json
fn last_response_inner<Engine>(d: &mut UserData<Engine>) -> *const c_char
where
    Engine: EngineTrait,
    Engine::Response: ResponseTrait + Default + Serialize,
{
    do_with_data!({
        let lr_json: String =
            serde_json::ser::to_string(&d.last_response).expect("cannot serialise");
        d.last_string = CString::new(lr_json).expect("bad string");
    });
    d.last_string.as_ptr()
}
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
