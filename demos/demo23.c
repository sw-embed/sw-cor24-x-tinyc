// cc24 demo23 -- enum
//
// New features:
//   - enum { A, B, C } -- auto-incrementing constants
//   - enum { X=5, Y, Z } -- explicit values
//   - enum tag { ... } -- named enum types
//   - enum constants resolve to int at parse time
//
// Expected: r0 = 42, UART output: "D23OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

enum color { RED, GREEN, BLUE };

int main() {
    int ok = 1;

    // auto-increment
    if (RED != 0) ok = 0;
    if (GREEN != 1) ok = 0;
    if (BLUE != 2) ok = 0;

    // explicit values
    enum { OFF = 0, ON = 1, AUTO = 10 };
    if (AUTO != 10) ok = 0;

    // in expressions
    int x = GREEN + BLUE;
    if (x != 3) ok = 0;

    // in statement expression
    int y = ({ enum { A=20, B }; B; });
    if (y != 21) ok = 0;

    if (ok) {
        puts("D23OK\n");
        return 42;
    }
    return 0;
}
