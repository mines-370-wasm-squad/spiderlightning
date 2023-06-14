use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};

use gpio::*;
use keyvalue::*;

wit_bindgen_rust::import!("../../wit/gpio.wit");
wit_error_rs::impl_error!(GpioError);

wit_bindgen_rust::import!("../../wit/keyvalue.wit");
wit_error_rs::impl_error!(KeyvalueError);

const BLINK_THRESHOLD: u32 = 500;

fn main() -> Result<()> {
    let input_pin = InputPin::get_named("push_down_button")?;
    let output_pin = OutputPin::get_named("led")?;
    let pwm_control_pin = InputPin::get_named("pwm_control_button")?;
    let pwm_output_pin = PwmOutputPin::get_named("pwm_led")?;

    let kv_store = Keyvalue::open("gpio-demo-control")?;
    kv_store.set("status", "disabled".as_bytes())?;

    let mut blink_current = LogicLevel::Low;
    let mut pwm_duty_cycle: f32 = 0.0;

    output_pin.write(LogicLevel::Low);
    pwm_output_pin.set_duty_cycle(0.0);
    pwm_output_pin.disable();

    loop {
        match String::from_utf8(kv_store.get("status")?)?.as_str() {
            "disabled" => {
                output_pin.write(LogicLevel::Low);
                pwm_output_pin.disable();
                blink_current = LogicLevel::Low;
                pwm_duty_cycle = 0.0;
                thread::sleep(Duration::from_millis(200));
            }
            "enabled" => {
                blink_current = match blink_current {
                    LogicLevel::Low => LogicLevel::High,
                    LogicLevel::High => LogicLevel::Low,
                };
                output_pin.write(blink_current);
                let mut blink_progress = 0;
                while blink_progress < BLINK_THRESHOLD {
                    pwm_duty_cycle = (pwm_duty_cycle
                        + match pwm_control_pin.read() {
                            LogicLevel::Low => -0.001,
                            LogicLevel::High => 0.001,
                        })
                    .clamp(0.0, 1.0);
                    pwm_output_pin.set_duty_cycle(pwm_duty_cycle);
                    blink_progress += match input_pin.read() {
                        LogicLevel::Low => 1,
                        LogicLevel::High => 2,
                    };

                    thread::sleep(Duration::from_millis(1));
                }
            }
            "stopped" => {
                return Ok(());
            }
            unknown => {
                return Err(anyhow!("unknown state {unknown}"));
            }
        }
    }
}
