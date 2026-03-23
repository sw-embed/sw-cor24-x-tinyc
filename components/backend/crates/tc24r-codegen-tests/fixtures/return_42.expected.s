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
        lc      r0,42
        la      r2,L0
        jmp     (r2)
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
