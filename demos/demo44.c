// tc24r demo44 -- Lisp Phase 1: data types, constructors, predicates, printer
//
// Implements McCarthy's core primitives in C:
//   - cons, car, cdr, atom, eq, null
//   - make_num, make_sym, make_nil
//   - print_val (recursive S-expression printer)
//
// Expected: r0 = 42, UART output: "D44OK"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// --- Value representation ---

enum { T_NUM, T_SYM, T_CONS, T_NIL };

struct val {
    int tag;
    int num;
    char *sym;
    struct val *car;
    struct val *cdr;
};

// --- Constructors ---

struct val *make_nil(void) {
    struct val *v = (struct val *)malloc(15);
    v->tag = T_NIL;
    return v;
}

struct val *make_num(int n) {
    struct val *v = (struct val *)malloc(15);
    v->tag = T_NUM;
    v->num = n;
    return v;
}

struct val *make_sym(char *s) {
    struct val *v = (struct val *)malloc(15);
    v->tag = T_SYM;
    v->sym = s;
    return v;
}

struct val *cons(struct val *a, struct val *d) {
    struct val *v = (struct val *)malloc(15);
    v->tag = T_CONS;
    v->car = a;
    v->cdr = d;
    return v;
}

// --- Accessors ---

struct val *car(struct val *x) { return x->car; }
struct val *cdr(struct val *x) { return x->cdr; }

// --- Predicates ---

int atom(struct val *x) {
    return x->tag != T_CONS;
}

int null(struct val *x) {
    return x->tag == T_NIL;
}

int eq(struct val *a, struct val *b) {
    if (a->tag != b->tag) return 0;
    if (a->tag == T_NUM) return a->num == b->num;
    if (a->tag == T_SYM) return strcmp(a->sym, b->sym) == 0;
    if (a->tag == T_NIL) return 1;
    return 0;
}

// --- Printer ---

void print_val(struct val *x) {
    if (x->tag == T_NUM) {
        printf("%d", x->num);
    } else if (x->tag == T_SYM) {
        printf("%s", x->sym);
    } else if (x->tag == T_NIL) {
        printf("nil");
    } else if (x->tag == T_CONS) {
        printf("(");
        print_val(x->car);
        struct val *rest = x->cdr;
        while (rest->tag == T_CONS) {
            printf(" ");
            print_val(rest->car);
            rest = rest->cdr;
        }
        if (rest->tag != T_NIL) {
            printf(" . ");
            print_val(rest);
        }
        printf(")");
    }
}

// --- Tests ---

int main() {
    int ok = 1;
    struct val *NIL = make_nil();

    // Constructors + accessors
    struct val *n1 = make_num(1);
    struct val *n2 = make_num(2);
    struct val *n3 = make_num(3);
    struct val *sa = make_sym("hello");

    if (n1->num != 1) ok = 0;
    if (sa->tag != T_SYM) ok = 0;

    // cons + car + cdr
    struct val *pair = cons(n1, n2);
    if (car(pair)->num != 1) ok = 0;
    if (cdr(pair)->num != 2) ok = 0;

    // Predicates
    if (!atom(n1)) ok = 0;
    if (atom(pair)) ok = 0;
    if (!null(NIL)) ok = 0;
    if (null(n1)) ok = 0;
    if (!eq(make_num(5), make_num(5))) ok = 0;
    if (eq(make_num(5), make_num(6))) ok = 0;
    if (!eq(make_sym("x"), make_sym("x"))) ok = 0;
    if (eq(make_sym("x"), make_sym("y"))) ok = 0;

    // Build list: (1 2 3)
    struct val *lst = cons(n1, cons(n2, cons(n3, NIL)));

    // Print list
    print_val(lst);
    printf("\n");

    // Print dotted pair
    print_val(cons(make_sym("a"), make_sym("b")));
    printf("\n");

    // Print nested: ((1 2) 3)
    print_val(cons(cons(n1, cons(n2, NIL)), cons(n3, NIL)));
    printf("\n");

    if (ok) {
        printf("D44OK\n");
        return 42;
    }
    return 0;
}
