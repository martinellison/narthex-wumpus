/*! main program for PC. Uses embedded web browser */

use anyhow::Result;
use log::debug;
use narthex_engine_trait::{EngineTrait, InterfaceType};
use narthex_web_app::{UserData, WebParams};
use simplelog::{LevelFilter, SimpleLogger};
use structopt::StructOpt;

/** main program */
fn main() {
    match main_inner() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("error {:?}", err)
        }
    }
}
/// command line options
#[derive(StructOpt, Debug)]
#[structopt(name = "wumpus")]
struct Options {
    // /// Config file
    // #[structopt(short, long, parse(from_os_str))]
    // config: PathBuf,
    /// Whether to show extra debug trace
    #[structopt(short, long)]
    verbose: bool,
    /// Whether to show webview debug trace
    #[structopt(short, long)]
    debug: bool,
}
/// actually run everything
fn main_inner() -> Result<()> {
    let opt = Options::from_args();
    SimpleLogger::init(LevelFilter::Trace, simplelog::Config::default())?;
    let verbose = opt.verbose;
    if verbose {
        debug!("options: {:?}", opt);
    }
    debug!("running engine with webview...");
    run_engine_with_webview(&opt)?;
    debug!("finished running engine with webview");
    Ok(())
}
/// build the web view and run the engine
fn run_engine_with_webview(opt: &Options) -> Result<()> {
    if opt.verbose {
        debug!("building webview...");
    }
    //let config_text = fs::read_to_string(&opt.config)?;
    //  let config: engine::Config = toml::from_str(&config_text)?;
    let config = engine::Config::default();
    let engine = engine::Engine::new(&config, InterfaceType::PC)?;
    let user_data = UserData::new(engine);
    let params = WebParams {
        title: "Wumpus".to_string(),
        height: 1500,
        debug: opt.debug,
        verbose: opt.verbose,
        ..WebParams::default()
    };
    user_data.run_engine_with_webview(params)?;
    debug!("run, ending main.");
    Ok(())
}
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
