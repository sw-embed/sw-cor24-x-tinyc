; add.s -- Local variable arithmetic
; C equivalent: int main() { int a=2; int b=3; return a+b; }
; Expected: r0 = 5 at halt

        .text

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-6           ; two local vars (2 * 3 bytes)
        lc      r0,2
        sw      r0,-3(fp)       ; a = 2
        lc      r0,3
        sw      r0,-6(fp)       ; b = 3
        lw      r0,-3(fp)       ; load a
        lw      r1,-6(fp)       ; load b
        add     r0,r1           ; r0 = a + b = 5
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
