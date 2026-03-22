; uart_rx.s -- UART receive (polling)
; C equivalent:
;   int getc(void) {
;       volatile char *status = (char *)0xFF0101;
;       volatile char *data = (char *)0xFF0100;
;       while (!(*status & 1)) {}   // wait for RX ready (bit 0)
;       return *data;
;   }
;   int main() { return getc(); }
; Expected: r0 = received byte from UART

        .text

        .globl  _getc
_getc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la      r2,0xFF0100     ; UART base address
.wait:
        lbu     r0,1(r2)        ; read UART status (zero-extended)
        lc      r1,1
        and     r0,r1           ; isolate bit 0 (RX ready)
        ceq     r0,z            ; test r0 == 0?
        brt     .wait           ; if zero (not ready), keep waiting
        lbu     r0,0(r2)        ; read received byte
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la      r0,_getc
        jal     r1,(r0)         ; call getc()
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
