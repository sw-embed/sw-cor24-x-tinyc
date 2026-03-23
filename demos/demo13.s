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
        add     sp,-21
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,0
        sw      r0,-6(fp)
L9:
        lc      r0,1
        ceq     r0,z
        brf     L11
        la      r2,L10
        jmp     (r2)
L11:
        lw      r0,-6(fp)
        lc      r1,5
        ceq     r0,r1
        brt     L14
        la      r2,L13
        jmp     (r2)
L14:
        la      r2,L10
        jmp     (r2)
L13:
        lw      r0,-6(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-6(fp)
        la      r2,L9
        jmp     (r2)
L10:
        lw      r0,-6(fp)
        lc      r1,5
        ceq     r0,r1
        brf     L17
        la      r2,L16
        jmp     (r2)
L17:
        lc      r0,0
        sw      r0,-3(fp)
L16:
        lc      r0,0
        sw      r0,-9(fp)
        lc      r0,0
        sw      r0,-12(fp)
L18:
        lw      r0,-12(fp)
        lc      r1,10
        cls     r0,r1
        brt     L20
        la      r2,L19
        jmp     (r2)
L20:
        lw      r0,-12(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        lc      r1,2
        push    r1
        push    r0
        la      r0,__tc24r_mod
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brt     L23
        la      r2,L22
        jmp     (r2)
L23:
        la      r2,L18
        jmp     (r2)
L22:
        lw      r0,-9(fp)
        lw      r1,-12(fp)
        add     r0,r1
        sw      r0,-9(fp)
        la      r2,L18
        jmp     (r2)
L19:
        lw      r0,-9(fp)
        lc      r1,25
        ceq     r0,r1
        brf     L26
        la      r2,L25
        jmp     (r2)
L26:
        lc      r0,0
        sw      r0,-3(fp)
L25:
        lc      r0,0
        sw      r0,-15(fp)
        lc      r0,0
        sw      r0,-15(fp)
L27:
        lw      r0,-15(fp)
        lc      r1,100
        cls     r0,r1
        brt     L30
        la      r2,L29
        jmp     (r2)
L30:
        lw      r0,-15(fp)
        lc      r1,3
        ceq     r0,r1
        brt     L33
        la      r2,L32
        jmp     (r2)
L33:
        la      r2,L29
        jmp     (r2)
L32:
L28:
        lw      r0,-15(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-15(fp)
        la      r2,L27
        jmp     (r2)
L29:
        lw      r0,-15(fp)
        lc      r1,3
        ceq     r0,r1
        brf     L36
        la      r2,L35
        jmp     (r2)
L36:
        lc      r0,0
        sw      r0,-3(fp)
L35:
        lc      r0,0
        sw      r0,-18(fp)
        lc      r0,1
        sw      r0,-21(fp)
L37:
        lw      r0,-21(fp)
        lc      r1,5
        cls     r1,r0
        brf     L40
        la      r2,L39
        jmp     (r2)
L40:
        lw      r0,-21(fp)
        lc      r1,2
        push    r1
        push    r0
        la      r0,__tc24r_mod
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brt     L43
        la      r2,L42
        jmp     (r2)
L43:
        la      r2,L38
        jmp     (r2)
L42:
        lw      r0,-18(fp)
        lw      r1,-21(fp)
        add     r0,r1
        sw      r0,-18(fp)
L38:
        lw      r0,-21(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-21(fp)
        la      r2,L37
        jmp     (r2)
L39:
        lw      r0,-18(fp)
        lc      r1,9
        ceq     r0,r1
        brf     L46
        la      r2,L45
        jmp     (r2)
L46:
        lc      r0,0
        sw      r0,-3(fp)
L45:
        lw      r0,-3(fp)
        ceq     r0,z
        brf     L49
        la      r2,L48
        jmp     (r2)
L49:
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        la      r2,L8
        jmp     (r2)
L48:
        lc      r0,0
        la      r2,L8
        jmp     (r2)
L8:
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
        .byte   68,49,51,79,75,10,0
