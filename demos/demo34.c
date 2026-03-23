// tc24r demo34 -- multi-dimensional array declarations
//
// New feature:
//   - int a[2][3]; (multi-dimensional array declaration)
//   - sizeof(int[N][M]) correct sizing
//
// Expected: r0 = 42, UART output: "D34OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

int main() {
    int ok = 1;

    // multi-dimensional array sizeof
    if (sizeof(int[2][3]) != 18) ok = 0;
    if (sizeof(int[3][4]) != 36) ok = 0;
    if (sizeof(char[2][3]) != 6) ok = 0;

    // multi-dim array declaration allocates correct stack space
    int a[2][3];
    int *p = (int *)a;
    p[0] = 10;
    p[1] = 20;
    p[2] = 30;
    p[3] = 40;
    p[4] = 50;
    p[5] = 60;
    if (p[0] + p[5] != 70) ok = 0;

    if (ok) {
        puts("D34OK\n");
        return 42;
    }
    return 0;
}
