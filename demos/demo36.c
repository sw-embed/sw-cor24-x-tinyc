// tc24r demo36 -- forward-declared struct tags
//
// New features:
//   - struct foo; (forward declaration before body)
//   - struct node { struct node *next; } (self-referential)
//   - struct foo *p; (pointer to incomplete type)
//
// Expected: r0 = 42, UART output: "D36OK"

#define UART_DATA 0xFF0100
#define UART_STATUS 0xFF0101
void putc(int c) { while (*(char *)UART_STATUS & 0x80) {} *(char *)UART_DATA = c; }
void puts(char *s) { while (*s) { putc(*s); s = s + 1; } }

// Forward declaration
struct item;

// Definition after forward decl
struct item {
    int value;
};

// Self-referential struct (linked list node)
struct node {
    int data;
    struct node *next;
};

int main() {
    int ok = 1;

    // Forward-declared struct
    struct item it;
    it.value = 10;
    if (it.value != 10) ok = 0;

    // Self-referential struct
    struct node a;
    struct node b;
    a.data = 1;
    a.next = &b;
    b.data = 2;
    b.next = (struct node *)0;

    if (a.data != 1) ok = 0;
    if (b.data != 2) ok = 0;
    // Pointer stored correctly
    if (a.next != &b) ok = 0;

    if (ok) {
        puts("D36OK\n");
        return 42;
    }
    return 0;
}
