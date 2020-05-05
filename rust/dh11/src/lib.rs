extern crate rppal;

use rppal::gpio;
use rppal::gpio::{Gpio, IoPin, Level};
use std::error;
use std::fmt;
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum Error {
    /// Unknown model.
    ///
    /// The Raspberry Pi model or SoC can't be identified. Support for
    /// new models is usually added shortly after they are officially
    /// announced and available to the public. Make sure you're using
    /// the latest release of RPPAL.
    ///
    /// You may also encounter this error if your Linux distribution
    /// doesn't provide any of the common user-accessible system files
    /// that are used to identify the model and SoC.
    UnknownModel,
    /// Pin is not available.
    ///
    /// The pin is already in use elsewhere in your application, or the GPIO peripheral
    /// doesn't expose a pin with the specified number. If the pin is currently in use, you
    /// can retrieve it again after the [`Pin`] (or a derived [`InputPin`], [`OutputPin`] or
    /// [`IoPin`]) instance goes out of scope.
    ///
    /// [`Pin`]: struct.Pin.html
    /// [`InputPin`]: struct.InputPin.html
    /// [`OutputPin`]: struct.OutputPin.html
    /// [`IoPin`]: struct.IoPin.html
    PinNotAvailable(u8),
    /// Permission denied when opening `/dev/gpiomem`, `/dev/mem` or `/dev/gpiochipN` for
    /// read/write access.
    ///
    /// More information on possible causes for this error can be found [here].
    ///
    /// [here]: index.html#permission-denied
    PermissionDenied(String),
    /// I/O error.
    Io(io::Error),
    /// Thread panicked.
    ThreadPanic,
    TimeOut(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::UnknownModel => write!(f, "Unknown Raspberry Pi model"),
            Error::PinNotAvailable(pin) => write!(f, "Pin {} is not available", pin),
            Error::PermissionDenied(ref path) => write!(f, "Permission denied: {}", path),
            Error::Io(ref err) => write!(f, "I/O error: {}", err),
            Error::ThreadPanic => write!(f, "Thread panicked"),
            Error::TimeOut(ref value) => write!(f, "Time Out: {}", value),
        }
    }
}

impl error::Error for Error {}

impl From<gpio::Error> for Error {
    fn from(_err: gpio::Error) -> Error {
        match _err {
            gpio::Error::UnknownModel => Error::UnknownModel,
            gpio::Error::Io(err) => Error::Io(err),
            gpio::Error::PinNotAvailable(pin) => Error::PinNotAvailable(pin),
            gpio::Error::PermissionDenied(path) => Error::PermissionDenied(path),
            gpio::Error::ThreadPanic => Error::ThreadPanic,
        }
    }
}

pub struct DHT11 {
    gpio: Gpio,
    pin: u8,
}

impl DHT11 {
    pub fn new(pin_num: u8) -> Result<DHT11, Error> {
        let gpio = Gpio::new()?;
        Ok(DHT11 { gpio, pin: pin_num })
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
        // start signal
        //
        // MCU will set Data Single-bus voltage level from high to low
        // and this process must take at least 18ms to ensure DHT’s detection of MCU's signal
        let mut pin = self.gpio.get(self.pin)?.into_io(gpio::Mode::Output); // default value should be high
        pin.set_high(); // high
        sleep(Duration::from_millis(100));
        pin.set_low(); // low
        sleep(Duration::from_millis(20));
        // MCU will pull up voltage and wait 20-40us for DHT’s response.
        pin.set_high(); // high
        
        // start get response
        pin.set_mode(gpio::Mode::Input);
        pin.set_pullupdown(gpio::PullUpDown::PullUp);
        sleep(Duration::from_micros(10));

        println!("dht start ok...");

        let mut buf = [0; 40];
        for k in 0..40 {
            self.wait(&pin, Level::High, 200)?;

            let i = match self.wait(&pin, Level::Low, 200){
                Ok(value) => value,
                Err(_) => break,
            };
            // println!("voltage length {} takes {} us", k, i);
            if i < 50 {
                buf[k] = 0;
            } else {
                buf[k] = 1;
            }
        }
        println!("data buf {:?}", &buf[..]);
        
        DHT11::calc(buf);
        
        Ok(())
    }

    fn calc(buf: [u8; 40]) {
        println!("data buf {:?}", &buf[..]);
        let mut data: [u8; 4] = [0; 4];
        for k in 0..4 {
            for v in 0..8 {
                data[k] <<= 1;
                data[k] |= buf[k * 8 + v];
            }
        }
        println!("data: {:?}", &data[..]);
        let f = data[2] as f32 + (data[3] & 0x0f) as f32 * 0.1;
        println!("temperature: {}", f);
        let f = data[0] as f32 + data[1] as f32 * 0.1;
        println!("humidity: {}", f);
    }

    fn wait(&self, input: &IoPin, value: Level, us: u32) -> Result<u128, Error> {
        let mut now = Instant::now();
        let old = now.clone();
        let target = now + Duration::new(0, us * 1000);
        while input.read() != value {
            if now > target {
                let ret = (now - old).as_micros();
                return Err(Error::TimeOut(format!("{} takes {} us", value, ret)));
            }
            now = Instant::now();
        }
        let ret = (now - old).as_micros();
        // println!("wait {} takes {} us", value, ret);
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calc() {
        let buf = [
            0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0,
        ];
        DHT11::calc(buf);
    }
}
