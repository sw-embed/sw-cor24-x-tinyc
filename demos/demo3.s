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

        .globl  _strlen
_strlen:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,0
        sw      r0,-3(fp)
L9:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L11
        la      r2,L10
        jmp     (r2)
L11:
        lw      r0,-3(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-3(fp)
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L9
        jmp     (r2)
L10:
        lw      r0,-3(fp)
        la      r2,L8
        jmp     (r2)
L8:
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
        la      r0,255
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        la      r1,255
        ceq     r0,r1
        brf     L15
        la      r2,L14
        jmp     (r2)
L15:
        lc      r0,0
        sw      r0,-3(fp)
L14:
        lc      r0,42
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        lc      r1,42
        ceq     r0,r1
        brf     L18
        la      r2,L17
        jmp     (r2)
L18:
        lc      r0,0
        sw      r0,-3(fp)
L17:
        la      r0,_S0
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        push    r0
        la      r0,_strlen
        jal     r1,(r0)
        add     sp,3
        sw      r0,-15(fp)
        lw      r0,-15(fp)
        lc      r1,5
        ceq     r0,r1
        brf     L21
        la      r2,L20
        jmp     (r2)
L21:
        lc      r0,0
        sw      r0,-3(fp)
L20:
        lw      r0,-12(fp)
        sw      r0,-18(fp)
        lw      r0,-18(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-18(fp)
        lw      r0,-18(fp)
        lbu     r0,0(r0)
        lc      r1,101
        ceq     r0,r1
        brf     L24
        la      r2,L23
        jmp     (r2)
L24:
        lc      r0,0
        sw      r0,-3(fp)
L23:
        lw      r0,-12(fp)
        lc      r1,4
        add     r0,r1
        sw      r0,-21(fp)
        lw      r0,-21(fp)
        lbu     r0,0(r0)
        lc      r1,111
        ceq     r0,r1
        brf     L27
        la      r2,L26
        jmp     (r2)
L27:
        lc      r0,0
        sw      r0,-3(fp)
L26:
        lw      r0,-3(fp)
        lc      r1,1
        ceq     r0,r1
        brt     L30
        la      r2,L29
        jmp     (r2)
L30:
        la      r0,_S1
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
L29:
        lw      r0,-3(fp)
        lc      r1,1
        ceq     r0,r1
        brt     L33
        la      r2,L32
        jmp     (r2)
L33:
        lc      r0,42
        la      r2,L12
        jmp     (r2)
L32:
        lc      r0,0
        la      r2,L12
        jmp     (r2)
L12:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   72,101,108,108,111,0
_S1:
        .byte   68,51,79,75,10,0
