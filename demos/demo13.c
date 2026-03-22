// cc24 demo13 -- break and continue
//
// New features:
//   - break: exits innermost loop
//   - continue: jumps to loop increment/condition
//   - Works in while, do...while, and for loops
//
// Expected: r0 = 42, UART output: "D13OK"

#define UART_DATA 0xFF0100
void putc(int c) { *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

int main() {
    int ok = 1;

    // break in while
    int i = 0;
    while (1) {
        if (i == 5) { break; }
        i = i + 1;
    }
    if (i != 5) { ok = 0; }

    // continue in while (sum odd numbers 1-9)
    int sum = 0;
    int j = 0;
    while (j < 10) {
        j = j + 1;
        if (j % 2 == 0) { continue; }
        sum = sum + j;
    }
    if (sum != 25) { ok = 0; }

    // break in for
    int k = 0;
    for (k = 0; k < 100; k = k + 1) {
        if (k == 3) { break; }
    }
    if (k != 3) { ok = 0; }

    // continue in for (skip evens)
    int total = 0;
    for (int n = 1; n <= 5; n = n + 1) {
        if (n % 2 == 0) { continue; }
        total = total + n;
    }
    if (total != 9) { ok = 0; }

    if (ok) {
        puts("D13OK\n");
        return 42;
    }
    return 0;
}
