extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let buzzer_pin = Pin::new(18); // number depends on chip, etc.
    buzzer_pin.with_exported(|| {
        sleep(Duration::from_millis(80));
        buzzer_pin.set_direction(Direction::Out)?;
        buzzer_pin.set_value(1)?;
        sleep(Duration::from_millis(500));
        buzzer_pin.set_value(0)?;
        Ok(())
    }).unwrap();
}
