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
        lc      r0,1
        ceq     r0,z
        brt     L1
        lc      r0,3
        bra     L0
        bra     L2
L1:
        lc      r0,4
        bra     L0
L2:
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
