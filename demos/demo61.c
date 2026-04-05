// demo61.c — Inline assembly on COR24
// Tests: asm("..."), multi-string asm, asm volatile, extended asm syntax

int result;

int get_42(void) {
    // Basic inline asm: load immediate into r0 (return register)
    asm("lc r0,42");
}

int add_values(int a, int b) {
    // Multi-string concatenated asm
    asm("lw r0,9(fp)\n"
        "lw r1,12(fp)\n"
        "add r0,r1");
}

void store_global(int val) {
    // asm volatile — qualifier accepted, treated same as asm
    asm volatile("lw r0,9(fp)\n"
                 "la r1,_result\n"
                 "sw r0,0(r1)");
}

int main() {
    int ok = 1;

    // Test basic asm
    int v1 = get_42();
    if (v1 != 42) ok = 0;

    // Test multi-string asm
    int v2 = add_values(17, 25);
    if (v2 != 42) ok = 0;

    // Test asm volatile
    store_global(99);
    if (result != 99) ok = 0;

    return !ok;
}
