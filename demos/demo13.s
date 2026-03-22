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
L5:
        lc      r0,1
        ceq     r0,z
        brt     L6
        lw      r0,-6(fp)
        push    r0
        lc      r0,5
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L8
        bra     L6
L8:
        lw      r0,-6(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-6(fp)
        bra     L5
L6:
        lw      r0,-6(fp)
        push    r0
        lc      r0,5
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
        lc      r0,0
        sw      r0,-9(fp)
        lc      r0,0
        sw      r0,-12(fp)
L11:
        lw      r0,-12(fp)
        push    r0
        lc      r0,10
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L12
        lw      r0,-12(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        push    r0
        lc      r0,2
        mov     r1,r0
        pop     r0
        push    r1
        push    r0
        la      r0,__cc24_mod
        jal     r1,(r0)
        add     sp,6
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L14
        bra     L11
L14:
        lw      r0,-9(fp)
        push    r0
        lw      r0,-12(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-9(fp)
        bra     L11
L12:
        lw      r0,-9(fp)
        push    r0
        lc      r0,25
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
        lc      r0,0
        sw      r0,-15(fp)
        lc      r0,0
        sw      r0,-15(fp)
L17:
        lw      r0,-15(fp)
        push    r0
        lc      r0,100
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L19
        lw      r0,-15(fp)
        push    r0
        lc      r0,3
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L21
        bra     L19
L21:
L18:
        lw      r0,-15(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-15(fp)
        bra     L17
L19:
        lw      r0,-15(fp)
        push    r0
        lc      r0,3
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L23
        lc      r0,0
        sw      r0,-3(fp)
L23:
        lc      r0,0
        sw      r0,-18(fp)
        lc      r0,1
        sw      r0,-21(fp)
L24:
        lw      r0,-21(fp)
        push    r0
        lc      r0,5
        mov     r1,r0
        pop     r0
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L26
        lw      r0,-21(fp)
        push    r0
        lc      r0,2
        mov     r1,r0
        pop     r0
        push    r1
        push    r0
        la      r0,__cc24_mod
        jal     r1,(r0)
        add     sp,6
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L28
        bra     L25
L28:
        lw      r0,-18(fp)
        push    r0
        lw      r0,-21(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-18(fp)
L25:
        lw      r0,-21(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-21(fp)
        bra     L24
L26:
        lw      r0,-18(fp)
        push    r0
        lc      r0,9
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L30
        lc      r0,0
        sw      r0,-3(fp)
L30:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L32
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L4
L32:
        lc      r0,0
        bra     L4
L4:
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
        .byte   68,49,51,79,75,10,0
