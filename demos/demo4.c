// cc24 demo4 -- software divide and modulo
//
// New features:
//   - Integer division (/) via software __cc24_div
//   - Integer modulo (%) via software __cc24_mod
//
// Expected: r0 = 42, UART output: "D4OK"

void putc(int c) {
    while (*(char *)0xFF0101 & 0x80) {}
    *(char *)0xFF0100 = c;
}

void puts(char *s) {
    while (*s) {
        putc(*s);
        s = s + 1;
    }
}

int main() {
    int ok = 1;

    // basic division
    if (17 / 5 != 3) { ok = 0; }
    if (17 % 5 != 2) { ok = 0; }

    // exact division
    if (100 / 10 != 10) { ok = 0; }
    if (100 % 10 != 0) { ok = 0; }

    // divide by 1
    if (42 / 1 != 42) { ok = 0; }
    if (42 % 1 != 0) { ok = 0; }

    // dividend < divisor
    if (3 / 7 != 0) { ok = 0; }
    if (3 % 7 != 3) { ok = 0; }

    if (ok == 1) {
        puts("D4OK\n");
        return 42;
    }
    return 0;
}
