use crate::mfrc522::address::{Address, RW};

/// 命令和状态
#[derive(Debug, Clone, Copy)]
pub enum Status {
    RFU00 = 0x00,      // 保留为将来之用
    CommandReg = 0x01, // 启动和停止命令的执行

    ///
    /// |  7 |   6 |   5 |     4 |        3 |        2 |    1 |      0 |
    /// | ---|  ---|  ---|    ---|       ---|       ---|   ---|     ---|
    /// |IRqInv|TxIEn|RxIEn|IdleIRq|HiAlertIRq|LoAlertIRq|ErrIRq|TimerIRq|
    ///
    /// * IRqInv 该位置位时,管脚 IRQ 上的信号与寄存器 Status1Reg 的 IRq 位的值相反。该位清零时,管脚 IRQ 上的信号与 IRq 位的值相同。该位与 DivIEnReg 寄存器的IrqPushPull 位一起使用,如果是默认值 1,则管脚 IRQ 的输出电平为三态。
    /// * TxIRq 允许发送器中断请求(由位 TxIRq 来指示)传递到 IRQ 管脚。
    /// * RxIRq 允许接收器中断请求(由位 RxIRq 来指示)传递到 IRQ 管脚。
    /// * IdleIRq 允许空闲中断请求(由位 IdleIRq 来指示)传递到 IRQ 管脚。
    /// * HiAlertIRq 允许高报警中断请求(由位 HiAlertIRq)传递到 IRQ 管脚。
    /// * LoAlertIRq 允许低报警中断请求(由位 LoAlertIRq)传递到 IRQ 管脚。
    /// * ErrIRq 允许错误中断请求(由位 ErrIRq)传递到 IRQ 管脚。
    /// * TimerIRq 允许定时器中断请求(由位 TimerIRq)传递到 IRQ 管脚。
    ///
    ComIEnReg = 0x02, // 中断请求传递的使能和禁能控制位。

    DivIEnReg = 0x03, // 中断请求传递的使能和禁能控制位

    ///
    /// |  7 |   6 |   5 |     4 |        3 |        2 |    1 |      0 |
    /// | ---|  ---|  ---|    ---|       ---|       ---|   ---|     ---|
    /// |Set1|TxIRq|RxIRq|IdleIRq|HiAlertIRq|LoAlertIRq|ErrIRq|TimerIRq|
    ///
    /// * Set1 该位置位时,Set1 定义 CommIRqReg 寄存器中的屏蔽位置位。该位清零时,Set1 定义 CommIRqReg 寄存器中的屏蔽位清零。
    /// * TxIRq 该位在发送数据的最后一位发送出去后立刻置位。
    /// * RxIRq 当接收器检测到一个有效数据流结束后,该位置位。如果寄存器 RxModeReg的 RxNoErr 位置位,当 FIFO 中有可用的数据字节时,RxIRQ 位只能置位。
    /// * IdleIRq 当一个命令自身终止(例如,当 CommandReg 的值从其它命令变为空闲命令时)该位置位。如果启动一个未知命令,CommandReg 的值变为空闲命令,并且 IdleIRq 置位。由微控制器启动的空闲命令不会置位 IdleIRq。
    /// * HiAlertIRq 该位在 Status1Reg 寄存器的 HiAlert 位置位时置位。与 HiAlert 相反,HiAlertIRq 将保存此中断事件,直到得到 Set1 的清零指示时才能被复位。
    /// * LoAlertIRq 该位在 Status1Reg 寄存器的 LoAlert 位置位时置位。与 LoAlert 相反,LoAlertIRq 将保存此中断事件,直到得到 Set1 的清零指示时才能被复位。
    /// * ErrIRq 只要 Error 寄存器中任何一个错误标志被设置,该位就置位。
    /// * TimerIRq 当定时器的 TimerValue 寄存器的值递减到零时,该位置位。
    ///
    ComIrqReg = 0x04, // 包含中断请求标志

    DivIrqReg = 0x05,     // 包含中断请求标志
    ErrorReg = 0x06,      // 错误标志,指示执行的上个命令的错误状态
    Status1Reg = 0x07,    // 包含通信的状态标志
    Status2Reg = 0x08,    // 包含接收器和发送器的状态标志
    FIFODataReg = 0x09,   // 64 字节 FIFO 缓冲区的输入和输出
    FIFOLevelReg = 0x0A,  // 指示 FIFO 中存储的字节数
    WaterLevelReg = 0x0B, // 定义 FIFO 下溢和上溢报警的 FIFO 深度

    ///
    /// |  7      |   6     |   5   |   4   |  3  |        2 |    1     |      0   |
    /// | ---     |  ---    |  ---  |  ---  | --- |       ---|   ---    |     ---  |
    /// |TstopNow |TstartNow|    0  |   1   |  0  |TxLastBits|TxLastBits|TxLastBits|
    ///
    /// * TstopNow 该位置位时,定时器立刻停止运行。该位读出时返回 0。
    /// * TStartNow 该位置位时,定时器立刻开始运行。该位读出时返回 0。
    /// * RxLastBits 显示最后接收到的字节的有效位的数目。如果该位为 0,则整个字节有效。
    ///
    ControlReg = 0x0C, // 不同的控制寄存器

    ///
    /// |  7      |   6   |   5   |   4   |  3  |        2 |    1     |      0   |
    /// | ---     |  ---  |  ---  |  ---  | --- |       ---|   ---    |     ---  |
    /// |StartSend|RxAlign|RxAlign|RxAlign|  0  |TxLastBits|TxLastBits|TxLastBits|
    ///
    /// * StartSend 该位置位时启动数据的发送。该位只在与收发命令一起使用时才有效。
    /// * RxAlign
    ///     用于面向位的帧的接收:RxAlign 定义第一个接收到的位在 FIFO 中的存放位
    ///     置。接收到的其它位依次存放在后面的位置。
    ///     例如:RxAlign=0:接收到的 LSB 位存放在位 0,接收到的第 2 位存放在位 1。
    ///     RxAlign=1:接收到的 LSB 位存放在位 1,接收到的第 2 位存放在位 2。
    ///     RxAlign=7:接收到的 LSB 位存放在位 7,接收到的第 2 位存放在下个
    ///     字节的位 0 的位置上。
    ///     此标志只在 106kbit/s 的位良好的防冲突机制中有效。其它模式中应设置为 0。
    /// * TxLastBits 用于面向位的帧的发送:TxLastBits 定义发送的最后一个字节的位数。000 表示最后一个字节的所有位都应发送。
    ///
    BitFramingReg = 0x0D, // 面向位的帧的调节

    CollReg = 0x0E, // RF 接口上检测到的第一个位冲突的位的位置
    RFU0F = 0x0F,   // 保留为将来之用
}

impl Address for Status {
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
