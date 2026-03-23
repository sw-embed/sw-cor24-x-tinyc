// tc24r demo33 -- comma-separated struct/union members
//
// New feature:
//   - struct { int a, b; } (multiple members of same type)
//   - union { int x, y; } (same for unions)
//
// Expected: r0 = 42, UART output: "D33OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

struct point { int x, y; };

int main() {
    int ok = 1;

    // comma-separated struct members
    struct point p;
    p.x = 3;
    p.y = 4;
    if (p.x + p.y != 7) ok = 0;

    // sizeof accounts for both members
    if (sizeof(struct point) != 6) ok = 0;

    // union with comma members (all share offset 0)
    union { int a, b; } u;
    u.a = 99;
    if (u.b != 99) ok = 0;

    // struct with mixed types and commas
    struct { int a, b; char c; } s;
    s.a = 10;
    s.b = 20;
    s.c = 30;
    if (s.a + s.b + s.c != 60) ok = 0;

    if (ok) {
        puts("D33OK\n");
        return 42;
    }
    return 0;
}
