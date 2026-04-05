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
        lc      r0,1
        push    r0
        lc      r0,0
        pop     r1
        sub     r0,r1
        sw      r0,-6(fp)
        sw      r0,-3(fp)
        lw      r0,-6(fp)
        lc      r1,1
        shl     r0,r1
        push    r0
        lw      r0,-3(fp)
        lc      r1,23
        srl     r0,r1
        mov     r1,r0
        pop     r0
        or      r0,r1
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,1
        shl     r0,r1
        push    r0
        lw      r0,-3(fp)
        lc      r1,0
        cls     r0,r1
        mov     r0,c
        mov     r1,r0
        pop     r0
        or      r0,r1
        sw      r0,-6(fp)
        lw      r0,-3(fp)
        lc      r1,1
        shl     r0,r1
        sw      r0,-3(fp)
        lc      r0,0
        bra     L0
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
