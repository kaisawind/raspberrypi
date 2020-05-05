use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

fn main() -> Result<(), Box<dyn Error>> {
    // Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
    let motion_pin: u8 = 23;
    // Retrieve the GPIO pin and configure it as an output.
    let pin = Gpio::new()?.get(motion_pin)?.into_input();

    loop {
        if pin.is_low() {
            println!("Nothing moves ...");
        } else {
            println!("Motion detected ...");
        }
        thread::sleep(Duration::from_millis(500));
    }
}
