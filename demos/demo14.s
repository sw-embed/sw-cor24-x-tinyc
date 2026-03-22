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
        brt     L8
        lc      r0,0
        sw      r0,-3(fp)
L8:
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
        brt     L10
        lc      r0,0
        sw      r0,-3(fp)
L10:
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
        brf     L13
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
        brf     L17
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
        brf     L17
        lc      r0,0
        bra     L18
L17:
        lc      r0,1
L18:
        ceq     r0,z
        brt     L16
        lc      r0,0
        sw      r0,-3(fp)
L16:
        lc      r0,0
        sw      r0,-15(fp)
        lc      r0,0
        sw      r0,-18(fp)
L19:
        lw      r0,-18(fp)
        push    r0
        lc      r0,5
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L21
        lw      r0,-15(fp)
        push    r0
        lw      r0,-18(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-15(fp)
L20:
        lw      r0,-18(fp)
        push    r0
        add     r0,1
        sw      r0,-18(fp)
        pop     r0
        bra     L19
L21:
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
        brt     L23
        lc      r0,0
        sw      r0,-3(fp)
L23:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L25
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L6
L25:
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
        .byte   68,49,52,79,75,10,0
