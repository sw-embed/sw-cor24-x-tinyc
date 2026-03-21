; return_const.s -- Minimal function that returns a constant
; C equivalent: int main() { return 42; }
; Expected: r0 = 42 at halt

        .text

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lc      r0,42           ; return value = 42
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
