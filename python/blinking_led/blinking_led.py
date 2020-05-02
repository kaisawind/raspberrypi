#!/usr/bin/python
# -*- coding: utf-8 -*-
import time
import RPi.GPIO as GPIO

led_pin = 37

GPIO.setmode(GPIO.BOARD)
GPIO.setup(led_pin, GPIO.OUT)

try:
    while True:
        GPIO.output(led_pin, GPIO.HIGH)
        time.sleep(0.2)
        GPIO.output(led_pin, GPIO.LOW)
        time.sleep(0.2)
except KeyboardInterrupt:
    GPIO.cleanup()