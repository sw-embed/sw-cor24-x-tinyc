// cc24 demo19 -- static and extern keywords
//
// New feature:
//   - static/extern consumed as storage-class specifiers
//   - static globals: internal linkage (same as global for single TU)
//   - static functions: accepted (same as normal function)
//   - extern: accepted and ignored (single translation unit)
//
// Expected: r0 = 42, UART output: "D19OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

static int g = 10;

static int add(int a, int b) {
    return a + b;
}

int main() {
    int ok = 1;

    // static global
    if (g != 10) { ok = 0; }

    // static function
    if (add(20, 12) != 32) { ok = 0; }

    // modify static global
    g = 42;
    if (g != 42) { ok = 0; }

    if (ok) {
        puts("D19OK\n");
        return 42;
    }
    return 0;
}
