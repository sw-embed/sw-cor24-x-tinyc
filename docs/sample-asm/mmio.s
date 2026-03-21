; mmio.s -- Memory-mapped I/O: send 'A' via UART
; C equivalent:
;   void putc(int c) {
;       volatile char *status = (char *)0xFF0101;
;       volatile char *data = (char *)0xFF0100;
;       while (*status < 0) {}  // wait for TX not busy
;       *data = c;
;   }
;   int main() { putc('A'); return 0; }
; Expected: 'A' (0x41) transmitted on UART TX

        .text

        .globl  _putc
_putc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la      r2,0xFF0100     ; UART base address
.wait:
        lb      r0,1(r2)        ; read UART status (sign-extended)
        cls     r0,z            ; status < 0? (bit 7 = TX busy)
        brt     .wait           ; spin while TX busy
        lw      r0,9(fp)        ; load character argument
        sb      r0,0(r2)        ; transmit byte
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
        lc      r0,65           ; 'A' = 0x41
        push    r0              ; push argument
        la      r0,_putc
        jal     r1,(r0)         ; call putc('A')
        add     sp,3            ; clean up argument
        lc      r0,0            ; return 0
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
