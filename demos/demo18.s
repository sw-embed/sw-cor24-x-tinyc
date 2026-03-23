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
        la      r1,128
        and     r0,r1
        ceq     r0,z
        brt     L2
        bra     L1
L2:
        la      r0,16711936
        mov     r1,r0
        lw      r0,9(fp)
        sb      r0,0(r1)
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _puts
_puts:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L4:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L5
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,_putc
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        bra     L4
L5:
L3:
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
        add     sp,-6
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,3
        lc      r1,3
        ceq     r0,r1
        brt     L8
        lc      r0,0
        sw      r0,-3(fp)
L8:
        lc      r0,1
        lc      r1,1
        ceq     r0,r1
        brt     L10
        lc      r0,0
        sw      r0,-3(fp)
L10:
        lc      r0,3
        lc      r1,3
        ceq     r0,r1
        brt     L12
        lc      r0,0
        sw      r0,-3(fp)
L12:
        lc      r0,3
        lc      r1,3
        ceq     r0,r1
        brt     L14
        lc      r0,0
        sw      r0,-3(fp)
L14:
        lc      r0,3
        lc      r1,3
        ceq     r0,r1
        brt     L16
        lc      r0,0
        sw      r0,-3(fp)
L16:
        lc      r0,4
        lc      r1,3
        mul     r0,r1
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,12
        ceq     r0,r1
        brt     L18
        lc      r0,0
        sw      r0,-3(fp)
L18:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L20
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L6
L20:
        lc      r0,0
        bra     L6
L6:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   68,49,56,79,75,10,0
