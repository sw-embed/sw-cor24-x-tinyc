; interrupt.s -- UART RX interrupt handler
; C equivalent (with inline asm for ISR setup):
;   volatile int rx_char = 0;
;
;   void isr_entry(void) {
;       // compiler-generated ISR prologue would go here
;       volatile char *uart = (char *)0xFF0100;
;       rx_char = *uart;  // read clears interrupt
;       // compiler-generated ISR epilogue would go here
;   }
;
;   int main() {
;       asm("la iv,_isr_entry");          // set interrupt vector
;       *(volatile char *)0xFF0010 = 1;   // enable UART RX interrupt
;       while (!rx_char) {}               // wait for character
;       return rx_char;
;   }
;
; Expected: r0 = first byte received via UART interrupt

        .text

; ISR entry -- saves all regs, reads UART, restores, returns via ir
        .globl  _isr_entry
_isr_entry:
        push    r0
        push    r1
        push    r2
        mov     r2,c            ; save condition flag in r2
        push    r2
        push    fp
        mov     fp,sp
        ; ISR body: read UART data (auto-acknowledges interrupt)
        la      r1,0xFF0100
        lbu     r0,0(r1)        ; read received byte
        la      r1,_rx_char
        sw      r0,0(r1)        ; store to global
        ; ISR epilogue
        mov     sp,fp
        pop     fp
        pop     r2
        clu     z,r2            ; restore condition flag
        pop     r2
        pop     r1
        pop     r0
        jmp     (ir)            ; return from interrupt

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        ; Set interrupt vector to _isr_entry
        la      r0,_isr_entry
        mov     iv,r0
        ; Enable UART RX interrupt
        la      r1,0xFF0010
        lc      r0,1
        sb      r0,0(r1)
        ; Wait for interrupt to set rx_char
.poll:
        la      r1,_rx_char
        lw      r0,0(r1)
        ceq     r0,z            ; rx_char == 0?
        brt     .poll           ; keep waiting if zero
        ; Return rx_char
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_rx_char:
        .word   0
