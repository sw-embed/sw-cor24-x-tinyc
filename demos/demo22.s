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
        lc      r0,1
        ceq     r0,z
        brf     L11
        la      r2,L9
        jmp     (r2)
L11:
        lc      r0,10
        sw      r0,-6(fp)
        la      r2,L10
        jmp     (r2)
L9:
        lc      r0,20
        sw      r0,-6(fp)
L10:
        lw      r0,-6(fp)
        lc      r1,10
        ceq     r0,r1
        brf     L14
        la      r2,L13
        jmp     (r2)
L14:
        lc      r0,0
        sw      r0,-3(fp)
L13:
        lc      r0,0
        sw      r0,-9(fp)
L15:
        lw      r0,-9(fp)
        lc      r1,5
        cls     r0,r1
        brt     L17
        la      r2,L16
        jmp     (r2)
L17:
        lw      r0,-9(fp)
        push    r0
        add     r0,1
        sw      r0,-9(fp)
        pop     r0
        la      r2,L15
        jmp     (r2)
L16:
        lw      r0,-9(fp)
        lc      r1,5
        ceq     r0,r1
        brf     L20
        la      r2,L19
        jmp     (r2)
L20:
        lc      r0,0
        sw      r0,-3(fp)
L19:
        lc      r0,0
        sw      r0,-12(fp)
        lc      r0,0
        sw      r0,-15(fp)
L21:
        lw      r0,-15(fp)
        lc      r1,10
        cls     r0,r1
        brt     L24
        la      r2,L23
        jmp     (r2)
L24:
        lw      r0,-12(fp)
        lw      r1,-15(fp)
        add     r0,r1
        sw      r0,-12(fp)
L22:
        lw      r0,-15(fp)
        push    r0
        add     r0,1
        sw      r0,-15(fp)
        pop     r0
        la      r2,L21
        jmp     (r2)
L23:
        lw      r0,-12(fp)
        lc      r1,45
        ceq     r0,r1
        brf     L27
        la      r2,L26
        jmp     (r2)
L27:
        lc      r0,0
        sw      r0,-3(fp)
L26:
        lc      r0,0
        sw      r0,-18(fp)
        lc      r0,1
        ceq     r0,z
        brf     L30
        la      r2,L29
        jmp     (r2)
L30:
        lc      r0,0
        ceq     r0,z
        brf     L33
        la      r2,L31
        jmp     (r2)
L33:
        lc      r0,1
        sw      r0,-18(fp)
        la      r2,L32
        jmp     (r2)
L31:
        lc      r0,2
        sw      r0,-18(fp)
L32:
L29:
        lw      r0,-18(fp)
        lc      r1,2
        ceq     r0,r1
        brf     L36
        la      r2,L35
        jmp     (r2)
L36:
        lc      r0,0
        sw      r0,-3(fp)
L35:
        lw      r0,-3(fp)
        ceq     r0,z
        brf     L39
        la      r2,L38
        jmp     (r2)
L39:
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        la      r2,L8
        jmp     (r2)
L38:
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
        .byte   68,50,50,79,75,10,0
