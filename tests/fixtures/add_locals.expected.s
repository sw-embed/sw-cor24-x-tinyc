        .text

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-6
        lc      r0,2
        sw      r0,-3(fp)
        lc      r0,3
        sw      r0,-6(fp)
        lw      r0,-3(fp)
        push    r0
        lw      r0,-6(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        bra     L0
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
