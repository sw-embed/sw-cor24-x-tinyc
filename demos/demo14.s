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
        lc      r1,6
        ceq     r0,r1
        brf     L11
        la      r2,L10
        jmp     (r2)
L11:
        lc      r0,0
        sw      r0,-3(fp)
L10:
        lw      r0,-6(fp)
        push    r0
        add     r0,-1
        sw      r0,-6(fp)
        pop     r0
        lw      r0,-6(fp)
        lc      r1,5
        ceq     r0,r1
        brf     L14
        la      r2,L13
        jmp     (r2)
L14:
        lc      r0,0
        sw      r0,-3(fp)
L13:
        lw      r0,-6(fp)
        push    r0
        add     r0,1
        sw      r0,-6(fp)
        pop     r0
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        lc      r1,5
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L19
        la      r2,L17
        jmp     (r2)
L19:
        lw      r0,-6(fp)
        lc      r1,6
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L20
        la      r2,L17
        jmp     (r2)
L20:
        lc      r0,0
        la      r2,L18
        jmp     (r2)
L17:
        lc      r0,1
L18:
        ceq     r0,z
        brf     L21
        la      r2,L16
        jmp     (r2)
L21:
        lc      r0,0
        sw      r0,-3(fp)
L16:
        lw      r0,-6(fp)
        add     r0,1
        sw      r0,-6(fp)
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        lc      r1,7
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L26
        la      r2,L24
        jmp     (r2)
L26:
        lw      r0,-6(fp)
        lc      r1,7
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L27
        la      r2,L24
        jmp     (r2)
L27:
        lc      r0,0
        la      r2,L25
        jmp     (r2)
L24:
        lc      r0,1
L25:
        ceq     r0,z
        brf     L28
        la      r2,L23
        jmp     (r2)
L28:
        lc      r0,0
        sw      r0,-3(fp)
L23:
        lc      r0,0
        sw      r0,-15(fp)
        lc      r0,0
        sw      r0,-18(fp)
L29:
        lw      r0,-18(fp)
        lc      r1,5
        cls     r0,r1
        brt     L32
        la      r2,L31
        jmp     (r2)
L32:
        lw      r0,-15(fp)
        lw      r1,-18(fp)
        add     r0,r1
        sw      r0,-15(fp)
L30:
        lw      r0,-18(fp)
        push    r0
        add     r0,1
        sw      r0,-18(fp)
        pop     r0
        la      r2,L29
        jmp     (r2)
L31:
        lw      r0,-15(fp)
        lc      r1,10
        ceq     r0,r1
        brf     L35
        la      r2,L34
        jmp     (r2)
L35:
        lc      r0,0
        sw      r0,-3(fp)
L34:
        lw      r0,-3(fp)
        ceq     r0,z
        brf     L38
        la      r2,L37
        jmp     (r2)
L38:
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        la      r2,L8
        jmp     (r2)
L37:
        lc      r0,0
        la      r2,L8
        jmp     (r2)
L8:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   68,49,52,79,75,10,0
