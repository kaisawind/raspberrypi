#!/usr/bin/python
# -*- coding: utf-8 -*-
import RPi.GPIO as GPIO
import time

# 定义 motion 传感器的管脚
motion_pin = 16

# 设置 GPIO 为 GPIO.BOARD
GPIO.setmode(GPIO.BOARD)
# 设置管脚模式为输入模式
GPIO.setup(motion_pin, GPIO.IN)

try:
    while True:
       if(GPIO.input(motion_pin) == 0):
             print("Nothing moves ...")  # 没有检测到移动的物体；
       elif(GPIO.input(motion_pin) == 1):
             print("Motion detected!")  # 有检测到移动的物体；
       time.sleep(1)
except KeyboardInterrupt:
    # 检测到CTRL+C，清除并退出脚本
    GPIO.cleanup()