// cc24 demo10 -- #include, #pragma once, system header
//
// New features:
//   - #include "file.h" (relative to source file)
//   - #include <cor24.h> (system include path via -I)
//   - #pragma once (prevents double inclusion)
//   - -I flag for include search paths
//
// Expected: r0 = 42, LED D2 on, UART output: "D10OK"

#include "demo10_io.h"
#include "demo10_io.h"
#include <cor24.h>

int main() {
    int ok = 1;

    // Verify #define from local header
    if (LED_REG != 0xFF0000) { ok = 0; }

    // Verify #define from system header (cor24.h)
    if (UART_STATUS != 0xFF0101) { ok = 0; }

    // Use functions from included header
    led_on();

    if (ok == 1) {
        puts("D10OK\n");
        return 42;
    }
    return 0;
}
