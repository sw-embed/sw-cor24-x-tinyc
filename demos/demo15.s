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

        .globl  _abs
_abs:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lc      r1,0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L11
        la      r2,L9
        jmp     (r2)
L11:
        lw      r0,9(fp)
        la      r2,L10
        jmp     (r2)
L9:
        lw      r0,9(fp)
        push    r0
        lc      r0,0
        pop     r1
        sub     r0,r1
L10:
        la      r2,L8
        jmp     (r2)
L8:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _max
_max:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r1,12(fp)
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brf     L15
        la      r2,L13
        jmp     (r2)
L15:
        lw      r0,9(fp)
        la      r2,L14
        jmp     (r2)
L13:
        lw      r0,12(fp)
L14:
        la      r2,L12
        jmp     (r2)
L12:
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
        add     sp,-9
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,1
        ceq     r0,z
        brf     L21
        la      r2,L19
        jmp     (r2)
L21:
        lc      r0,42
        la      r2,L20
        jmp     (r2)
L19:
        lc      r0,0
L20:
        lc      r1,42
        ceq     r0,r1
        brf     L22
        la      r2,L18
        jmp     (r2)
L22:
        lc      r0,0
        sw      r0,-3(fp)
L18:
        lc      r0,0
        ceq     r0,z
        brf     L27
        la      r2,L25
        jmp     (r2)
L27:
        lc      r0,42
        la      r2,L26
        jmp     (r2)
L25:
        lc      r0,99
L26:
        lc      r1,99
        ceq     r0,r1
        brf     L28
        la      r2,L24
        jmp     (r2)
L28:
        lc      r0,0
        sw      r0,-3(fp)
L24:
        lc      r0,3
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,5
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brf     L31
        la      r2,L29
        jmp     (r2)
L31:
        lc      r0,1
        la      r2,L30
        jmp     (r2)
L29:
        lw      r0,-6(fp)
        lc      r1,2
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brf     L34
        la      r2,L32
        jmp     (r2)
L34:
        lc      r0,2
        la      r2,L33
        jmp     (r2)
L32:
        lc      r0,3
L33:
L30:
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        lc      r1,2
        ceq     r0,r1
        brf     L37
        la      r2,L36
        jmp     (r2)
L37:
        lc      r0,0
        sw      r0,-3(fp)
L36:
        lc      r0,7
        push    r0
        lc      r0,0
        pop     r1
        sub     r0,r1
        push    r0
        la      r0,_abs
        jal     r1,(r0)
        add     sp,3
        lc      r1,7
        ceq     r0,r1
        brf     L40
        la      r2,L39
        jmp     (r2)
L40:
        lc      r0,0
        sw      r0,-3(fp)
L39:
        lc      r0,3
        push    r0
        la      r0,_abs
        jal     r1,(r0)
        add     sp,3
        lc      r1,3
        ceq     r0,r1
        brf     L43
        la      r2,L42
        jmp     (r2)
L43:
        lc      r0,0
        sw      r0,-3(fp)
L42:
        lc      r0,20
        push    r0
        lc      r0,10
        push    r0
        la      r0,_max
        jal     r1,(r0)
        add     sp,6
        lc      r1,20
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
        la      r2,L16
        jmp     (r2)
L48:
        lc      r0,0
        la      r2,L16
        jmp     (r2)
L16:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   68,49,53,79,75,10,0
