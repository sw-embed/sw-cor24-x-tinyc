// cc24 demo5 -- arrays
//
// New features:
//   - Array declarations: int a[N]
//   - Array indexing: a[i] (read and write)
//   - Array name decays to pointer to first element
//
// Expected: r0 = 42, UART output: "D5OK"

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

int sum_array(int *arr, int len) {
    int total = 0;
    int i = 0;
    while (i < len) {
        total = total + *(arr + i);
        i = i + 1;
    }
    return total;
}

int main() {
    int ok = 1;

    // basic array read/write
    int a[4];
    a[0] = 10;
    a[1] = 20;
    a[2] = 30;
    a[3] = 40;
    if (a[0] + a[3] != 50) { ok = 0; }

    // array passed to function as pointer
    int total = sum_array(a, 4);
    if (total != 100) { ok = 0; }

    // char array
    char buf[3];
    buf[0] = 65;  // 'A'
    buf[1] = 66;  // 'B'
    buf[2] = 0;
    if (buf[0] != 65) { ok = 0; }
    if (buf[1] != 66) { ok = 0; }

    if (ok == 1) {
        puts("D5OK\n");
        return 42;
    }
    return 0;
}
