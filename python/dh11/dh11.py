#!/usr/bin/python3
# -*- coding: utf-8 -*-

import sys
import Adafruit_DHT

# 设置传感器类型
sensor = 11
# 设置管脚
pin = 17

# 尝试获取传感器读数。使用将重试的read_retry方法
# 到15次获得传感器读数(每次重试之间等待2秒).
humidity, temperature = Adafruit_DHT.read_retry(sensor, pin)

# 下面一行的注释，是将温度转换为华氏温度。
# temperature = temperature * 9/5.0 + 32

# 注意，有时您不会得到读数，
# 结果将为null(因为Linux不能保证读取传感器的调用的时间)。
# 如果发生这种情况，再试一次!
if humidity is not None and temperature is not None:
    print('Temp={0:0.1f}*  Humidity={1:0.1f}%'.format(temperature, humidity))
else:
    print('Failed to get reading. Try again!')