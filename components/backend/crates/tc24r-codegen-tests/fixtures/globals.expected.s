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
        la      r1,_x
        lw      r0,0(r1)
        push    r0
        lc      r0,5
        mov     r1,r0
        pop     r0
        add     r0,r1
        la      r1,_x
        sw      r0,0(r1)
        la      r1,_x
        lw      r0,0(r1)
        la      r2,L0
        jmp     (r2)
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_x:
        .word   10
