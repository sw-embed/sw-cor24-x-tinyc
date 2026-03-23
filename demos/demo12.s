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
        add     sp,-12
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,0
        sw      r0,-6(fp)
L7:
        lc      r0,1
        sw      r0,-6(fp)
L8:
        lc      r0,0
        ceq     r0,z
        brf     L7
L9:
        lw      r0,-6(fp)
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L11
        lc      r0,0
        sw      r0,-3(fp)
L11:
        lc      r0,0
        sw      r0,-9(fp)
L12:
        lw      r0,-9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-9(fp)
L13:
        lw      r0,-9(fp)
        lc      r1,5
        cls     r0,r1
        brt     L12
L14:
        lw      r0,-9(fp)
        lc      r1,5
        ceq     r0,r1
        brt     L16
        lc      r0,0
        sw      r0,-3(fp)
L16:
        lc      r0,0
        sw      r0,-12(fp)
L17:
        lw      r0,-12(fp)
        lc      r1,100
        cls     r1,r0
        brf     L18
        la      r0,999
        sw      r0,-12(fp)
        bra     L17
L18:
        lw      r0,-12(fp)
        lc      r1,0
        ceq     r0,r1
        brt     L20
        lc      r0,0
        sw      r0,-3(fp)
L20:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L22
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L6
L22:
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
        .byte   68,49,50,79,75,10,0
