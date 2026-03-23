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
        brt     L7
        lc      r0,10
        sw      r0,-6(fp)
        bra     L8
L7:
        lc      r0,20
        sw      r0,-6(fp)
L8:
        lw      r0,-6(fp)
        lc      r1,10
        ceq     r0,r1
        brt     L10
        lc      r0,0
        sw      r0,-3(fp)
L10:
        lc      r0,0
        sw      r0,-9(fp)
L11:
        lw      r0,-9(fp)
        lc      r1,5
        cls     r0,r1
        brf     L12
        lw      r0,-9(fp)
        push    r0
        add     r0,1
        sw      r0,-9(fp)
        pop     r0
        bra     L11
L12:
        lw      r0,-9(fp)
        lc      r1,5
        ceq     r0,r1
        brt     L14
        lc      r0,0
        sw      r0,-3(fp)
L14:
        lc      r0,0
        sw      r0,-12(fp)
        lc      r0,0
        sw      r0,-15(fp)
L15:
        lw      r0,-15(fp)
        lc      r1,10
        cls     r0,r1
        brf     L17
        lw      r0,-12(fp)
        lw      r1,-15(fp)
        add     r0,r1
        sw      r0,-12(fp)
L16:
        lw      r0,-15(fp)
        push    r0
        add     r0,1
        sw      r0,-15(fp)
        pop     r0
        bra     L15
L17:
        lw      r0,-12(fp)
        lc      r1,45
        ceq     r0,r1
        brt     L19
        lc      r0,0
        sw      r0,-3(fp)
L19:
        lc      r0,0
        sw      r0,-18(fp)
        lc      r0,1
        ceq     r0,z
        brt     L21
        lc      r0,0
        ceq     r0,z
        brt     L22
        lc      r0,1
        sw      r0,-18(fp)
        bra     L23
L22:
        lc      r0,2
        sw      r0,-18(fp)
L23:
L21:
        lw      r0,-18(fp)
        lc      r1,2
        ceq     r0,r1
        brt     L25
        lc      r0,0
        sw      r0,-3(fp)
L25:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L27
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L6
L27:
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
        .byte   68,50,50,79,75,10,0
