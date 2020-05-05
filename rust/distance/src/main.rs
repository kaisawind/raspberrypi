extern crate rppal;

use std::error::Error;
use rppal::gpio::Gpio;

const TRIG:u8 = 16; // 超声触发器
const ECHO:u8 = 12; // 超声接收器

fn main() -> Result<(), Box<dyn Error>> {
    println!("Distance Measurement In Progress");
    // Retrieve the GPIO pin and configure it as an output.
    let echo_pin = Gpio::new()?.get(ECHO)?.into_input();
    let trig_pin = Gpio::new()?.get(TRIG)?.into_output();

    Ok(())
}
