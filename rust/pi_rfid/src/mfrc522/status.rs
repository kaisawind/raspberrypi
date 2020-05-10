use crate::mfrc522::address::{Address, RW};

/// 命令和状态
#[derive(Debug, Clone, Copy)]
pub enum Status {
    RFU00 = 0x00,         // 保留为将来之用
    CommandReg = 0x01,    // 启动和停止命令的执行
    ComIEnReg = 0x02,     // 中断请求传递的使能和禁能控制位。
    DivIEnReg = 0x03,     // 中断请求传递的使能和禁能控制位
    ComIrqReg = 0x04,     // 包含中断请求标志
    DivIrqReg = 0x05,     // 包含中断请求标志
    ErrorReg = 0x06,      // 错误标志,指示执行的上个命令的错误状态
    Status1Reg = 0x07,    // 包含通信的状态标志
    Status2Reg = 0x08,    // 包含接收器和发送器的状态标志
    FIFODataReg = 0x09,   // 64 字节 FIFO 缓冲区的输入和输出
    FIFOLevelReg = 0x0A,  // 指示 FIFO 中存储的字节数
    WaterLevelReg = 0x0B, // 定义 FIFO 下溢和上溢报警的 FIFO 深度
    ControlReg = 0x0C,    // 不同的控制寄存器
    BitFramingReg = 0x0D, // 面向位的帧的调节
    CollReg = 0x0E,       // RF 接口上检测到的第一个位冲突的位的位置
    RFU0F = 0x0F,         // 保留为将来之用
}

impl Address for Status {
    fn r_addr(&self) -> u8 {
        ((*self as u8) << 1) | (RW::R as u8)
    }
    fn w_addr(&self) -> u8 {
        ((*self as u8) << 1) | (RW::W as u8)
    }
}
