// tc24r demo55 -- constant expression in array size
//
// Use arithmetic expressions (#define constants) in array dimensions.
//
// Expected: r0 = 55, UART output: "D55OK"

#include <stdio.h>

#define ROWS 3
#define COLS 4
#define TOTAL (ROWS * COLS)

int buf[ROWS * COLS];
char tag[2 + 3];

int main() {
    // Fill global array sized by constant expression
    int i = 0;
    while (i < TOTAL) {
        buf[i] = i * 10;
        i = i + 1;
    }
    if (buf[0] != 0) return 1;
    if (buf[5] != 50) return 2;
    if (buf[11] != 110) return 3;

    // Local array with constant expression size
    int local[ROWS + COLS];
    local[0] = 100;
    local[6] = 200;
    if (local[0] != 100) return 4;
    if (local[6] != 200) return 5;

    // Char array with expression
    tag[0] = 'D';
    tag[1] = '5';
    tag[2] = '5';
    tag[3] = 'O';
    tag[4] = 'K';

    printf("D55OK\n");
    return 55;
}
