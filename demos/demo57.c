// tc24r demo57 -- address-of struct members (&a.y, &a.b.x) passed to functions
//
// Exercises & on struct members (plain and nested), passing those
// pointers to functions, and using them for read/write.
//
// Expected: r0 = 57, UART output: "D57OK"

#include <stdio.h>

struct inner { int x; };
struct outer { struct inner b; int mode; };

int read_via_ptr(int *p) {
    return *p;
}

void write_via_ptr(int *p, int val) {
    *p = val;
}

int test_addr_of_simple_member(void) {
    struct point { int x; int y; };
    struct point p;
    p.x = 10;
    p.y = 20;
    int *px = &p.x;
    int *py = &p.y;
    if (*px != 10) return 1;
    if (*py != 20) return 2;
    write_via_ptr(px, 99);
    if (p.x != 99) return 3;
    return *px + *py;
}

int test_addr_of_nested_member(void) {
    struct outer a;
    a.b.x = 7;
    a.mode = 3;
    int *px = &a.b.x;
    int *pm = &a.mode;
    if (*px != 7) return 1;
    if (*pm != 3) return 2;
    write_via_ptr(px, 50);
    if (a.b.x != 50) return 3;
    return read_via_ptr(px) + read_via_ptr(pm);
}

int test_addr_of_passed_to_func(void) {
    struct point { int x; int y; };
    struct point p;
    p.x = 100;
    p.y = 200;
    int sum = read_via_ptr(&p.x) + read_via_ptr(&p.y);
    if (sum != 300) return 1;
    return sum;
}

int main() {
    int r = 0;

    r = test_addr_of_simple_member();
    if (r != 119) return 1;

    r = test_addr_of_nested_member();
    if (r != 53) return 2;

    r = test_addr_of_passed_to_func();
    if (r != 300) return 3;

    printf("D57OK\n");
    return 57;
}
