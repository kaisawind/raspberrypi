use crate::mfrc522::address::{Address, RW};

/// 命令
#[derive(Debug, Clone, Copy)]
pub enum Command {
    RFU10 = 0x10,          // 保留为将来之用
    ModeReg = 0x11,        // 定义发送和接收的常用模式
    TxModeReg = 0x12,      // 定义发送过程的数据传输速率
    RxModeReg = 0x13,      // 定义接收过程中的数据传输速率
    TxControlReg = 0x14,   // 控制天线驱动器管脚 TX1 和 TX2 的逻辑特性
    TxAutoReg = 0x15,      // 控制天线驱动器的设置
    TxSeIReg = 0x16,       // 选择天线驱动器的内部源
    RxSeIReg = 0x17,       // 选择内部的接收器设置
    RxThresholdReg = 0x18, // 选择位译码器的阈值
    DemodReg = 0x19,       // 定义解调器的设置
    RFU1A = 0x1A,          // 保留为将来之用
    RFU1B = 0x1B,          // 保留为将来之用
    MifareReg = 0x1C,      // 控制 ISO 14443/MIFARE 模式中 106kbit/s 的通信
    RFU1D = 0x1D,          // 保留为将来之用
    RFU1E = 0x1E,          // 保留为将来之用
    SerialSpeedReq = 0x1F, // 选择串行 UART 接口的速率
}

impl Address for Command {
    fn r_addr(&self) -> u8 {
        ((*self as u8) << 1) | (RW::R as u8)
    }
    fn w_addr(&self) -> u8 {
        ((*self as u8) << 1) | (RW::W as u8)
    }
}
