// cc24 demo9 -- UART RX interrupt
//
// New features:
//   - __attribute__((interrupt)) ISR with automatic prologue/epilogue
//   - Interrupt vector setup via asm()
//   - MMIO interrupt enable
//   - Polling a global set by ISR
//
// Expected: r0 = received UART character (0x41 = 'A' when --uart-input "A")

#define UART_DATA   0xFF0100
#define UART_STATUS 0xFF0101
#define INT_ENABLE  0xFF0010

int rx_char = 0;

__attribute__((interrupt))
void uart_isr() {
    // Read UART data register (clears interrupt)
    rx_char = *(char *)UART_DATA;
}

int main() {
    // Set interrupt vector to uart_isr
    asm("la r0,_uart_isr\nmov iv,r0");

    // Enable UART RX interrupt
    *(char *)INT_ENABLE = 1;

    // Poll until ISR sets rx_char
    while (rx_char == 0) {}

    return rx_char;
}
