// tc24r demo52 -- function pointer: array dispatch table
//
// Store functions in an array, call through indexed array element.
//
// Expected: r0 = 52, UART output: "D52OK"

#include <stdio.h>

int double_it(int x) {
    return x + x;
}

int triple_it(int x) {
    return x + x + x;
}

int quad_it(int x) {
    return x + x + x + x;
}

int main() {
    int (*table[4])(int);
    table[0] = double_it;
    table[1] = triple_it;
    table[2] = quad_it;

    // double_it(5) = 10
    int r0 = table[0](5);
    if (r0 != 10) return 1;

    // triple_it(7) = 21
    int r1 = table[1](7);
    if (r1 != 21) return 2;

    // quad_it(3) = 12
    int r2 = table[2](3);
    if (r2 != 12) return 3;

    // sum = 10 + 21 + 12 = 43, need 52
    int sum = r0 + r1 + r2;
    if (sum != 43) return 4;

    // Loop dispatch: accumulate table[i](1) for i=0..2
    // double(1)=2, triple(1)=3, quad(1)=4 => 9
    int acc = 0;
    int i;
    for (i = 0; i < 3; i = i + 1) {
        acc = acc + table[i](1);
    }
    if (acc != 9) return 5;

    printf("D52OK\n");
    return 52;
}
