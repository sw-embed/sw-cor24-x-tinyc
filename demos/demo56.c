// tc24r demo56 -- postfix ++/-- on struct members and array elements
//
// Exercises ++ and -- on struct members (. and ->), array elements,
// and plain locals. Both prefix and postfix forms.
//
// Expected: r0 = 56, UART output: "D56OK"

#include <stdio.h>

struct point { int x; int y; };

int test_struct_dot(void) {
    struct point p;
    p.x = 10;
    p.x++;
    if (p.x != 11) return 1;
    p.y = 20;
    p.y--;
    if (p.y != 19) return 2;
    ++p.x;
    if (p.x != 12) return 3;
    --p.y;
    if (p.y != 18) return 4;
    return p.x + p.y;
}

int test_struct_arrow(void) {
    struct point p;
    struct point *q = &p;
    q->x = 5;
    q->x++;
    if (q->x != 6) return 1;
    --q->x;
    if (q->x != 5) return 2;
    return q->x;
}

int test_array(void) {
    int a[4];
    a[0] = 0;
    a[1] = 0;
    a[2] = 0;
    a[3] = 0;
    a[0] = 100;
    a[0]++;
    if (a[0] != 101) return 1;
    a[2] = 50;
    a[2]--;
    if (a[2] != 49) return 2;
    ++a[3];
    if (a[3] != 1) return 3;
    --a[1];
    if (a[1] != -1) return 4;
    return a[0] + a[1] + a[2] + a[3];
}

int test_postinc_value(void) {
    int i = 7;
    int j = i++;
    if (j != 7) return 1;
    if (i != 8) return 2;
    return j + i;
}

int main() {
    int r = 0;

    r = test_struct_dot();
    if (r != 30) return 1;

    r = test_struct_arrow();
    if (r != 5) return 2;

    r = test_array();
    if (r != 150) return 3;

    r = test_postinc_value();
    if (r != 15) return 4;

    printf("D56OK\n");
    return 56;
}
