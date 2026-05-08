// tc24r demo62 -- parenthesised and mixed const-expr in array sizes
//
// Complements demo55 by exercising:
//   - parens for grouping             [(A + B) * C]
//   - identifier × literal mix        [N * 16]
//   - nested parens                   [((A + 1) * (B - 1))]
//   - mul tighter than add            [A + B * C]   == 2 + 3*4 = 14
//
// Expected: r0 = 62, UART output: "D62OK"

#include <stdio.h>

#define A 2
#define B 3
#define C 4
#define N 4

char paren[(A + B) * C];          /* 5 * 4 = 20  */
char mix[N * 16];                 /* 4 * 16 = 64 */
char nested[((A + 1) * (B - 1))]; /* 3 * 2 = 6   */
char prec[A + B * C];             /* 2 + 12 = 14 */

int main(void) {
    /* Touch each array at indices that prove the size. If any of
       these out-of-bounds in the literal-equivalent layout, the
       compiler would have either rejected the size or laid the
       array out wrong. */
    paren[0] = 'P';
    paren[19] = 'p';
    mix[0] = 'M';
    mix[63] = 'm';
    nested[0] = 'N';
    nested[5] = 'n';
    prec[0] = 'R';
    prec[13] = 'r';

    if (paren[0] != 'P' || paren[19] != 'p') return 1;
    if (mix[0] != 'M' || mix[63] != 'm') return 2;
    if (nested[0] != 'N' || nested[5] != 'n') return 3;
    if (prec[0] != 'R' || prec[13] != 'r') return 4;

    printf("D62OK\n");
    return 62;
}
