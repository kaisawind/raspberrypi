extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let relay_pin = Pin::new(21); // 继电器
    relay_pin.with_exported(|| {
        sleep(Duration::from_millis(80));
        relay_pin.set_direction(Direction::Out)?;
        relay_pin.set_value(0)?;
        sleep(Duration::from_millis(500));
        relay_pin.set_value(1)?;
        Ok(())
    }).unwrap();
}
