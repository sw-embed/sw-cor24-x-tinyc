// tc24r demo31 -- tentative definitions
//
// New feature:
//   - int x; int x = 5; (duplicate global, keeps initializer)
//   - int y = 7; int y; (keeps existing initializer)
//
// Expected: r0 = 42, UART output: "D31OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

int x;
int x = 5;
int y = 7;
int y;

int main() {
    int ok = 1;

    // tentative then initialized: keeps 5
    if (x != 5) ok = 0;

    // initialized then tentative: keeps 7
    if (y != 7) ok = 0;

    if (ok) {
        puts("D31OK\n");
        return 42;
    }
    return 0;
}
