#!/usr/bin/python3
# -*- coding: utf-8 -*-
import time
import RPi.GPIO as GPIO

# UX3-6 ON
sound_pin = 18 # 声音传感器

GPIO.setmode(GPIO.BOARD)
GPIO.setup(sound_pin, GPIO.IN, pull_up_down=GPIO.PUD_UP)

try:
    while True:
        if(GPIO.input(sound_pin) == GPIO.LOW):
            print("Sound Detected")
            time.sleep(0.1)
except KeyboardInterrupt:
    GPIO.cleanup()