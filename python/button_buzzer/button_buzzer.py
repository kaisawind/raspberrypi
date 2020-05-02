#!/usr/bin/python3
# -*- coding: utf-8 -*-
import time
import RPi.GPIO as GPIO

button_pin = 37 # 按键
buzzer_pin = 12 # 蜂鸣器

GPIO.setmode(GPIO.BOARD)

GPIO.setup(button_pin, GPIO.IN, pull_up_down=GPIO.PUD_UP)
GPIO.setup(buzzer_pin, GPIO.OUT)

try:
    while True:
        if (GPIO.input(button_pin)==0):
            GPIO.output(buzzer_pin, GPIO.HIGH)
        else:
            GPIO.output(buzzer_pin, GPIO.LOW)
except KeyboardInterrupt:
    GPIO.cleanup()