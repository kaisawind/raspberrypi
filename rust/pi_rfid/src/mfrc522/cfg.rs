use crate::mfrc522::address::{Address, RW};

/// CFG
#[derive(Debug, Clone, Copy)]
pub enum CFG {
    RFU20 = 0x20,             // 保留为将来之用
    CRCResultRegM = 0x21,     // 显示 CRC 计算的实际MSB值
    CRCResultRegL = 0x22,     // 显示 CRC 计算的实际LSB 值
    RFU23 = 0x23,             // 保留为将来之用
    ModWidthReg = 0x24,       // 控制 ModWidth 的设置
    RFU25 = 0x25,             // 保留为将来之用
    RFCfgReg = 0x26,          // 配置接收器增益
    GsNReg = 0x27,            // 选择天线驱动器管脚 TX1 和 TX2 的调制电导
    CWGsCfgReg = 0x28,        // 选择天线驱动器管脚 TX1 和 TX2 的调制电导
    ModGsCfgReg = 0x29,       // 选择天线驱动器管脚 TX1 和 TX2 的调制电导
    TModeReg = 0x2A,          // 定义内部定时器的设置
    TPrescalerReg = 0x2B,     //
    TReloadRegH = 0x2C,       // 描述 16 位长的定时器重装值
    TReloadRegL = 0x2D,       // 描述 16 位长的定时器重装值
    TcounterValueRegH = 0x2E, // 显示 16 位长的实际定时器值
    TcounterValueRegL = 0x2F, // 显示 16 位长的实际定时器值
}

impl Address for CFG {
    fn r_addr(&self) -> u8 {
        ((*self as u8) << 1) | (RW::R as u8)
    }
    fn w_addr(&self) -> u8 {
        ((*self as u8) << 1) | (RW::W as u8)
    }
}
