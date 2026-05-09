// tc24r demo64 -- no-std minimal: copy "hello" to memory and halt.
//
// No #include directives. Nothing from <stdio.h>, <stdlib.h>, <string.h>.
// This is the smallest possible program that proves the DCE pass
// produces a clean `.s` when nothing is needed:
//   - no _abs, _atoi, _malloc, _free, _printf*, _strcmp, ...
//   - just _start, _halt, _main, the data symbol for the buffer,
//     and any codegen-emitted helpers actually used (none here).
//
// The 'expected r0' is just sizeof("hello") (5 chars) so we can
// verify it ran. The runner script also checks that DCE has
// trimmed the symbol table.
//
// Expected: r0 = 5

char buf[5];

int main(void) {
    buf[0] = 'h';
    buf[1] = 'e';
    buf[2] = 'l';
    buf[3] = 'l';
    buf[4] = 'o';

    /* Sum-of-indices proof we wrote each byte by referring to the
       global location, not by short-circuiting. Result is just
       a witness — meaningful number is buf[0]..[4] vs 'h'..'o'. */
    int n = 0;
    if (buf[0] == 'h') n = n + 1;
    if (buf[1] == 'e') n = n + 1;
    if (buf[2] == 'l') n = n + 1;
    if (buf[3] == 'l') n = n + 1;
    if (buf[4] == 'o') n = n + 1;
    return n; /* 5 */
}
