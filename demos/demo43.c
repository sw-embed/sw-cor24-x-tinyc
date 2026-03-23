// tc24r demo43 -- Lisp-style cons cells (struct pointer return + nested access)
//
// New features:
//   - Functions returning struct pointers (struct val *make())
//   - Nested arrow access through malloc'd structs
//   - Cons cell pattern: car/cdr linked list
//
// Expected: r0 = 42, UART output: "D43OK"

#include <stdio.h>
#include <stdlib.h>

struct val { int tag; int num; struct val *car; struct val *cdr; };

struct val *make_num(int n) {
    struct val *v = (struct val *)malloc(12);
    v->tag = 0;
    v->num = n;
    return v;
}

struct val *cons(struct val *a, struct val *d) {
    struct val *v = (struct val *)malloc(12);
    v->tag = 1;
    v->car = a;
    v->cdr = d;
    return v;
}

int main() {
    int ok = 1;

    // Build list: (1 2 3)
    struct val *nil = make_num(0);
    struct val *lst = cons(make_num(1), cons(make_num(2), cons(make_num(3), nil)));

    // car = 1
    if (lst->car->num != 1) ok = 0;
    // cadr = 2
    if (lst->cdr->car->num != 2) ok = 0;
    // caddr = 3
    if (lst->cdr->cdr->car->num != 3) ok = 0;

    // Sum the list
    int sum = 0;
    struct val *p = lst;
    while (p->tag == 1) {
        sum = sum + p->car->num;
        p = p->cdr;
    }
    if (sum != 6) ok = 0;

    if (ok) {
        printf("D43OK\n");
        return 42;
    }
    return 0;
}
