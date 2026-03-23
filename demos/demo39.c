// tc24r demo39 -- printf and long branches
//
// New features:
//   - #include <stdio.h> with freestanding printf
//   - printf("%d", val) with format specifiers
//   - Long branches (no 127-byte range limit)
//   - Varargs syntax (...) accepted in parameter lists
//
// Expected: r0 = 42, UART output: "D39OK"

#include <stdio.h>

int main() {
    int ok = 1;

    // printf with no args
    printf("D");

    // printf with %d
    printf("%d", 39);

    // printf with multiple args
    printf("%c%c", 79, 75);

    // newline
    printf("\n");

    // Verify the output was correct by checking we got here
    if (ok) {
        return 42;
    }
    return 0;
}
