#![allow(dead_code)]
use crate::mfrc522::address::Address;
use mfrc522::cfg::CFG;
use mfrc522::command::Command;
use mfrc522::error::Error;
use mfrc522::pcd::PCD;
use mfrc522::status::Status;
use rppal::gpio;
use rppal::gpio::Gpio;
use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};

mod mfrc522;

#[derive(Debug)]
pub struct Mfrc522 {
    spi: Spi,
    gpio: Gpio,
    pin_num: u8,
}

impl Mfrc522 {
    pub fn new() -> Result<Self, Error> {
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1000000, Mode::Mode0)?;
        // let spi = Spi::new(bus, slave_select, clock_speed, mode)?;
        let gpio = Gpio::new()?;
        let mut mfrc522 = Mfrc522 {
            spi,
            gpio,
            pin_num: 22,
        };
        mfrc522.gpio.get(22)?.into_output().set_high();
        mfrc522.init()?;
        Ok(mfrc522)
    }

    pub fn write(&mut self, address: u8, value: u8) -> Result<(), Error> {
        self.spi.write(&[address, value])?;
        Ok(())
    }

    pub fn read(&mut self, address: u8) -> Result<(), Error> {
        let mut buffer = [address, 0];
        let size = self.spi.read(&mut buffer[..])?;
        println!("read size {}", size);
        Ok(())
    }

    fn init(&mut self) -> Result<(), Error> {
        self.write(Status::CommandReg.w_addr(), PCD::SoftReset as u8)?;

        self.write(CFG::TModeReg.w_addr(), 0x8D)?;
        self.write(CFG::TPrescalerReg.w_addr(), 0x3E)?;
        self.write(CFG::TReloadRegL.w_addr(), 30)?;
        self.write(CFG::TReloadRegH.w_addr(), 0)?;

        self.write(Command::TxAutoReg.w_addr(), 0x40)?;
        self.write(Command::ModeReg.w_addr(), 0x3D)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
