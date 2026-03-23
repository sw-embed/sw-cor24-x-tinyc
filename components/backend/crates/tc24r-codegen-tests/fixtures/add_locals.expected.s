        .text

        .globl  _start
_start:
        la      r0,_main
        jal     r1,(r0)
_halt:
        bra     _halt

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
        lw      r1,-6(fp)
        add     r0,r1
        la      r2,L0
        jmp     (r2)
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
