// cc24 demo11 -- logical AND (&&) and OR (||) with short-circuit
//
// New features:
//   - && operator with short-circuit (0 && x skips x, yields 0)
//   - || operator with short-circuit (1 || x skips x, yields 1)
//   - Correct C precedence: || below &&, && below bitwise |
//
// Expected: r0 = 42, UART output: "D11OK"

#define UART_DATA   0xFF0100
#define UART_STATUS 0xFF0101

void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }

void puts(char *s) {
    while (*s) { putc(*s); s = s + 1; }
}

int main() {
    int ok = 1;

    // && both true
    if (!(1 && 2)) { ok = 0; }

    // && short-circuit: first false
    if (0 && 1) { ok = 0; }

    // || first true
    if (!(1 || 0)) { ok = 0; }

    // || both false
    if (0 || 0) { ok = 0; }

    // || second true
    if (!(0 || 5)) { ok = 0; }

    // mixed: (1 && 0) || 1 = 1
    if (!((1 && 0) || 1)) { ok = 0; }

    if (ok) {
        puts("D11OK\n");
        return 42;
    }
    return 0;
}
