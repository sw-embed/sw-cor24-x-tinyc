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
        lc      r0,0
        sw      r0,-3(fp)
L1:
        lw      r0,-3(fp)
        lc      r1,5
        cls     r0,r1
        brt     L3
        la      r2,L2
        jmp     (r2)
L3:
        lw      r0,-3(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-3(fp)
        la      r2,L1
        jmp     (r2)
L2:
        lw      r0,-3(fp)
        la      r2,L0
        jmp     (r2)
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
