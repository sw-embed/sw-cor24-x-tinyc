// tc24r demo59 -- struct member array element write/read
//
// Exercises BUG-003 fix: writing and reading elements of char and int
// arrays that are struct members. Previously, writes would use array
// contents as a pointer (crashing after zero-init).
//
// Expected: r0 = 59, UART output: "D59OK"

#include <stdio.h>

struct S { int mode; char cmd[8]; int vals[4]; };

int test_char_array_write(void) {
    struct S e;
    e.mode = 0;
    e.cmd[0] = 'H';
    e.cmd[1] = 'i';
    e.cmd[2] = 0;
    if (e.cmd[0] != 'H') return 1;
    if (e.cmd[1] != 'i') return 2;
    if (e.cmd[2] != 0) return 3;
    return e.cmd[0] + e.cmd[1];
}

int test_int_array_write(void) {
    struct S e;
    e.vals[0] = 10;
    e.vals[1] = 20;
    e.vals[2] = 30;
    e.vals[3] = 40;
    if (e.vals[0] != 10) return 1;
    if (e.vals[3] != 40) return 2;
    return e.vals[0] + e.vals[1] + e.vals[2] + e.vals[3];
}

int test_via_pointer(struct S *p) {
    p->cmd[0] = 'X';
    p->cmd[1] = 'Y';
    p->cmd[2] = 0;
    return p->cmd[0] + p->cmd[1];
}

int main() {
    int r = 0;

    r = test_char_array_write();
    if (r != 'H' + 'i') return 1;

    r = test_int_array_write();
    if (r != 100) return 2;

    {
        struct S e;
        r = test_via_pointer(&e);
        if (r != 'X' + 'Y') return 3;
    }

    printf("D59OK\n");
    return 59;
}
