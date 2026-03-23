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

        .globl  _add
_add:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r1,12(fp)
        add     r0,r1
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
        add     sp,-3
        lc      r0,1
        sw      r0,-3(fp)
        la      r1,_g
        lw      r0,0(r1)
        lc      r1,10
        ceq     r0,r1
        brf     L12
        la      r2,L11
        jmp     (r2)
L12:
        lc      r0,0
        sw      r0,-3(fp)
L11:
        lc      r0,12
        push    r0
        lc      r0,20
        push    r0
        la      r0,_add
        jal     r1,(r0)
        add     sp,6
        lc      r1,32
        ceq     r0,r1
        brf     L15
        la      r2,L14
        jmp     (r2)
L15:
        lc      r0,0
        sw      r0,-3(fp)
L14:
        lc      r0,42
        la      r1,_g
        sw      r0,0(r1)
        la      r1,_g
        lw      r0,0(r1)
        lc      r1,42
        ceq     r0,r1
        brf     L18
        la      r2,L17
        jmp     (r2)
L18:
        lc      r0,0
        sw      r0,-3(fp)
L17:
        lw      r0,-3(fp)
        ceq     r0,z
        brf     L21
        la      r2,L20
        jmp     (r2)
L21:
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        la      r2,L9
        jmp     (r2)
L20:
        lc      r0,0
        la      r2,L9
        jmp     (r2)
L9:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_g:
        .word   10
_S0:
        .byte   68,49,57,79,75,10,0
