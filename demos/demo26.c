// tc24r demo26 -- switch/case
//
// New features:
//   - switch (expr) { case V: ... break; default: ... }
//   - fall-through between cases
//   - break exits switch (not enclosing loop)
//
// Expected: r0 = 42, UART output: "D26OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

int classify(int x) {
    switch (x) {
        case 0:
            return 10;
        case 1:
            return 20;
        case 2:
            return 30;
        default:
            return 99;
    }
}

int main() {
    int ok = 1;

    // basic case matching
    if (classify(0) != 10) ok = 0;
    if (classify(1) != 20) ok = 0;
    if (classify(2) != 30) ok = 0;
    if (classify(3) != 99) ok = 0;

    // switch with break (no fall-through)
    int result = 0;
    int val = 2;
    switch (val) {
        case 1:
            result = 11;
            break;
        case 2:
            result = 22;
            break;
        case 3:
            result = 33;
            break;
    }
    if (result != 22) ok = 0;

    // fall-through behavior
    int count = 0;
    switch (1) {
        case 1:
            count = count + 1;
        case 2:
            count = count + 1;
        case 3:
            count = count + 1;
            break;
        default:
            count = 100;
    }
    if (count != 3) ok = 0;

    // switch inside a loop -- break targets switch, not loop
    int sum = 0;
    int i = 0;
    while (i < 4) {
        switch (i) {
            case 0:
                sum = sum + 1;
                break;
            case 1:
                sum = sum + 10;
                break;
            default:
                sum = sum + 100;
                break;
        }
        i = i + 1;
    }
    if (sum != 211) ok = 0;

    if (ok) {
        puts("D26OK\n");
        return 42;
    }
    return 0;
}
