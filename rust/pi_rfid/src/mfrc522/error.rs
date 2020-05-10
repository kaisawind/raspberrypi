use rppal::gpio;
use rppal::spi;
use std::error;
use std::fmt;
use std::io;

/// Errors that can occur when accessing the SPI peripheral.
#[derive(Debug)]
pub enum Error {
    /// I/O error.
    Io(io::Error),
    Spi(spi::Error),
    Gpio(gpio::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "I/O error: {}", err),
            Error::Spi(ref err) => write!(f, "spi error: {}", err),
            Error::Gpio(ref err) => write!(f, "gpio error: {}", err),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<spi::Error> for Error {
    fn from(err: spi::Error) -> Error {
        Error::Spi(err)
    }
}

impl From<gpio::Error> for Error {
    fn from(err: gpio::Error) -> Error {
        Error::Gpio(err)
    }
}
