// cc24 demo12 -- do...while loop
//
// New feature:
//   - do { body } while (cond); -- executes body at least once
//
// Expected: r0 = 42, UART output: "D12OK"

#define UART_DATA 0xFF0100

void putc(int c) { *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

int main() {
    int ok = 1;

    // do...while executes at least once even if cond is false
    int ran = 0;
    do {
        ran = 1;
    } while (0);
    if (!ran) { ok = 0; }

    // do...while loop counting
    int i = 0;
    do {
        i = i + 1;
    } while (i < 5);
    if (i != 5) { ok = 0; }

    // compared with while (would not execute)
    int j = 0;
    while (j > 100) {
        j = 999;
    }
    if (j != 0) { ok = 0; }

    if (ok) {
        puts("D12OK\n");
        return 42;
    }
    return 0;
}
