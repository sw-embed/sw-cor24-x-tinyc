// cc24 demo8 -- preprocessor #define
//
// New features:
//   - #define NAME value (constant substitution)
//   - Substitution skips string literals
//   - No partial identifier matching (FOO does not match FOOBAR)
//
// Expected: r0 = 42, LED D2 on, UART output: "D8OK"

#define LED_REG      0xFF0000
#define UART_REG     0xFF0100
#define UART_STATUS  0xFF0101
#define LED_ON       0
#define ANSWER    42
#define GREETING  "D8OK\n"

void led_on() {
    *(char *)LED_REG = LED_ON;
}

void putc(int c) {
    while (*(char *)UART_STATUS & 0x80) {}
    *(char *)UART_REG = c;
}

void puts(char *s) {
    while (*s) {
        putc(*s);
        s = s + 1;
    }
}

int main() {
    int ok = 1;

    // #define constants in expressions
    int a = ANSWER;
    if (a != 42) { ok = 0; }

    // #define in MMIO
    led_on();

    // #define does not substitute inside strings
    char *msg = "ANSWER is not replaced here";
    int len = 0;
    char *p = msg;
    while (*p) { len = len + 1; p = p + 1; }
    if (len != 27) { ok = 0; }

    // #define with no partial match
    int ANSWERING = 99;
    if (ANSWERING != 99) { ok = 0; }

    if (ok == 1) {
        puts(GREETING);
        return ANSWER;
    }
    return 0;
}
