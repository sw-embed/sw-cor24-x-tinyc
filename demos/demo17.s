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
        add     sp,-27
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,10
        sw      r0,-6(fp)
        lc      r0,20
        sw      r0,-9(fp)
        lc      r0,30
        sw      r0,-12(fp)
        lw      r0,-6(fp)
        lw      r1,-9(fp)
        add     r0,r1
        lw      r1,-12(fp)
        add     r0,r1
        lc      r1,60
        ceq     r0,r1
        brt     L8
        lc      r0,0
        sw      r0,-3(fp)
L8:
        lc      r0,3
        sw      r0,-15(fp)
        lc      r0,4
        sw      r0,-18(fp)
        lw      r0,-15(fp)
        lw      r1,-18(fp)
        add     r0,r1
        lc      r1,7
        ceq     r0,r1
        brt     L10
        lc      r0,0
        sw      r0,-3(fp)
L10:
        lc      r0,99
        sw      r0,-21(fp)
        lc      r0,-21
        add     r0,fp
        sw      r0,-24(fp)
        lc      r0,-21
        add     r0,fp
        sw      r0,-27(fp)
        lw      r0,-24(fp)
        lw      r0,0(r0)
        lc      r1,99
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L13
        lw      r0,-27(fp)
        lw      r0,0(r0)
        lc      r1,99
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L13
        lc      r0,0
        bra     L14
L13:
        lc      r0,1
L14:
        ceq     r0,z
        brt     L12
        lc      r0,0
        sw      r0,-3(fp)
L12:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L16
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L6
L16:
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
        .byte   68,49,55,79,75,10,0
