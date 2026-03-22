#pragma once

#define LED_REG     0xFF0000
#define UART_DATA   0xFF0100
#define INT_ENABLE  0xFF0010
#define LED_ON      0

void putc(int c) {
    *(char *)UART_DATA = c;
}

void puts(char *s) {
    while (*s) {
        putc(*s);
        s = s + 1;
    }
}

void led_on() {
    *(char *)LED_REG = LED_ON;
}
