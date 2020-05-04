extern crate sysfs_gpio;

use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::Error;
use sysfs_gpio::{Direction, Pin};

pub struct DHT11 {
    pin: Pin,
}

impl DHT11 {
    pub fn new(pin_num: u64) -> Result<DHT11, Error> {
        let pin = Pin::new(pin_num);
        pin.export()?;
        Ok(DHT11 { pin })
    }

    /// MCU sends a start signal
    ///
    /// When MCU sends a start signal, DHT11 changes from the low-power-consumption mode to the
    /// running-mode, waiting for MCU completing the start signal. Once it is completed, DHT11 sends a
    /// response signal of 40-bit data that include the relative humidity and temperature information to
    /// MCU. Users can choose to collect (read) some data. Without the start signal from MCU, DHT11
    /// will not give the response signal to MCU. Once data is collected, DHT11 will change to the low-
    /// power-consumption mode until it receives a start signal from MCU again.
    pub fn start(&self) -> Result<(), Error> {
        // sleep 80 ms, exclude some error
        sleep(Duration::from_millis(80));

        // start signal
        //
        // MCU will set Data Single-bus voltage level from high to low
        // and this process must take at least 18ms to ensure DHT’s detection of MCU's signal
        self.pin.set_direction(Direction::High)?; // default value should be high
        sleep(Duration::from_millis(100));
        self.pin.set_value(0)?; // low
        sleep(Duration::from_millis(20));
        // MCU will pull up voltage and wait 20-40us for DHT’s response.
        self.pin.set_value(1)?; // high
        sleep(Duration::from_millis(30));

        // start get response
        self.pin.set_direction(Direction::In)?;
        // dht sends out response signal
        // Once DHT detects the start signal, it will send out a low-voltage-level response signal, which
        // lasts 80us.
        let mut i = 0;
        loop {
            i = i + 1;
            if i > 80 {
                return Err(Error::Unexpected(String::from(
                    "dht low response signal time out",
                )));
            }
            let value = self.pin.get_value()?;
            if value == 0 {
                println!("get dht low signal, takes {} us", i);
                break;
            }
            sleep(Duration::from_micros(1));
        }

        // dht will pull up voltage and get ready for sensor's output
        // DHT sets Data Single-bus voltage level from low to high and
        // keeps it for 80us for DHT’s preparation for sending data.
        let mut i = 0;
        loop {
            i = i + 1;
            if i > 80 {
                return Err(Error::Unexpected(String::from(
                    "dht high response signal time out",
                )));
            }
            let value = self.pin.get_value()?;
            if value == 1 {
                println!("get dht high signal, takes {} us", i);
                break;
            }
        }

        println!("dht start ok...");
        Ok(())
    }

    pub fn read(&self) -> Result<(), Error> {
        let mut buf = [0; 40];
        for k in 0..40 {
            let mut i = 0;
            loop {
                i += 1;
                if i > 50 {
                    return Err(Error::Unexpected(String::from(
                        "dht low response data time out",
                    )));
                }
                let value = self.pin.get_value()?;
                if value == 0 {
                    println!("get dht low data signal, takes {} us", i);
                    break;
                }
                sleep(Duration::from_micros(1));
            }

            let mut i = 0;
            loop {
                i += 1;
                if i > 70 {
                    return Err(Error::Unexpected(String::from(
                        "dht low response data time out",
                    )));
                }
                let value = self.pin.get_value()?;
                if value == 1 {
                    println!("get dht high data signal, takes {} us", i);
                    break;
                }
                sleep(Duration::from_micros(1));
            }
            println!("voltage length takes {} us", i);
            if i < 50 {
                buf[k] = 0;
            } else {
                buf[k] = 1;
            }
        }
        println!("data buf {:?}", &buf[..]);
        Ok(())
    }
}

impl Drop for DHT11 {
    fn drop(&mut self) {
        self.pin.unexport().unwrap();
    }
}
