// cc24 demo2 -- Phase 4 features: char, pointers, casts, MMIO
//
// New features since demo.c:
//   - C comments (// and /* */)
//   - char type
//   - pointer declarations (int *, char *)
//   - address-of (&x)
//   - pointer dereference (*p) read and write
//   - cast expressions ((char *)addr)
//   - MMIO: LED D2 control, UART TX output
//
// COR24-TB hardware: 1 LED (D2), 1 button (S2), UART
//   LED at 0xFF0000, active-low: bit 0 = 0 turns LED on, 1 turns off
//   UART data at 0xFF0100
//
// Expected result after execution:
//   r0 = 42
//   LED = 0x00 (D2 on, active-low)
//   UART output: "OK"

/* MMIO helpers */
void led_on() {
    *(char *)0xFF0000 = 0;  /* active-low: 0 = LED on */
}

void led_off() {
    *(char *)0xFF0000 = 1;  /* active-low: 1 = LED off */
}

void uart_putc(int c) {
    while (*(char *)0xFF0101 & 0x80) {}
    *(char *)0xFF0100 = c;
}

int main() {
    int ok = 1;

    // --- char type ---
    char a = 65;
    char b = 66;
    if (a != 65) { ok = 0; }
    if (b != 66) { ok = 0; }

    // --- pointer to int: read via dereference ---
    int x = 123;
    int *px = &x;
    if (*px != 123) { ok = 0; }

    // --- pointer write: modify through pointer ---
    *px = 456;
    if (x != 456) { ok = 0; }

    // --- pointer to char ---
    char ch = 77;
    char *pc = &ch;
    if (*pc != 77) { ok = 0; }

    // --- MMIO: turn on LED D2 ---
    led_on();

    // --- UART output ---
    if (ok == 1) {
        uart_putc(79);   /* 'O' */
        uart_putc(75);   /* 'K' */
        uart_putc(10);   /* newline */
    }

    if (ok == 1) {
        return 42;
    }
    return 0;
}
