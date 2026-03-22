// cc24 demo14 -- increment and decrement operators
//
// New features:
//   - Prefix: ++i, --i (returns new value)
//   - Postfix: i++, i-- (returns old value)
//
// Expected: r0 = 42, UART output: "D14OK"

#define UART_DATA 0xFF0100
void putc(int c) { *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

int main() {
    int ok = 1;

    // basic increment
    int a = 5;
    a++;
    if (a != 6) { ok = 0; }

    // basic decrement
    a--;
    if (a != 5) { ok = 0; }

    // postfix returns old value
    int b = a++;
    if (b != 5 || a != 6) { ok = 0; }

    // prefix returns new value
    int c = ++a;
    if (c != 7 || a != 7) { ok = 0; }

    // in for loop
    int sum = 0;
    for (int i = 0; i < 5; i++) {
        sum = sum + i;
    }
    if (sum != 10) { ok = 0; }

    if (ok) {
        puts("D14OK\n");
        return 42;
    }
    return 0;
}
