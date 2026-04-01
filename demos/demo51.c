// tc24r demo51 -- function pointer: basic variable call
//
// Assign a function to a pointer variable, call through the pointer.
//
// Expected: r0 = 51, UART output: "D51OK"

#include <stdio.h>

int add(int a, int b) {
    return a + b;
}

int main() {
    int (*fp)(int, int);
    fp = add;

    int result = fp(3, 4);
    if (result != 7) return 1;

    // Verify with a second call
    result = fp(20, 31);
    if (result != 51) return 2;

    printf("D51OK\n");
    return 51;
}
