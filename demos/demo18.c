// cc24 demo18 -- sizeof operator
//
// New feature:
//   - sizeof(type): compile-time constant
//   - sizeof(int) = 3 (24-bit), sizeof(char) = 1, sizeof(void *) = 3
//
// Expected: r0 = 42, UART output: "D18OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

int main() {
    int ok = 1;

    if (sizeof(int) != 3) { ok = 0; }
    if (sizeof(char) != 1) { ok = 0; }
    if (sizeof(void *) != 3) { ok = 0; }
    if (sizeof(int *) != 3) { ok = 0; }
    if (sizeof(char *) != 3) { ok = 0; }

    // sizeof in expressions
    int arr_bytes = 4 * sizeof(int);
    if (arr_bytes != 12) { ok = 0; }

    if (ok) {
        puts("D18OK\n");
        return 42;
    }
    return 0;
}
