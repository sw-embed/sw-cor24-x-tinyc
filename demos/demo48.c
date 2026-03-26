// tc24r demo48 -- global struct array declaration and access
//
// BUG-011 fix: struct arrays at global scope now parse correctly.
// Previously failed with "expected Semicolon, got LBracket".
// Also required fixing data emission to allocate correct size
// for struct array elements (multiple words per struct).
//
// Expected: r0 = 42, UART output: "D48OK"

#include <stdio.h>

struct entry { int key; int val; };
struct entry table[4];

int lookup(int k) {
    int i = 0;
    while (i < 4) {
        if (table[i].key == k) {
            return table[i].val;
        }
        i = i + 1;
    }
    return -1;
}

int main() {
    // Populate table
    table[0].key = 1;  table[0].val = 10;
    table[1].key = 2;  table[1].val = 20;
    table[2].key = 3;  table[2].val = 30;
    table[3].key = 4;  table[3].val = 40;

    // Lookup and sum
    int a = lookup(2);  // 20
    int b = lookup(3);  // 30

    int ok = 1;
    if (a != 20) ok = 0;
    if (b != 30) ok = 0;
    if (lookup(99) != -1) ok = 0;

    if (ok) {
        printf("D48OK\n");
        return 42;
    }
    return 0;
}
