        .text

        .globl  _start
_start:
        la      r0,_main
        jal     r1,(r0)
_halt:
        bra     _halt

        .globl  _putc
_putc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L1:
        la      r0,16711937
        lbu     r0,0(r0)
        push    r0
        la      r0,128
        mov     r1,r0
        pop     r0
        and     r0,r1
        ceq     r0,z
        brt     L2
        bra     L1
L2:
        lw      r0,9(fp)
        push    r0
        la      r0,16711936
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
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
        lc      r0,72
        push    r0
        la      r0,_putc
        jal     r1,(r0)
        add     sp,3
        lc      r0,105
        push    r0
        la      r0,_putc
        jal     r1,(r0)
        add     sp,3
        lc      r0,33
        push    r0
        la      r0,_putc
        jal     r1,(r0)
        add     sp,3
        lc      r0,10
        push    r0
        la      r0,_putc
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        bra     L3
L3:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
