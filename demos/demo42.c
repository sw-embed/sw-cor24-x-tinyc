// tc24r demo42 -- nested struct member access (linked list)
//
// New feature:
//   - a.next->val (chained dot + arrow access)
//   - a->b->c (multi-level arrow traversal)
//   - Codegen resolves struct types through expression chains
//
// Expected: r0 = 42, UART output: "D42OK"

#include <stdio.h>
#include <stdlib.h>

struct node {
    int val;
    struct node *next;
};

int sum_list(struct node *p) {
    int total = 0;
    while (p != (struct node *)0) {
        total = total + p->val;
        p = p->next;
    }
    return total;
}

int main() {
    int ok = 1;

    // Build a linked list: 10 -> 20 -> 12
    struct node a;
    struct node b;
    struct node c;
    a.val = 10; a.next = &b;
    b.val = 20; b.next = &c;
    c.val = 12; c.next = (struct node *)0;

    // Chained access
    if (a.next->val != 20) ok = 0;
    if (a.next->next->val != 12) ok = 0;

    // Walk the list
    if (sum_list(&a) != 42) ok = 0;

    if (ok) {
        printf("D42OK\n");
        return 42;
    }
    return 0;
}
