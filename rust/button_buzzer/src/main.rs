extern crate sysfs_gpio;
extern crate ctrlc;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let button_pin = Pin::new(26); // 按键
    let buzzer_pin = Pin::new(18); // 蜂鸣器
    
    button_pin.export().unwrap();
    buzzer_pin.export().unwrap();
    sleep(Duration::from_millis(80));

    button_pin.set_direction(Direction::In).unwrap();
    buzzer_pin.set_direction(Direction::Out).unwrap();

    while running.load(Ordering::SeqCst) {
        let value = match button_pin.get_value() {
            Ok(value) => value,
            Err(_) => continue,
        };
        if value == 0  {
            match buzzer_pin.set_value(1) {
                Ok(()) => println!("buzzer is on ..."),
                Err(_) => continue,
            };
        } else {
            match buzzer_pin.set_value(0){
                Ok(()) => println!("buzzer is off ..."),
                Err(_) => continue,
            };
        }
        sleep(Duration::from_millis(10));
    }

    match button_pin.unexport(){
        Ok(()) => println!("button unexport ok"),
        Err(_) => println!("button unexport error"),
    };
    match buzzer_pin.unexport(){
        Ok(()) => println!("buzzer unexport ok"),
        Err(_) => println!("buzzer unexport error"),
    };
}
