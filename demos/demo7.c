// cc24 demo7 -- pointer subtraction
//
// New features:
//   - ptr - int: retreats pointer by n elements (scaled)
//   - ptr - ptr: yields element count between two pointers
//
// Expected: r0 = 42, UART output: "D7OK"

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

int strlen(char *s) {
    char *p = s;
    while (*p) {
        p = p + 1;
    }
    return p - s;
}

int main() {
    int ok = 1;

    // ptr - int: retreat through int array
    int a[3];
    a[0] = 10;
    a[1] = 20;
    a[2] = 30;
    int *p = a + 2;
    p = p - 1;
    if (*p != 20) { ok = 0; }

    // ptr - ptr on int pointers
    int *start = a;
    int *end = a + 3;
    if (end - start != 3) { ok = 0; }

    // ptr - ptr on char pointers (strlen)
    int len = strlen("Hello");
    if (len != 5) { ok = 0; }

    // char ptr - int
    char *s = "ABCDE";
    char *mid = s + 4;
    mid = mid - 2;
    if (*mid != 67) { ok = 0; }  /* 'C' = 67 */

    if (ok == 1) {
        puts("D7OK\n");
        return 42;
    }
    return 0;
}
