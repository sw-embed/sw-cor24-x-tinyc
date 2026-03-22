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
        add     sp,-18
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,5
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        push    r0
        add     r0,1
        sw      r0,-6(fp)
        pop     r0
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
        brt     L6
        lc      r0,0
        sw      r0,-3(fp)
L6:
        lw      r0,-6(fp)
        push    r0
        add     r0,-1
        sw      r0,-6(fp)
        pop     r0
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
        brt     L8
        lc      r0,0
        sw      r0,-3(fp)
L8:
        lw      r0,-6(fp)
        push    r0
        add     r0,1
        sw      r0,-6(fp)
        pop     r0
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        push    r0
        lc      r0,5
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L11
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
        brf     L11
        lc      r0,0
        bra     L12
L11:
        lc      r0,1
L12:
        ceq     r0,z
        brt     L10
        lc      r0,0
        sw      r0,-3(fp)
L10:
        lw      r0,-6(fp)
        add     r0,1
        sw      r0,-6(fp)
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        push    r0
        lc      r0,7
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L15
        lw      r0,-6(fp)
        push    r0
        lc      r0,7
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L15
        lc      r0,0
        bra     L16
L15:
        lc      r0,1
L16:
        ceq     r0,z
        brt     L14
        lc      r0,0
        sw      r0,-3(fp)
L14:
        lc      r0,0
        sw      r0,-15(fp)
        lc      r0,0
        sw      r0,-18(fp)
L17:
        lw      r0,-18(fp)
        push    r0
        lc      r0,5
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L19
        lw      r0,-15(fp)
        push    r0
        lw      r0,-18(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-15(fp)
L18:
        lw      r0,-18(fp)
        push    r0
        add     r0,1
        sw      r0,-18(fp)
        pop     r0
        bra     L17
L19:
        lw      r0,-15(fp)
        push    r0
        lc      r0,10
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L21
        lc      r0,0
        sw      r0,-3(fp)
L21:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L23
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L4
L23:
        lc      r0,0
        bra     L4
L4:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   68,49,52,79,75,10,0
