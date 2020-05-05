#!/usr/bin/python
# -*- coding: utf-8 -*-

import RPi.GPIO as GPIO
import time

# GPIO 设置编码方式选择
GPIO.setmode(GPIO.BOARD)

# 超声波传感器模块管脚定义
TRIG = 36
ECHO = 32

print("Distance Measurement In Progress")

GPIO.setup(TRIG,GPIO.OUT)
GPIO.setup(ECHO,GPIO.IN)

GPIO.output(TRIG, False)
print("Waiting For Sensor To Settle")
time.sleep(2)

GPIO.output(TRIG, True)
time.sleep(0.00001)
GPIO.output(TRIG, False)
# 等待超声波的回波信号
while GPIO.input(ECHO)==0:
  pulse_start = time.time()

while GPIO.input(ECHO)==1:
  pulse_end = time.time()

pulse_duration = pulse_end - pulse_start
# 计算超声波的距离值
distance = pulse_duration * 17150

distance = round(distance, 2)
# 打印出超声波的距离值
print("Distance:",distance,"cm")
# 清空GPIO
GPIO.cleanup()