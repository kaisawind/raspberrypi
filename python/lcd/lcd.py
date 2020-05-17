#!/usr/bin/python3
# -*- coding: utf-8 -*-

# Example using a character LCD backpack.
import time
import Adafruit_CharLCD as LCD

# 定义16x2 LCD的列和行大小。
lcd_columns = 16
lcd_rows    = 2

# 初始化IIC 总线 LCD模块
lcd = LCD.Adafruit_CharLCDBackpack(address=0x21)

try:
    # 打开背光
    lcd.set_backlight(0)

    # 打印两行消息
    lcd.message('Hello\nworld!')

    # 等待5秒
    time.sleep(5.0)

    # 演示光标移动。
    lcd.clear()
    lcd.show_cursor(True)
    lcd.message('Show cursor')

    time.sleep(5.0)

    # 演示光标闪烁。
    lcd.clear()
    lcd.blink(True)
    lcd.message('Blink cursor')

    time.sleep(5.0)

    # 停止闪烁和显示光标。
    lcd.show_cursor(False)
    lcd.blink(False)

    # 演示左右滚动消息。
    lcd.clear()
    message = 'Scroll'
    lcd.message(message)
    for i in range(lcd_columns-len(message)):
        time.sleep(0.5)
        lcd.move_right()
    for i in range(lcd_columns-len(message)):
        time.sleep(0.5)
        lcd.move_left()

    # 演示关闭和打开背光。
    lcd.clear()
    lcd.message('Flash backlight\nin 5 seconds...')
    time.sleep(5.0)
    # 关闭背光。
    lcd.set_backlight(1)
    time.sleep(2.0)
    # 改变消息
    lcd.clear()
    lcd.message('Goodbye!')
    # 打开背光。
    lcd.set_backlight(0)
    # 关闭背光。
    time.sleep(2.0)
    lcd.clear()
    lcd.set_backlight(1)
except KeyboardInterrupt:
    # 关掉屏幕
    lcd.clear()
    lcd.set_backlight(1)