#!/usr/bin/python3
# -*- coding: utf-8 -*-
import time
import RPi.GPIO as GPIO

relay_pin = 40 # 继电器

GPIO.setmode(GPIO.BOARD)
GPIO.setup(relay_pin, GPIO.OUT)

GPIO.output(relay_pin, GPIO.LOW)

time.sleep(0.5)

GPIO.output(relay_pin, GPIO.HIGH)

GPIO.cleanup()