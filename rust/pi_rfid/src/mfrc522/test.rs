use crate::mfrc522::address::{Address, RW};

/// 测试
#[derive(Debug, Clone, Copy)]
pub enum Test {
    RFU30 = 0x30,           // 保留为将来之用
    TestSel1Reg = 0x31,     // 常用测试信号的配置
    TestSel2Reg = 0x32,     // 常用测试信号的配置和 PRBS 控制
    TestPinEnReg = 0x33,    // D1-D7 输出驱动器的使能管脚(注:仅用于串行接口)
    TestPinValueReg = 0x34, // 定义 D1-D7 用作 I/O 总线时的值
    TestBusReg = 0x35,      // 显示内部测试总线的状态
    AutoTestReg = 0x36,     // 控制数字自测试
    VersionReg = 0x37,      // 显示版本
    AnalogTestReg = 0x38,   // 控制管脚 AUX1 和 AUX2
    TestDAC1Reg = 0x39,     // 定义 TestDAC1 的测试值
    TestDAC2Reg = 0x3A,     // 定义 TestDAC2 的测试值
    TestADCReg = 0x3B,      // 显示 ADC I 和 Q 通道的实际值
    RFU3C = 0x3C,           // 保留为将来之用
    RFU3D = 0x3D,           // 保留为将来之用
    RFU3E = 0x3E,           // 保留为将来之用
    RFU3F = 0x3F,           // 保留为将来之用
}

impl Address for Test {
    fn r_addr(&self) -> u8 {
        ((*self as u8) << 1) | (RW::R as u8)
    }
    fn w_addr(&self) -> u8 {
        ((*self as u8) << 1) | (RW::W as u8)
    }
    fn as_u8(self) -> u8 {
        self as u8
    }
}
