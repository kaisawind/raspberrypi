#![allow(dead_code)]
extern crate bitflags;
extern crate ctrlc;
extern crate i2cdev;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

const NUNCHUCK_SLAVE_ADDR: u16 = 0x5c; // 默认设备I2C地址

#[repr(u8)]
enum InstructionSet {
    PowerDown = 0b0000_0000,
    PowerOn = 0b0000_0001,
    Reset = 0b0000_0111,
    ContinuouslyHResolutionMode = 0b0001_0000,
    ContinuouslyHResolutionMode2 = 0b0001_0001,
    ContinuouslyLResolutionMode = 0b0001_0011,
    OneTimeHResolutionMode = 0b0010_0000,
    OneTimeHResolutionMode2 = 0b0010_0001,
    OneTimeLResolutionMode = 0b0010_0011,
}

struct LightSensor {
    device: LinuxI2CDevice,
}

impl LightSensor {
    pub fn new(num: u64) -> Result<LightSensor, LinuxI2CError> {
        let path = String::from("/dev/i2c-") + num.to_string().as_str();
        let dev = LinuxI2CDevice::new(path, NUNCHUCK_SLAVE_ADDR)?;
        let light_sensor = LightSensor { device: dev };
        Ok(light_sensor)
    }

    pub fn read_ight(&mut self) -> Result<f64, LinuxI2CError> {
        let value = self
            .device
            .smbus_read_i2c_block_data(InstructionSet::OneTimeHResolutionMode as u8, 32)?;
        let ret = (value[1] + 0xff * value[0]) as f64 / 1.2;
        Ok(ret)
    }
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut light_sensor = LightSensor::new(1).unwrap();
    while running.load(Ordering::SeqCst) {
        let value = light_sensor.read_ight().unwrap();
        println!("Light Level : {} lx", value);
        sleep(Duration::from_millis(500));
    }
}
