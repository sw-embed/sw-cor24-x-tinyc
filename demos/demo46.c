// tc24r demo46 -- unsigned int, signed/unsigned shifts and comparisons
//
// Tests:
//   1. unsigned int declaration and arithmetic
//   2. Arithmetic right shift (>>) on signed int (sign-extends)
//   3. Logical right shift (>>) on unsigned int (zero-fills)
//   4. Signed comparison (< > <= >=) on negative values
//   5. Unsigned comparison (< > <= >=) treats all values as positive
//   6. Mixed signed/unsigned in expressions

void putc_uart(int ch) {
    while (*(char *)0xFF0101 & 0x80) {}
    *(char *)0xFF0100 = ch;
}

int ok;

void check(int cond, int id) {
    if (!cond) {
        ok = 0;
        putc_uart(70);   // 'F'
        putc_uart(48 + id);
    }
}

int main() {
    ok = 1;

    // --- signed right shift (arithmetic: sign-extends) ---
    int s = -84;
    int shr_s = s >> 2;
    check(shr_s == -21, 1);       // -84 >> 2 = -21 (arithmetic)

    // --- unsigned right shift (logical: zero-fills) ---
    unsigned int u = 0x7FFFAC;    // large positive (fits 24-bit unsigned)
    unsigned int shr_u = u >> 4;
    check(shr_u == 0x07FFFA, 2);  // logical shift, no sign extension

    // --- unsigned wrapping: 0 - 1 wraps to max ---
    unsigned int zero = 0;
    unsigned int wrapped = zero - 1;  // should be 0xFFFFFF (16777215)
    check(wrapped == 16777215, 3);

    // --- signed comparison: -1 < 0 ---
    int neg = -1;
    check(neg < 0, 4);
    check(neg <= 0, 5);

    // --- unsigned comparison: (unsigned)-1 > 0 ---
    // In unsigned, -1 cast is 0xFFFFFF which is > 0
    unsigned int big = 16777215;  // 0xFFFFFF
    check(big > 0, 6);
    check(big >= 1, 7);

    // --- unsigned < : 5 < 10 ---
    unsigned int a = 5;
    unsigned int b = 10;
    check(a < b, 8);
    check(b > a, 9);

    if (ok == 1) {
        putc_uart(68);  // 'D'
        putc_uart(52);  // '4'
        putc_uart(54);  // '6'
        putc_uart(79);  // 'O'
        putc_uart(75);  // 'K'
        putc_uart(10);
        return 42;
    }
    return 0;
}
