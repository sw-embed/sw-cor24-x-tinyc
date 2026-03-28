// tc24r demo50 -- large local array with nested function calls
//
// BUG-013 fix: char buf[256] on the stack caused offset overflow
// in lw/sw instructions (8-bit signed immediate range -128..127).
// Nested function calls would corrupt buf[0] because the store
// to buf[0] at offset -256 was truncated to offset 0 (saved fp).
//
// Expected: r0 = 50, UART output: "D50OK"

#include <stdio.h>

void fill(char *buf, int n) {
    int i;
    for (i = 0; i < n; i = i + 1) {
        buf[i] = 65 + (i % 26);
    }
}

int check(char *buf, int n) {
    int i;
    for (i = 0; i < n; i = i + 1) {
        if (buf[i] != 65 + (i % 26)) return 0;
    }
    return 1;
}

int main() {
    char buf[256];
    int sentinel;

    sentinel = 99;
    fill(buf, 256);

    // Nested calls between fill and check -- if the frame is corrupted,
    // sentinel or buf[0] will be wrong after the call returns.
    printf("");

    if (sentinel != 99) return 1;
    if (!check(buf, 256)) return 2;
    if (buf[0] != 65) return 3;
    if (buf[255] != 65 + (255 % 26)) return 4;

    printf("D50OK\n");
    return 50;
}
