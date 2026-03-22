        .text

        .globl  _start
_start:
        la      r0,_main
        jal     r1,(r0)
_halt:
        bra     _halt

        .globl  _fib
_fib:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        push    r0
        lc      r0,2
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L2
        lc      r0,1
        bra     L0
L2:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        sub     r0,r1
        push    r0
        la      r0,_fib
        jal     r1,(r0)
        add     sp,3
        push    r0
        lw      r0,9(fp)
        push    r0
        lc      r0,2
        mov     r1,r0
        pop     r0
        sub     r0,r1
        push    r0
        la      r0,_fib
        jal     r1,(r0)
        add     sp,3
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

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lc      r0,6
        push    r0
        la      r0,_fib
        jal     r1,(r0)
        add     sp,3
        bra     L3
L3:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
