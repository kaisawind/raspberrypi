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

    // UX3-6 ON
    let sound_pin = Pin::new(24); // 声音传感器
    
    sound_pin.with_exported(|| {
        sleep(Duration::from_millis(80));
        sound_pin.set_direction(Direction::In)?;
        while running.load(Ordering::SeqCst) {
            let value = sound_pin.get_value()?;
            if value == 0 {
                println!("Sound Detected");
                sleep(Duration::from_millis(100));
            }
            sleep(Duration::from_millis(10));
        }
        Ok(())
    }).unwrap();
}
