use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};

use gpio::*;
use keyvalue::*;

wit_bindgen_rust::import!("../../wit/gpio.wit");
wit_error_rs::impl_error!(GpioError);

wit_bindgen_rust::import!("../../wit/keyvalue.wit");
wit_error_rs::impl_error!(KeyvalueError);

fn sleep(input: LogicLevel) {
    thread::sleep(Duration::from_millis(match input {
        LogicLevel::Low => 500,
        LogicLevel::High => 250,
    }));
}

fn main() -> Result<()> {
    let input_pin = InputPin::get_named("push_down_button")?;
    let output_pin = OutputPin::get_named("led")?;

    let kv_store = Keyvalue::open("gpio-demo-control")?;
    kv_store.set("status", "disabled".as_bytes())?;

    fn run_disabled() {
        thread::sleep(Duration::from_millis(250));
    }

    let run_enabled = || {
        output_pin.write(LogicLevel::High);
        sleep(input_pin.read());
        output_pin.write(LogicLevel::Low);
        sleep(input_pin.read());
    };

    loop {
        match String::from_utf8(kv_store.get("status")?)?.as_str() {
            "disabling" => {
                kv_store.set("status", "disabled".as_bytes())?;
                run_disabled();
            }
            "disabled" => run_disabled(),
            "enabling" => {
                kv_store.set("status", "enabled".as_bytes())?;
                run_enabled();
            }
            "enabled" => run_enabled(),
            "stopping" => {
                kv_store.set("status", "stopped".as_bytes())?;
                return Ok(());
            }
            unknown => {
                return Err(anyhow!("unknown state {unknown}"));
            }
        }
    }
}
