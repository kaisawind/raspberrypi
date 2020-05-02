#!/usr/bin/python
# -*- coding: utf-8 -*-
import time
import RPi.GPIO as GPIO

vibration_pin = 13 # 震动小马达

GPIO.setmode(GPIO.BOARD)
GPIO.setup(vibration_pin, GPIO.OUT)

GPIO.output(vibration_pin, GPIO.HIGH)

time.sleep(20)

GPIO.output(vibration_pin, GPIO.LOW)

GPIO.cleanup()