#!/usr/bin/python3
# -*- coding: utf-8 -*-

import RPi.GPIO as GPIO
import smbus
import time

print("GPIO.RPI_REVISION", GPIO.RPI_REVISION)
 # 为IIC总线找到正确的设备
if(GPIO.RPI_REVISION == 1):
    bus = smbus.SMBus(0)
else:
    bus = smbus.SMBus(1)

class LightSensor():

    def __init__(self):

        # 根据数据表定义一些常量

        self.DEVICE = 0x5c # 默认设备I2C地址

        self.POWER_DOWN = 0x00 # No active state
        self.POWER_ON = 0x01 # Power on
        self.RESET = 0x07 # Reset data register value

        # 以4lx分辨率开始测量。时间通常16ms。
        self.CONTINUOUS_LOW_RES_MODE = 0x13
        # 从1lx分辨率开始测量，通常时间120ms。
        self.CONTINUOUS_HIGH_RES_MODE_1 = 0x10
        # 以0.5lx分辨率开始测量，通常时间120毫秒。
        self.CONTINUOUS_HIGH_RES_MODE_2 = 0x11
        # 从1lx分辨率开始测量，通常时间120毫秒。
        # 测量结束后，自动将设备设置为关机状态。
        self.ONE_TIME_HIGH_RES_MODE_1 = 0x20
        # 开始测量0.5lx分辨率，通常时间120毫秒。
        # 测量结束后，自动将设备设置为关机状态。
        self.ONE_TIME_HIGH_RES_MODE_2 = 0x21
        # 从1lx分辨率开始测量，通常时间120毫秒。
        # 测量结束后，自动将设备设置为关机状态。
        self.ONE_TIME_LOW_RES_MODE = 0x23

    def convertToNumber(self, data):

        # 简单的函数转换2字节的数据
        # 变成一个十进制数
        return ((data[1] + (256 * data[0])) / 1.2)

    def readLight(self):

        data = bus.read_i2c_block_data(self.DEVICE,self.ONE_TIME_HIGH_RES_MODE_1)
        return self.convertToNumber(data)

def main():

    sensor = LightSensor()
    try:
        while True:
            print("Light Level : " + str(sensor.readLight()) + " lx")
            time.sleep(0.5)
    except KeyboardInterrupt:
        pass

if __name__ == "__main__":
    main()