#![allow(dead_code)]
use crate::mfrc522::address::Address;
use mfrc522::cfg::CFG;
use mfrc522::command::Command;
use mfrc522::error::Error;
use mfrc522::pcd::PCD;
use mfrc522::status::Status;
use rppal::gpio::Gpio;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi, Segment};

mod mfrc522;

const MAX_LEN: usize = 16;

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

    pub fn write<T: Address>(&mut self, address: T, value: u8) -> Result<(), Error> {
        let size = self.spi.write(&[address.w_addr(), value])?;
        println!("write address 0x{:02x} value 0x{:02x} size {}", address.as_u8(), value, size);
        Ok(())
    }

    pub fn read<T: Address>(&mut self, address: T) -> Result<u8, Error> {
        let mut buffer = [0u8; 1];
        self.spi.transfer_segments(&[
            Segment::with_write(&[address.r_addr()]),
            Segment::with_read(&mut buffer),
        ])?;
        println!("read buffer {:?}", buffer);
        Ok(buffer[0])
    }

    pub fn request(&mut self, mode:u8) -> Result<(), Error> {
        // 用于面向位的帧的发送:TxLastBits 定义发送的最后一个字节的位数。000 表示最后一个字节的所有位都应发送。
        self.write(Status::BitFramingReg, 0x07)?;
        let (data, len) = self.transceive(&[mode])?;
        if len != 0x10 {
            return Err(Error::Transceive)
        }
        println!("request data {:?}, len {}", data, len);
        Ok(())
    }

    fn clear_bit_mask<T: Address + Clone + Copy>(&mut self, address: T, value: u8)  -> Result<(), Error> {
        let temp = self.read(address)?;
        self.write(address, temp & !value)?;
        Ok(())
    }

    fn set_bit_mask<T: Address + Clone + Copy>(&mut self, address: T, value: u8)  -> Result<(), Error> {
        let temp = self.read(address)?;
        self.write(address, temp | value)?;
        Ok(())
    }

    fn transceive(&mut self, data: &[u8]) -> Result<([u8; MAX_LEN], u8), Error> {
        self.write(Status::ComIEnReg, 0b1111_0111)?;
        self.clear_bit_mask(Status::ComIrqReg, 0b1000_0000)?;
        self.set_bit_mask(Status::FIFOLevelReg, 0b1000_0000)?;

        self.write(Status::CommandReg, PCD::Idle as u8)?;  // 初期化指令

        for v in data {
            self.write(Status::FIFODataReg, *v)?;
        }

        self.write(Status::CommandReg, PCD::Transceive as u8)?;  // 发送指令
        self.set_bit_mask(Status::BitFramingReg, 0b1000_0000)?;  // 启动发送

        let mut i = 2000;
        let (count, irq) = loop {
            let n = self.read(Status::ComIrqReg)?;
            i -= 1;
            if (i == 0) || (n & 0x01 == 0x01) || (n & 0x30 == 0x30) {
                break (i, n)
            }
        };

        self.clear_bit_mask(Status::BitFramingReg, 0b1000_0000)?; // 发送完了

        if count == 0 {
            return Err(Error::Transceive);
        }

        if self.read(Status::ErrorReg)? != 0x00 {
            return Err(Error::Transceive);
        }

        if irq & 0b0111_0111 & 0x01 == 0x01 {
            return Err(Error::NotAgree);
        }

        let mut length;
        let count = self.read(Status::FIFOLevelReg)?;
        let last_bits = self.read(Status::ControlReg)?;
        if last_bits != 0 {
            length = (count - 1) * 8 + last_bits;
        } else {
            length = count * 8;
        }

        if count == 0 {
            length = 1;
        }

        if count > MAX_LEN as u8 {
            length = MAX_LEN as u8;
        }

        let mut data = [0;MAX_LEN];
        for i in 0..count as usize {
            let value = self.read(Status::FIFODataReg)?;
            data[i] = value;
        }
        println!("data {:?} length {}", data, length);
        Ok((data, length))
    }

    fn init(&mut self) -> Result<(), Error> {
        self.write(Status::CommandReg, PCD::SoftReset as u8)?;

        self.write(CFG::TModeReg, 0x8D)?;
        self.write(CFG::TPrescalerReg, 0x3E)?;
        self.write(CFG::TReloadRegL, 30)?;
        self.write(CFG::TReloadRegH, 0)?;

        self.write(Command::TxAutoReg, 0x40)?;
        self.write(Command::ModeReg, 0x3D)?;

        self.antenna_on()?;
        Ok(())
    }

    fn antenna_on(&mut self) -> Result<(), Error> {
        let reg = self.read(Command::TxControlReg)?;
        println!("read TxControlReg {}", reg);
        // 如果寄存器没有激活，则需要激活
        if (reg & PCD::CalcCRC as u8) != PCD::CalcCRC as u8 {
            self.write(Command::TxControlReg, reg | PCD::CalcCRC as u8)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("hello world");
        assert_eq!(2 + 2, 4);
    }
}
