// tc24r demo63 -- adjacent string-literal concatenation (C99 phase 6)
//                  + char-storage init (pr/codegen-string-storage-bugs)
//
// Verifies end-to-end:
//   - basic two-chunk concat in a return expression
//   - N-chunk concat across newlines, indexed via function-return ptr
//   - per-chunk escape resolution: "ab\n" "cd" yields a single 6-byte
//     literal in .data with the bytes {a,b,'\n',c,d,0}
//   - concat in a function-call argument
//   - sizeof(char arr[] = "abc" "def") reports 7, not 4 — both global
//     and local
//   - char *global = "..." indexing reads the string bytes
//   - char local[] = "..." indexing reads the string bytes (zero-pad
//     for explicit size)
//
// Expected: r0 = 63, UART output: "D63OK"

#include <stdio.h>

/* Global char array — bytes-in-place storage. */
char g_arr[] = "abc" "def";

/* Global char pointer — must store the address of an anonymous rodata
   literal, NOT the bytes inline. */
char *g_ptr = "ab\n" "cd";

char *greeting(void) {
    return "hello, "
           "world!"
           " "
           "D63";
}

int main(void) {
    /* Per-chunk escape resolution via global pointer (bug 1 path). */
    if (g_ptr[0] != 'a' || g_ptr[1] != 'b' || g_ptr[2] != '\n'
        || g_ptr[3] != 'c' || g_ptr[4] != 'd' || g_ptr[5] != 0) {
        return 1;
    }

    /* Global implicit-size: sizeof(g_arr) must use the concatenated
       length (7 = 6 chars + null), not 4 (just "abc" + null). */
    if (sizeof(g_arr) != 7) return 2;
    if (g_arr[0] != 'a' || g_arr[5] != 'f' || g_arr[6] != 0) return 3;

    /* Local implicit-size: sizeof + byte-level access (bug 2 path). */
    char local[] = "ab" "cd" "ef" "gh";
    if (sizeof(local) != 9) return 4;
    if (local[0] != 'a' || local[7] != 'h' || local[8] != 0) return 5;

    /* Local explicit-size with zero pad — char[10] = "abc". */
    char padded[10] = "abc";
    if (padded[0] != 'a' || padded[2] != 'c' || padded[3] != 0) return 6;
    if (padded[9] != 0) return 7; /* trailing zero pad */

    /* Function returning a multi-chunk concatenated string. */
    char *gr = greeting();
    if (gr[0] != 'h' || gr[7] != 'w' || gr[13] != ' ') return 8;
    if (gr[14] != 'D' || gr[15] != '6' || gr[16] != '3' || gr[17] != 0) return 9;

    /* In a function call argument. */
    printf("D63" "OK\n");
    return 63;
}
