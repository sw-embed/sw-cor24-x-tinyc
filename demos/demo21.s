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
        brf     L3
        la      r2,L2
        jmp     (r2)
L3:
        la      r2,L1
        jmp     (r2)
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
L5:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L7
        la      r2,L6
        jmp     (r2)
L7:
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
        la      r2,L5
        jmp     (r2)
L6:
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
        add     sp,-12
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,10
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,5
        add     r0,r1
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,15
        ceq     r0,r1
        brf     L11
        la      r2,L10
        jmp     (r2)
L11:
        lc      r0,0
        sw      r0,-3(fp)
L10:
        lw      r0,-6(fp)
        lc      r1,3
        sub     r0,r1
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,12
        ceq     r0,r1
        brf     L14
        la      r2,L13
        jmp     (r2)
L14:
        lc      r0,0
        sw      r0,-3(fp)
L13:
        lw      r0,-6(fp)
        lc      r1,2
        mul     r0,r1
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,24
        ceq     r0,r1
        brf     L17
        la      r2,L16
        jmp     (r2)
L17:
        lc      r0,0
        sw      r0,-3(fp)
L16:
        lw      r0,-6(fp)
        lc      r1,4
        push    r1
        push    r0
        la      r0,__tc24r_div
        jal     r1,(r0)
        add     sp,6
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,6
        ceq     r0,r1
        brf     L20
        la      r2,L19
        jmp     (r2)
L20:
        lc      r0,0
        sw      r0,-3(fp)
L19:
        lw      r0,-6(fp)
        lc      r1,4
        push    r1
        push    r0
        la      r0,__tc24r_mod
        jal     r1,(r0)
        add     sp,6
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,2
        ceq     r0,r1
        brf     L23
        la      r2,L22
        jmp     (r2)
L23:
        lc      r0,0
        sw      r0,-3(fp)
L22:
        la      r0,255
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        lc      r1,15
        and     r0,r1
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        lc      r1,15
        ceq     r0,r1
        brf     L26
        la      r2,L25
        jmp     (r2)
L26:
        lc      r0,0
        sw      r0,-3(fp)
L25:
        lw      r0,-9(fp)
        lc      r1,48
        or      r0,r1
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        lc      r1,63
        ceq     r0,r1
        brf     L29
        la      r2,L28
        jmp     (r2)
L29:
        lc      r0,0
        sw      r0,-3(fp)
L28:
        lw      r0,-9(fp)
        la      r1,255
        xor     r0,r1
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        la      r1,192
        ceq     r0,r1
        brf     L32
        la      r2,L31
        jmp     (r2)
L32:
        lc      r0,0
        sw      r0,-3(fp)
L31:
        lc      r0,1
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        lc      r1,3
        shl     r0,r1
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        lc      r1,8
        ceq     r0,r1
        brf     L35
        la      r2,L34
        jmp     (r2)
L35:
        lc      r0,0
        sw      r0,-3(fp)
L34:
        lw      r0,-12(fp)
        lc      r1,1
        srl     r0,r1
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        lc      r1,4
        ceq     r0,r1
        brf     L38
        la      r2,L37
        jmp     (r2)
L38:
        lc      r0,0
        sw      r0,-3(fp)
L37:
        lw      r0,-3(fp)
        ceq     r0,z
        brf     L41
        la      r2,L40
        jmp     (r2)
L41:
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        la      r2,L8
        jmp     (r2)
L40:
        lc      r0,0
        la      r2,L8
        jmp     (r2)
L8:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

__tc24r_div:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r1,12(fp)
        lc      r2,0
__tc24r_div_lp:
        cls     r0,r1
        brt     __tc24r_div_dn
        sub     r0,r1
        add     r2,1
        bra     __tc24r_div_lp
__tc24r_div_dn:
        mov     r0,r2
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
__tc24r_mod:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r1,12(fp)
__tc24r_mod_lp:
        cls     r0,r1
        brt     __tc24r_mod_dn
        sub     r0,r1
        bra     __tc24r_mod_lp
__tc24r_mod_dn:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   68,50,49,79,75,10,0
