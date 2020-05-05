extern crate rppal;

use rppal::gpio::Gpio;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

const TRIG: u8 = 16; // 超声触发器
const ECHO: u8 = 12; // 超声接收器

fn main() -> Result<(), Box<dyn Error>> {
    println!("Distance Measurement In Progress");
    // Retrieve the GPIO pin and configure it as an output.
    let echo_pin = Gpio::new()?.get(ECHO)?.into_input();
    let mut trig_pin = Gpio::new()?.get(TRIG)?.into_output();

    trig_pin.set_low();
    println!("Waiting For Sensor To Settle");
    sleep(Duration::from_secs(2));

    trig_pin.set_high();
    sleep(Duration::from_micros(10));
    trig_pin.set_low();

    let mut pulse_start: Instant = Instant::now();
    let mut pulse_end: Instant = Instant::now();
    while echo_pin.is_low() {
        pulse_start = Instant::now();
    }
    while echo_pin.is_high() {
        pulse_end = Instant::now();
    }

    let pulse_duration = pulse_end - pulse_start;
    let distance = (pulse_duration.as_secs() * 34000) as f64 / 2.0; // 声音传播速度340m/s
    println!("Distance: {:.2} cm", distance);
    Ok(())
}
