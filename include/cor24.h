#pragma once

// COR24-TB hardware register addresses
#define LED_REG     0xFF0000
#define UART_DATA   0xFF0100
#define UART_STATUS 0xFF0101
#define INT_ENABLE  0xFF0010

// LED D2 (active-low: 0 = on, 1 = off)
#define LED_ON  0
#define LED_OFF 1
