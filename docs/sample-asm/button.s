; button.s -- Read button state via MMIO
; C equivalent:
;   int button_pressed(void) {
;       volatile char *gpio = (char *)0xFF0000;
;       return *gpio & 1;       // bit 0 = button state
;   }
;   int main() { return button_pressed(); }
; Expected: r0 = 0 or 1 depending on button state

        .text

        .globl  _button_pressed
_button_pressed:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la      r1,0xFF0000     ; GPIO address
        lbu     r0,0(r1)        ; read button state (zero-extended)
        lc      r1,1
        and     r0,r1           ; mask to bit 0
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
        la      r0,_button_pressed
        jal     r1,(r0)         ; call button_pressed()
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
