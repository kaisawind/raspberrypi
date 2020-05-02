extern crate sysfs_gpio;
extern crate ctrlc;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let led_pin = Pin::new(26); // number depends on chip, etc.
    
    led_pin.with_exported(|| {
        sleep(Duration::from_millis(80));
        led_pin.set_direction(Direction::Out)?;
        while running.load(Ordering::SeqCst) {
            led_pin.set_value(1)?;
            sleep(Duration::from_millis(200));
            led_pin.set_value(0)?;
            sleep(Duration::from_millis(200));
        }
        Ok(())
    }).unwrap();
}
