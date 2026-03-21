; branch.s -- Conditional branch
; C equivalent: int main() { if (1) return 3; else return 4; }
; Expected: r0 = 3 at halt

        .text

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lc      r0,1            ; condition = 1
        ceq     r0,z            ; test r0 == 0?
        brt     .else           ; if true (r0==0), go to else
        lc      r0,3            ; return 3
        bra     .done
.else:
        lc      r0,4            ; return 4
.done:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
