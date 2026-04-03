// tc24r demo58 -- global data initialization
//
// Exercises static/global array initialization with string literals
// and initializer lists, including char and int arrays, zero-padding,
// and runtime access.
//
// Expected: r0 = 58, UART output: "D58OK"

#include <stdio.h>

static const char greeting[] = "hello";
int fib_table[] = {1, 1, 2, 3, 5, 8, 13, 21};
char vowels[] = {'A', 'E', 'I', 'O', 'U', 0};
int sparse[5] = {100, 200};

int test_string_init(void) {
    if (greeting[0] != 'h') return 1;
    if (greeting[4] != 'o') return 2;
    if (greeting[5] != '\0') return 3;
    return greeting[0] + greeting[1] + greeting[2];
}

int test_int_init_list(void) {
    if (fib_table[0] != 1) return 1;
    if (fib_table[5] != 8) return 2;
    if (fib_table[7] != 21) return 3;
    return fib_table[0] + fib_table[1] + fib_table[2];
}

int test_char_init_list(void) {
    if (vowels[0] != 'A') return 1;
    if (vowels[4] != 'U') return 2;
    if (vowels[5] != '\0') return 3;
    return vowels[0] + vowels[1] + vowels[2] + vowels[3] + vowels[4];
}

int test_zero_padding(void) {
    if (sparse[0] != 100) return 1;
    if (sparse[1] != 200) return 2;
    if (sparse[2] != 0) return 3;
    if (sparse[4] != 0) return 4;
    return sparse[0] + sparse[1];
}

int main() {
    int r = 0;

    r = test_string_init();
    if (r != 104 + 101 + 108) return 1;

    r = test_int_init_list();
    if (r != 4) return 2;

    r = test_char_init_list();
    if (r != 'A' + 'E' + 'I' + 'O' + 'U') return 3;

    r = test_zero_padding();
    if (r != 300) return 4;

    printf("D58OK\n");
    return 58;
}
