// tc24r demo53 -- function pointer: passed as parameter
//
// Pass a function pointer as an argument to another function.
//
// Expected: r0 = 53, UART output: "D53OK"

#include <stdio.h>

int add(int a, int b) {
    return a + b;
}

int sub(int a, int b) {
    return a - b;
}

int apply(int (*f)(int, int), int x, int y) {
    return f(x, y);
}

int main() {
    int r1 = apply(add, 30, 20);
    if (r1 != 50) return 1;

    int r2 = apply(sub, 30, 20);
    if (r2 != 10) return 2;

    // 50 + 10 = 60, but we want 53
    int r3 = apply(add, r1, 3);
    if (r3 != 53) return 3;

    printf("D53OK\n");
    return 53;
}
