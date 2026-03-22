// cc24 demo6 -- global char, global pointer, global array-like patterns
//
// New features / fixes:
//   - Global char variables use .byte (not .word)
//   - Global char load/store uses lbu/sb
//   - Global pointer declarations (int *ptr)
//   - Global pointer lookahead fix (int *ptr vs int func())
//
// Expected: r0 = 42, UART output: "D6OK"

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

char flag = 0;
int counter = 0;
int *cptr;

int main() {
    int ok = 1;

    // global char: read initial value
    if (flag != 0) { ok = 0; }

    // global char: write and read back
    flag = 1;
    if (flag != 1) { ok = 0; }

    // global int: write and read back
    counter = 100;
    if (counter != 100) { ok = 0; }

    // global pointer: point to global int
    cptr = &counter;
    *cptr = 200;
    if (counter != 200) { ok = 0; }

    // global pointer: read through pointer
    if (*cptr != 200) { ok = 0; }

    if (ok == 1) {
        puts("D6OK\n");
        return 42;
    }
    return 0;
}
