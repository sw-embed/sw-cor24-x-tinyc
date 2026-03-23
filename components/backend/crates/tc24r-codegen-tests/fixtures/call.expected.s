        .text

        .globl  _start
_start:
        la      r0,_main
        jal     r1,(r0)
_halt:
        bra     _halt

        .globl  _add
_add:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        push    r0
        lw      r0,12(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        la      r2,L0
        jmp     (r2)
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lc      r0,5
        push    r0
        lc      r0,2
        push    r0
        la      r0,_add
        jal     r1,(r0)
        add     sp,6
        la      r2,L1
        jmp     (r2)
L1:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
