extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // UX2-1 ON
    let vibration_pin = Pin::new(27); // 震动小马达
    vibration_pin.with_exported(|| {
        sleep(Duration::from_millis(80));
        vibration_pin.set_direction(Direction::Out)?;
        vibration_pin.set_value(1)?;
        sleep(Duration::from_millis(2000));
        vibration_pin.set_value(0)?;
        Ok(())
    }).unwrap();
}
