// tc24r demo28 -- union support
//
// New features:
//   - union { type member; ... } definition
//   - union tag { ... } named union
//   - Members share same memory (offset 0)
//   - sizeof(union) = max member size
//
// Expected: r0 = 42, UART output: "D28OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

union intchar {
    int i;
    char c;
};

int main() {
    int ok = 1;

    // basic union: members share same memory
    union intchar u;
    u.i = 42;
    if (u.i != 42) ok = 0;

    // writing to one member affects the other (same offset)
    u.i = 65;
    if (u.c != 65) ok = 0;

    // sizeof union = max member size (int=3, char=1, so 3)
    if (sizeof(union intchar) != 3) ok = 0;

    // anonymous union
    union { int x; int y; } v;
    v.x = 100;
    // x and y share same memory
    if (v.y != 100) ok = 0;

    if (ok) {
        puts("D28OK\n");
        return 42;
    }
    return 0;
}
