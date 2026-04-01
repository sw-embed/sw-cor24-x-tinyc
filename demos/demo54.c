// tc24r demo54 -- global function pointer declaration
//
// Declare function pointers at global scope (not just local).
//
// Expected: r0 = 54, UART output: "D54OK"

#include <stdio.h>

int add(int a, int b) {
    return a + b;
}

int mul(int a, int b) {
    return a * b;
}

int (*op)(int, int);

int main() {
    // Assign and call through global function pointer
    op = add;
    int r1 = op(20, 30);
    if (r1 != 50) return 1;

    // Reassign to a different function
    op = mul;
    int r2 = op(6, 9);
    if (r2 != 54) return 2;

    printf("D54OK\n");
    return 54;
}
