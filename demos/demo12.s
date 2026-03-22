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
        add     sp,-12
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,0
        sw      r0,-6(fp)
L5:
        lc      r0,1
        sw      r0,-6(fp)
        lc      r0,0
        ceq     r0,z
        brf     L5
        lw      r0,-6(fp)
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L7
        lc      r0,0
        sw      r0,-3(fp)
L7:
        lc      r0,0
        sw      r0,-9(fp)
L8:
        lw      r0,-9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        push    r0
        lc      r0,5
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L8
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
        brt     L10
        lc      r0,0
        sw      r0,-3(fp)
L10:
        lc      r0,0
        sw      r0,-12(fp)
L11:
        lw      r0,-12(fp)
        push    r0
        lc      r0,100
        mov     r1,r0
        pop     r0
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brt     L12
        la      r0,999
        sw      r0,-12(fp)
        bra     L11
L12:
        lw      r0,-12(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L14
        lc      r0,0
        sw      r0,-3(fp)
L14:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L16
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L4
L16:
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
        .byte   68,49,50,79,75,10,0
