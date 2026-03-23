#pragma once

// tc24r freestanding stdio.h
// Minimal printf mapped to UART output.
// Format: %d (decimal), %c (char), %x (hex), %s (string), %% (literal %).

#define NULL 0
#define EOF (-1)

#define _STDIO_UART_DATA   0xFF0100
#define _STDIO_UART_STATUS 0xFF0101

void _putc_uart(int ch) {
    while (*(char *)_STDIO_UART_STATUS & 0x80) {}
    *(char *)_STDIO_UART_DATA = ch;
}

int putchar(int ch) {
    _putc_uart(ch);
    return ch;
}

int puts(char *s) {
    while (*s) {
        _putc_uart(*s);
        s = s + 1;
    }
    _putc_uart(10);
    return 0;
}

void _print_int(int n) {
    if (n < 0) {
        _putc_uart(45);
        n = 0 - n;
    }
    if (n == 0) {
        _putc_uart(48);
        return;
    }
    char buf[8];
    int i = 0;
    while (n > 0) {
        buf[i] = 48 + n % 10;
        n = n / 10;
        i++;
    }
    while (i > 0) {
        i--;
        _putc_uart(buf[i]);
    }
}

void _print_hex(int n) {
    if (n == 0) {
        _putc_uart(48);
        return;
    }
    char hex[6];
    int i = 0;
    while (n > 0) {
        int d = n & 15;
        if (d < 10) {
            hex[i] = 48 + d;
        } else {
            hex[i] = 87 + d;
        }
        n = n >> 4;
        i++;
    }
    while (i > 0) {
        i--;
        _putc_uart(hex[i]);
    }
}

void _print_str(char *s) {
    while (*s) {
        _putc_uart(*s);
        s = s + 1;
    }
}

// Format one arg based on specifier char
void _fmt_one(int spec, int val) {
    if (spec == 100) { _print_int(val); }
    else if (spec == 120) { _print_hex(val); }
    else if (spec == 99) { _putc_uart(val); }
    else if (spec == 115) { _print_str((char *)val); }
    else if (spec == 37) { _putc_uart(37); }
    else { _putc_uart(37); _putc_uart(spec); }
}

int __tc24r_printf0(char *fmt) {
    while (*fmt) {
        if (*fmt == 37) {
            fmt = fmt + 1;
            if (*fmt == 37) { _putc_uart(37); }
            else { _putc_uart(37); _putc_uart(*fmt); }
        } else {
            _putc_uart(*fmt);
        }
        fmt = fmt + 1;
    }
    return 0;
}

int __tc24r_printf1(char *fmt, int a0) {
    int ai = 0;
    while (*fmt) {
        if (*fmt == 37) {
            fmt = fmt + 1;
            if (*fmt == 37) { _putc_uart(37); }
            else if (ai == 0) { _fmt_one(*fmt, a0); ai++; }
            else { _putc_uart(37); _putc_uart(*fmt); }
        } else {
            _putc_uart(*fmt);
        }
        fmt = fmt + 1;
    }
    return 0;
}

int __tc24r_printf2(char *fmt, int a0, int a1) {
    int ai = 0;
    while (*fmt) {
        if (*fmt == 37) {
            fmt = fmt + 1;
            if (*fmt == 37) { _putc_uart(37); }
            else if (ai == 0) { _fmt_one(*fmt, a0); ai++; }
            else if (ai == 1) { _fmt_one(*fmt, a1); ai++; }
            else { _putc_uart(37); _putc_uart(*fmt); }
        } else {
            _putc_uart(*fmt);
        }
        fmt = fmt + 1;
    }
    return 0;
}

int __tc24r_printf3(char *fmt, int a0, int a1, int a2) {
    int ai = 0;
    while (*fmt) {
        if (*fmt == 37) {
            fmt = fmt + 1;
            if (*fmt == 37) { _putc_uart(37); }
            else if (ai == 0) { _fmt_one(*fmt, a0); ai++; }
            else if (ai == 1) { _fmt_one(*fmt, a1); ai++; }
            else if (ai == 2) { _fmt_one(*fmt, a2); ai++; }
            else { _putc_uart(37); _putc_uart(*fmt); }
        } else {
            _putc_uart(*fmt);
        }
        fmt = fmt + 1;
    }
    return 0;
}

// Varargs prototype (actual dispatch via codegen rewrite)
int printf(char *fmt, ...);
