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
        lc      r0,10
        push    r0
        lc      r0,-9
        add     r0,fp
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,20
        push    r0
        lc      r0,-6
        add     r0,fp
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,-9
        add     r0,fp
        lw      r0,0(r0)
        push    r0
        lc      r0,-6
        add     r0,fp
        lw      r0,0(r0)
        mov     r1,r0
        pop     r0
        add     r0,r1
        lc      r1,30
        ceq     r0,r1
        brf     L11
        la      r2,L10
        jmp     (r2)
L11:
        lc      r0,0
        sw      r0,-3(fp)
L10:
        lc      r0,3
        push    r0
        lc      r0,-15
        add     r0,fp
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,4
        push    r0
        lc      r0,-12
        add     r0,fp
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,-15
        add     r0,fp
        lw      r0,0(r0)
        lc      r1,3
        ceq     r0,r1
        brf     L14
        la      r2,L13
        jmp     (r2)
L14:
        lc      r0,0
        sw      r0,-3(fp)
L13:
        lc      r0,-12
        add     r0,fp
        lw      r0,0(r0)
        lc      r1,4
        ceq     r0,r1
        brf     L17
        la      r2,L16
        jmp     (r2)
L17:
        lc      r0,0
        sw      r0,-3(fp)
L16:
        lc      r0,6
        lc      r1,6
        ceq     r0,r1
        brf     L20
        la      r2,L19
        jmp     (r2)
L20:
        lc      r0,0
        sw      r0,-3(fp)
L19:
        lc      r0,-15
        add     r0,fp
        lw      r0,0(r0)
        push    r0
        lc      r0,-12
        add     r0,fp
        lw      r0,0(r0)
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-18(fp)
        lw      r0,-18(fp)
        lc      r1,7
        ceq     r0,r1
        brf     L23
        la      r2,L22
        jmp     (r2)
L23:
        lc      r0,0
        sw      r0,-3(fp)
L22:
        lc      r0,-15
        add     r0,fp
        lw      r0,0(r0)
        lc      r1,10
        add     r0,r1
        push    r0
        lc      r0,-15
        add     r0,fp
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,-15
        add     r0,fp
        lw      r0,0(r0)
        lc      r1,13
        ceq     r0,r1
        brf     L26
        la      r2,L25
        jmp     (r2)
L26:
        lc      r0,0
        sw      r0,-3(fp)
L25:
        lw      r0,-3(fp)
        ceq     r0,z
        brf     L29
        la      r2,L28
        jmp     (r2)
L29:
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        la      r2,L8
        jmp     (r2)
L28:
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
        .byte   68,50,53,79,75,10,0
