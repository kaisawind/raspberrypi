/// 地址字节按下面的格式传输。第一个字节的 MSB 位设置使用的模式。MSB 位为 1 时
/// 从 MFRC522 读出数据; MSB 位为 0 时将数据写入 MFRC522。第一个字节的位 6-1 定义地
/// 址,最后一位应当设置为 0。
/// |地址(MOSI)| 位 7,MSB|位 6-位 1| 位 0|
/// |---|---|---|---|
/// |字节 0|1:读<br>0:写|地址|RFU(0)|
pub enum RW {
    R = 0b1000_0000,
    W = 0b0000_0000,
}

pub trait Address {
    fn r_addr(&self) -> u8;
    fn w_addr(&self) -> u8;
    fn as_u8(self) -> u8;
}
