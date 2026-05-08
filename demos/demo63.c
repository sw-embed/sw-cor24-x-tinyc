// tc24r demo63 -- adjacent string-literal concatenation (C99 phase 6)
//
// Verifies end-to-end:
//   - basic two-chunk concat in a return expression (function returns
//     pointer to a single concatenated rodata literal)
//   - N-chunk concat across newlines, indexed via function-return ptr
//   - per-chunk escape resolution: "ab\n" "cd" yields a single 6-byte
//     literal in .data with the bytes {a,b,'\n',c,d,0}
//   - concat in a function-call argument
//   - sizeof(char arr[] = "abc" "def") reports 7, not 4 — both global
//     and local
//
// NOTE: This demo deliberately uses function-return + global char[]
// access to read string contents. tc24r currently has separate,
// pre-existing codegen issues with `char *global = "..."` writes and
// `char local[] = "..."` indexed reads — those are orthogonal to the
// concatenation feature this saga ships.
//
// Expected: r0 = 63, UART output: "D63OK"

#include <stdio.h>

/* Global char array — this codegen path stores bytes literally and
   indexing works correctly. */
char g_arr[] = "abc" "def";

/* Returns a pointer to the (single) concatenated rodata literal. */
char *escape_str(void) {
    return "ab\n" "cd";
}

char *greeting(void) {
    return "hello, "
           "world!"
           " "
           "D63";
}

int main(void) {
    /* Per-chunk escape resolution: "ab\n" + "cd" must resolve each
       chunk's escapes first, then join. So byte 2 is the literal
       newline (10), not '\\' (92). */
    char *e = escape_str();
    if (e[0] != 'a' || e[1] != 'b' || e[2] != '\n'
        || e[3] != 'c' || e[4] != 'd' || e[5] != 0) {
        return 1;
    }

    /* Global implicit-size: sizeof(g_arr) must use the concatenated
       length (7 = 6 chars + null), not 4 (just "abc" + null). */
    if (sizeof(g_arr) != 7) return 2;
    if (g_arr[0] != 'a' || g_arr[5] != 'f' || g_arr[6] != 0) return 3;

    /* Local sizeof — the bug fix this saga is centred on. Indexing
       into the local is skipped (separate codegen issue); sizeof
       alone is sufficient to verify the size-inference fix. */
    char local[] = "ab" "cd" "ef" "gh";
    if (sizeof(local) != 9) return 4;

    /* Function returning a multi-chunk concatenated string. */
    char *gr = greeting();
    if (gr[0] != 'h' || gr[7] != 'w' || gr[13] != ' ') return 5;
    if (gr[14] != 'D' || gr[15] != '6' || gr[16] != '3' || gr[17] != 0) return 6;

    /* In a function call argument. */
    printf("D63" "OK\n");
    return 63;
}
