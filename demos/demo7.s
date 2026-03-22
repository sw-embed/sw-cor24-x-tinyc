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

        .globl  _puts
_puts:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L2:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L3
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,_putc
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        bra     L2
L3:
L1:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _strlen
_strlen:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lw      r0,9(fp)
        sw      r0,-3(fp)
L5:
        lw      r0,-3(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L6
        lw      r0,-3(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-3(fp)
        bra     L5
L6:
        lw      r0,-3(fp)
        push    r0
        lw      r0,9(fp)
        mov     r1,r0
        pop     r0
        sub     r0,r1
        bra     L4
L4:
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
        add     sp,-30
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,10
        push    r0
        lc      r0,-12
        add     r0,fp
        push    r0
        lc      r0,0
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,20
        push    r0
        lc      r0,-12
        add     r0,fp
        push    r0
        lc      r0,1
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,30
        push    r0
        lc      r0,-12
        add     r0,fp
        push    r0
        lc      r0,2
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,-12
        add     r0,fp
        push    r0
        lc      r0,2
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-15(fp)
        lw      r0,-15(fp)
        push    r0
        lc      r0,1
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        sub     r0,r1
        sw      r0,-15(fp)
        lw      r0,-15(fp)
        lw      r0,0(r0)
        push    r0
        lc      r0,20
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L9
        lc      r0,0
        sw      r0,-3(fp)
L9:
        lc      r0,-12
        add     r0,fp
        sw      r0,-18(fp)
        lc      r0,-12
        add     r0,fp
        push    r0
        lc      r0,3
        lc      r1,3
        mul     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-21(fp)
        lw      r0,-21(fp)
        push    r0
        lw      r0,-18(fp)
        mov     r1,r0
        pop     r0
        sub     r0,r1
        push    r0
        lc      r0,3
        push    r0
        pop     r1
        pop     r0
        push    r1
        push    r0
        la      r0,__cc24_div
        jal     r1,(r0)
        add     sp,6
        push    r0
        lc      r0,3
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L11
        lc      r0,0
        sw      r0,-3(fp)
L11:
        la      r0,_S0
        push    r0
        la      r0,_strlen
        jal     r1,(r0)
        add     sp,3
        sw      r0,-24(fp)
        lw      r0,-24(fp)
        push    r0
        lc      r0,5
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L13
        lc      r0,0
        sw      r0,-3(fp)
L13:
        la      r0,_S1
        sw      r0,-27(fp)
        lw      r0,-27(fp)
        push    r0
        lc      r0,4
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-30(fp)
        lw      r0,-30(fp)
        push    r0
        lc      r0,2
        mov     r1,r0
        pop     r0
        sub     r0,r1
        sw      r0,-30(fp)
        lw      r0,-30(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,67
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L15
        lc      r0,0
        sw      r0,-3(fp)
L15:
        lw      r0,-3(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L17
        la      r0,_S2
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L7
L17:
        lc      r0,0
        bra     L7
L7:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

__cc24_div:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r1,12(fp)
        lc      r2,0
__cc24_div_lp:
        cls     r0,r1
        brt     __cc24_div_dn
        sub     r0,r1
        add     r2,1
        bra     __cc24_div_lp
__cc24_div_dn:
        mov     r0,r2
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   72,101,108,108,111,0
_S1:
        .byte   65,66,67,68,69,0
_S2:
        .byte   68,55,79,75,10,0
