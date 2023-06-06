use std::thread;
use std::time::Duration;

use anyhow::Result;

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
    kv_store.set("status", "ready".as_bytes())?;

    while kv_store.get("status")? != "enqueued".as_bytes() {
        thread::sleep(Duration::from_millis(250));
    }

    kv_store.set("status", "running".as_bytes())?;

    while kv_store.get("status")? != "dequeued".as_bytes() {
        output_pin.write(LogicLevel::High);
        sleep(input_pin.read());
        output_pin.write(LogicLevel::Low);
        sleep(input_pin.read());
    }

    kv_store.set("status", "stopped".as_bytes())?;
    Ok(())
}
