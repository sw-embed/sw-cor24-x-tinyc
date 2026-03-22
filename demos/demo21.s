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
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
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
        lc      r0,10
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        push    r0
        lc      r0,5
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        push    r0
        lc      r0,15
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L8
        lc      r0,0
        sw      r0,-3(fp)
L8:
        lw      r0,-6(fp)
        push    r0
        lc      r0,3
        mov     r1,r0
        pop     r0
        sub     r0,r1
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        push    r0
        lc      r0,12
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L10
        lc      r0,0
        sw      r0,-3(fp)
L10:
        lw      r0,-6(fp)
        push    r0
        lc      r0,2
        mov     r1,r0
        pop     r0
        mul     r0,r1
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        push    r0
        lc      r0,24
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L12
        lc      r0,0
        sw      r0,-3(fp)
L12:
        lw      r0,-6(fp)
        push    r0
        lc      r0,4
        mov     r1,r0
        pop     r0
        push    r1
        push    r0
        la      r0,__cc24_div
        jal     r1,(r0)
        add     sp,6
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        push    r0
        lc      r0,6
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L14
        lc      r0,0
        sw      r0,-3(fp)
L14:
        lw      r0,-6(fp)
        push    r0
        lc      r0,4
        mov     r1,r0
        pop     r0
        push    r1
        push    r0
        la      r0,__cc24_mod
        jal     r1,(r0)
        add     sp,6
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        push    r0
        lc      r0,2
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L16
        lc      r0,0
        sw      r0,-3(fp)
L16:
        la      r0,255
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        push    r0
        lc      r0,15
        mov     r1,r0
        pop     r0
        and     r0,r1
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        push    r0
        lc      r0,15
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L18
        lc      r0,0
        sw      r0,-3(fp)
L18:
        lw      r0,-9(fp)
        push    r0
        lc      r0,48
        mov     r1,r0
        pop     r0
        or      r0,r1
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        push    r0
        lc      r0,63
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L20
        lc      r0,0
        sw      r0,-3(fp)
L20:
        lw      r0,-9(fp)
        push    r0
        la      r0,255
        mov     r1,r0
        pop     r0
        xor     r0,r1
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        push    r0
        la      r0,192
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L22
        lc      r0,0
        sw      r0,-3(fp)
L22:
        lc      r0,1
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        push    r0
        lc      r0,3
        mov     r1,r0
        pop     r0
        shl     r0,r1
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        push    r0
        lc      r0,8
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L24
        lc      r0,0
        sw      r0,-3(fp)
L24:
        lw      r0,-12(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        srl     r0,r1
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        push    r0
        lc      r0,4
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L26
        lc      r0,0
        sw      r0,-3(fp)
L26:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L28
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L6
L28:
        lc      r0,0
        bra     L6
L6:
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
__cc24_mod:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r1,12(fp)
__cc24_mod_lp:
        cls     r0,r1
        brt     __cc24_mod_dn
        sub     r0,r1
        bra     __cc24_mod_lp
__cc24_mod_dn:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   68,50,49,79,75,10,0
