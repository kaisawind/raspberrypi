pub enum PCD {
    Idle = 0b000_0000,         //无动作;取消当前命令的执行。
    CalcCRC = 0b0000_0011,     //激活 CRC 协处理器或执行自测试。
    Transmit = 0b0000_0100,    // 发送 FIFO 缓冲区的命令。
    NoCmdChange = 0b0000_0111, //无命令改变。该命令用来修改命令寄存器的不同位,但又不触及其它命令,如掉电。
    Receive = 0b0000_1000,     //激活接收器电路。
    // 如果寄存器 ControlReg 的 Initiator 位被设为 1:将 FIFO 缓冲区的数据发送到天线并在发送完成后自动激活接收器。
    // 如果寄存器 ControlReg 的 Initiator 位被设为 0:接收天线的数据并自动激活发送器。
    Transceive = 0b0000_1100,
    MFAuthent = 0b0000_1110, //执行读卡器的 MIFARE 标准认证。
    SoftReset = 0b0000_1111, //复位 MFRC522。
}
