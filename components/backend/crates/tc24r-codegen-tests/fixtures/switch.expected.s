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
        brf     L5
        la      r2,L2
        jmp     (r2)
L5:
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,1
        ceq     r0,r1
        brf     L6
        la      r2,L3
        jmp     (r2)
L6:
        pop     r0
        la      r2,L4
        jmp     (r2)
L2:
        lc      r0,10
        la      r2,L0
        jmp     (r2)
L3:
        lc      r0,20
        la      r2,L0
        jmp     (r2)
L4:
        lc      r0,30
        la      r2,L0
        jmp     (r2)
L1:
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
