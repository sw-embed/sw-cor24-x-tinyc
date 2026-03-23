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
        add     sp,-3
        lc      r0,1
        sw      r0,-3(fp)
        lw      r0,-3(fp)
        push    r0
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,0
        ceq     r0,r1
        brt     L2
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,1
        ceq     r0,r1
        brt     L3
        pop     r0
        bra     L4
L2:
        lc      r0,10
        bra     L0
L3:
        lc      r0,20
        bra     L0
L4:
        lc      r0,30
        bra     L0
L1:
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
