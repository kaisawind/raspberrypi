extern crate i2cdev;
extern crate sysfs_gpio;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError, LinuxI2CMessage};

struct LightSensor;

impl LightSensor {
    pub fn new(num: u64) -> Result<LightSensor, LinuxI2CError> {
        let path = "/dev/i2c-" + num.to_string();
        let mut dev = LinuxI2CDevice::new();
        let light_sensor = LightSensor {

        };
        Ok(light_sensor)
    }
}

fn main() {
    println!("Hello, world!");
}
