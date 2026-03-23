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

        .globl  _sum_array
_sum_array:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-6
        lc      r0,0
        sw      r0,-3(fp)
        lc      r0,0
        sw      r0,-6(fp)
L7:
        lw      r0,-6(fp)
        lw      r1,12(fp)
        cls     r0,r1
        brf     L8
        lw      r0,-3(fp)
        push    r0
        lw      r0,9(fp)
        push    r0
        lw      r0,-6(fp)
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        lw      r0,0(r0)
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-3(fp)
        lw      r0,-6(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-6(fp)
        bra     L7
L8:
        lw      r0,-3(fp)
        bra     L6
L6:
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
        add     sp,-21
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,-15
        add     r0,fp
        push    r0
        lc      r0,0
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     r1,r0
        lc      r0,10
        sw      r0,0(r1)
        lc      r0,-15
        add     r0,fp
        push    r0
        lc      r0,1
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     r1,r0
        lc      r0,20
        sw      r0,0(r1)
        lc      r0,-15
        add     r0,fp
        push    r0
        lc      r0,2
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     r1,r0
        lc      r0,30
        sw      r0,0(r1)
        lc      r0,-15
        add     r0,fp
        push    r0
        lc      r0,3
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     r1,r0
        lc      r0,40
        sw      r0,0(r1)
        lc      r0,-15
        add     r0,fp
        push    r0
        lc      r0,0
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        lc      r0,-15
        add     r0,fp
        push    r0
        lc      r0,3
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        lw      r0,0(r0)
        mov     r1,r0
        pop     r0
        add     r0,r1
        lc      r1,50
        ceq     r0,r1
        brt     L11
        lc      r0,0
        sw      r0,-3(fp)
L11:
        lc      r0,4
        push    r0
        lc      r0,-15
        add     r0,fp
        push    r0
        la      r0,_sum_array
        jal     r1,(r0)
        add     sp,6
        sw      r0,-18(fp)
        lw      r0,-18(fp)
        lc      r1,100
        ceq     r0,r1
        brt     L13
        lc      r0,0
        sw      r0,-3(fp)
L13:
        lc      r0,-21
        add     r0,fp
        lc      r1,0
        add     r0,r1
        mov     r1,r0
        lc      r0,65
        sb      r0,0(r1)
        lc      r0,-21
        add     r0,fp
        lc      r1,1
        add     r0,r1
        mov     r1,r0
        lc      r0,66
        sb      r0,0(r1)
        lc      r0,-21
        add     r0,fp
        lc      r1,2
        add     r0,r1
        mov     r1,r0
        lc      r0,0
        sb      r0,0(r1)
        lc      r0,-21
        add     r0,fp
        lc      r1,0
        add     r0,r1
        lbu     r0,0(r0)
        lc      r1,65
        ceq     r0,r1
        brt     L15
        lc      r0,0
        sw      r0,-3(fp)
L15:
        lc      r0,-21
        add     r0,fp
        lc      r1,1
        add     r0,r1
        lbu     r0,0(r0)
        lc      r1,66
        ceq     r0,r1
        brt     L17
        lc      r0,0
        sw      r0,-3(fp)
L17:
        lw      r0,-3(fp)
        lc      r1,1
        ceq     r0,r1
        brf     L19
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L9
L19:
        lc      r0,0
        bra     L9
L9:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   68,53,79,75,10,0
