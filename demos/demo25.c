// cc24 demo25 -- struct support
//
// New features:
//   - struct { type member; ... } definition
//   - struct tag { ... } named struct
//   - struct tag var; variable declaration
//   - var.member read and write
//   - sizeof(struct tag)
//
// Expected: r0 = 42, UART output: "D25OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

struct point {
    int x;
    int y;
};

int main() {
    int ok = 1;

    // anonymous struct
    struct { int a; int b; } s;
    s.a = 10;
    s.b = 20;
    if (s.a + s.b != 30) ok = 0;

    // named struct
    struct point p;
    p.x = 3;
    p.y = 4;
    if (p.x != 3) ok = 0;
    if (p.y != 4) ok = 0;

    // sizeof
    if (sizeof(struct point) != 6) ok = 0;

    // struct in expression
    int sum = p.x + p.y;
    if (sum != 7) ok = 0;

    // compound assignment on member
    p.x += 10;
    if (p.x != 13) ok = 0;

    if (ok) {
        puts("D25OK\n");
        return 42;
    }
    return 0;
}
