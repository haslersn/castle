#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod hinge;
mod led;
mod lock;
mod server;
mod util;

use crate::util::Result;
use env_logger::Builder;
use env_logger::Env;
use hinge::Hinge;
use led::LedController;
use lock::Lock;
use mcp23x17::Expander;
use server::ServerSettings;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Deserialize)]
struct Config {
    expander_device: String,
    output_pins: OutputPinsSettings,
    input_pins: InputPinsSettings,
    server: ServerSettings,
}

#[derive(Deserialize)]
struct OutputPinsSettings {
    green_leds: Vec<u8>,
    red_leds: Vec<u8>,
    lock: u8,
}

#[derive(Deserialize)]
struct InputPinsSettings {
    hinge: u8,
}

fn read_config() -> Result<Config> {
    let conf_str = std::fs::read_to_string("castle.toml")?;
    toml::from_str(&conf_str).map_err(From::from)
}

fn main() -> Result {
    // initialize logger w/ log level "info"
    Builder::from_env(Env::new().default_filter_or("info")).init();

    let conf = read_config()?;

    let expander = Expander::new(&conf.expander_device)?;
    let green_leds = conf
        .output_pins
        .green_leds
        .iter()
        .map(|&pin| expander.output(pin))
        .collect();
    let red_leds = conf
        .output_pins
        .red_leds
        .iter()
        .map(|&pin| expander.output(pin))
        .collect();
    let lock = Arc::new(Mutex::new(Lock::new(
        expander.output(conf.output_pins.lock),
    )?));
    let hinge = Arc::new(Mutex::new(Hinge::new(
        expander.input(conf.input_pins.hinge),
    )));

    let mut led_ctrl = LedController::new(green_leds, red_leds, lock.clone(), hinge.clone());

    crossbeam::thread::scope(|s| {
        // spawn LED controller thread
        s.spawn(|_| {
            led_ctrl.run();
        });

        // spawn web server thread
        s.spawn(|_| {
            server::run(conf.server, hinge, lock.clone());
        });
    })
    .unwrap();

    Ok(())
}
