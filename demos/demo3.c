// cc24 demo3 -- hex literals, pointer arithmetic, string constants
//
// New features since demo2:
//   - Hex integer literals (0xFF, 0xFF0100)
//   - Pointer arithmetic (char *p; p + 1 advances 1 byte)
//   - String constants ("Hello" becomes label in .data)
//   - String iteration via pointer + dereference
//
// Expected result:
//   r0 = 42
//   UART output: "D3OK"

void putc(int c) {
    *(char *)0xFF0100 = c;
}

void puts(char *s) {
    while (*s) {
        putc(*s);
        s = s + 1;
    }
}

int strlen(char *s) {
    int n = 0;
    while (*s) {
        n = n + 1;
        s = s + 1;
    }
    return n;
}

int main() {
    int ok = 1;

    // --- hex literals ---
    int a = 0xFF;
    if (a != 255) { ok = 0; }
    int b = 0x2A;
    if (b != 42) { ok = 0; }

    // --- string constant + strlen ---
    char *msg = "Hello";
    int len = strlen(msg);
    if (len != 5) { ok = 0; }

    // --- pointer arithmetic: char * advances by 1 ---
    char *p = msg;
    p = p + 1;
    if (*p != 101) { ok = 0; }  /* 'e' = 101 */

    // --- pointer arithmetic: second char of string ---
    char *q = msg + 4;
    if (*q != 111) { ok = 0; }  /* 'o' = 111 */

    // --- UART output ---
    if (ok == 1) {
        puts("D3OK\n");
    }

    if (ok == 1) {
        return 0x2A;
    }
    return 0;
}
