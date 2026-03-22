// cc24 demo22 -- braceless control flow bodies
//
// New feature:
//   - if (x) stmt; else stmt;  (without braces)
//   - while (x) stmt;
//   - for (...) stmt;
//
// Expected: r0 = 42, UART output: "D22OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

int main() {
    int ok = 1;

    // braceless if/else
    int x;
    if (1) x = 10; else x = 20;
    if (x != 10) ok = 0;

    // braceless while
    int i = 0;
    while (i < 5) i++;
    if (i != 5) ok = 0;

    // braceless for
    int s = 0;
    for (int j = 0; j < 10; j++) s += j;
    if (s != 45) ok = 0;

    // nested braceless
    int y = 0;
    if (1)
        if (0) y = 1;
        else y = 2;
    if (y != 2) ok = 0;

    if (ok) {
        puts("D22OK\n");
        return 42;
    }
    return 0;
}
