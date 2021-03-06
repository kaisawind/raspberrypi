pub enum PICC {
    REQIDL = 0x26,
    REQALL = 0x52,
    ANTICOLL = 0x93,
    AUTHENT1A = 0x60,
    AUTHENT1B = 0x61,
    READ = 0x30,
    WRITE = 0xA0,
    DECREMENT = 0xC0,
    INCREMENT = 0xC1,
    RESTORE = 0xC2,
    TRANSFER = 0xB0,
    HALT = 0x50,
}
