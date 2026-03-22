// cc24 demo21 -- compound assignment operators
//
// New features:
//   +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=
//   Desugared at parse time: x += e becomes x = x + e
//
// Expected: r0 = 42, UART output: "D21OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

int main() {
    int ok = 1;

    int a = 10;
    a += 5;
    if (a != 15) { ok = 0; }

    a -= 3;
    if (a != 12) { ok = 0; }

    a *= 2;
    if (a != 24) { ok = 0; }

    a /= 4;
    if (a != 6) { ok = 0; }

    a %= 4;
    if (a != 2) { ok = 0; }

    int b = 0xFF;
    b &= 0x0F;
    if (b != 15) { ok = 0; }

    b |= 0x30;
    if (b != 63) { ok = 0; }

    b ^= 0xFF;
    if (b != 192) { ok = 0; }

    int c = 1;
    c <<= 3;
    if (c != 8) { ok = 0; }

    c >>= 1;
    if (c != 4) { ok = 0; }

    if (ok) {
        puts("D21OK\n");
        return 42;
    }
    return 0;
}
