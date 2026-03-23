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
        brt     L2
        bra     L1
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
        lc      r1,1
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
        brt     L7
        lw      r0,9(fp)
        bra     L8
L7:
        lw      r0,9(fp)
        push    r0
        lc      r0,0
        pop     r1
        sub     r0,r1
L8:
        bra     L6
L6:
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
        brt     L10
        lw      r0,9(fp)
        bra     L11
L10:
        lw      r0,12(fp)
L11:
        bra     L9
L9:
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
        brt     L15
        lc      r0,42
        bra     L16
L15:
        lc      r0,0
L16:
        lc      r1,42
        ceq     r0,r1
        brt     L14
        lc      r0,0
        sw      r0,-3(fp)
L14:
        lc      r0,0
        ceq     r0,z
        brt     L19
        lc      r0,42
        bra     L20
L19:
        lc      r0,99
L20:
        lc      r1,99
        ceq     r0,r1
        brt     L18
        lc      r0,0
        sw      r0,-3(fp)
L18:
        lc      r0,3
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,5
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brt     L21
        lc      r0,1
        bra     L22
L21:
        lw      r0,-6(fp)
        lc      r1,2
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brt     L23
        lc      r0,2
        bra     L24
L23:
        lc      r0,3
L24:
L22:
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        lc      r1,2
        ceq     r0,r1
        brt     L26
        lc      r0,0
        sw      r0,-3(fp)
L26:
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
        brt     L28
        lc      r0,0
        sw      r0,-3(fp)
L28:
        lc      r0,3
        push    r0
        la      r0,_abs
        jal     r1,(r0)
        add     sp,3
        lc      r1,3
        ceq     r0,r1
        brt     L30
        lc      r0,0
        sw      r0,-3(fp)
L30:
        lc      r0,20
        push    r0
        lc      r0,10
        push    r0
        la      r0,_max
        jal     r1,(r0)
        add     sp,6
        lc      r1,20
        ceq     r0,r1
        brt     L32
        lc      r0,0
        sw      r0,-3(fp)
L32:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L34
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L12
L34:
        lc      r0,0
        bra     L12
L12:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   68,49,53,79,75,10,0
