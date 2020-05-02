#!/usr/bin/python
# -*- coding: utf-8 -*-
import time
import RPi.GPIO as GPIO

buzzer_pin = 12

GPIO.setmode(GPIO.BOARD)
GPIO.setup(buzzer_pin, GPIO.OUT)

GPIO.output(buzzer_pin, GPIO.HIGH)

time.sleep(0.5)

GPIO.output(buzzer_pin, GPIO.LOW)

GPIO.cleanup()