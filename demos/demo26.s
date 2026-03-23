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

        .globl  _classify
_classify:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        push    r0
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,0
        ceq     r0,r1
        brt     L8
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,1
        ceq     r0,r1
        brt     L9
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,2
        ceq     r0,r1
        brt     L10
        pop     r0
        bra     L11
L8:
        lc      r0,10
        bra     L6
L9:
        lc      r0,20
        bra     L6
L10:
        lc      r0,30
        bra     L6
L11:
        lc      r0,99
        bra     L6
L7:
L6:
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
        lc      r0,0
        push    r0
        la      r0,_classify
        jal     r1,(r0)
        add     sp,3
        push    r0
        lc      r0,10
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
        lc      r0,1
        push    r0
        la      r0,_classify
        jal     r1,(r0)
        add     sp,3
        push    r0
        lc      r0,20
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L16
        lc      r0,0
        sw      r0,-3(fp)
L16:
        lc      r0,2
        push    r0
        la      r0,_classify
        jal     r1,(r0)
        add     sp,3
        push    r0
        lc      r0,30
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L18
        lc      r0,0
        sw      r0,-3(fp)
L18:
        lc      r0,3
        push    r0
        la      r0,_classify
        jal     r1,(r0)
        add     sp,3
        push    r0
        lc      r0,99
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L20
        lc      r0,0
        sw      r0,-3(fp)
L20:
        lc      r0,0
        sw      r0,-6(fp)
        lc      r0,2
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        push    r0
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,1
        ceq     r0,r1
        brt     L22
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,2
        ceq     r0,r1
        brt     L23
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,3
        ceq     r0,r1
        brt     L24
        pop     r0
        bra     L21
L22:
        lc      r0,11
        sw      r0,-6(fp)
        bra     L21
L23:
        lc      r0,22
        sw      r0,-6(fp)
        bra     L21
L24:
        lc      r0,33
        sw      r0,-6(fp)
        bra     L21
L21:
        lw      r0,-6(fp)
        push    r0
        lc      r0,22
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L27
        lc      r0,0
        sw      r0,-3(fp)
L27:
        lc      r0,0
        sw      r0,-12(fp)
        lc      r0,1
        push    r0
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,1
        ceq     r0,r1
        brt     L29
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,2
        ceq     r0,r1
        brt     L30
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,3
        ceq     r0,r1
        brt     L31
        pop     r0
        bra     L32
L29:
        lw      r0,-12(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-12(fp)
L30:
        lw      r0,-12(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-12(fp)
L31:
        lw      r0,-12(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-12(fp)
        bra     L28
L32:
        lc      r0,100
        sw      r0,-12(fp)
L28:
        lw      r0,-12(fp)
        push    r0
        lc      r0,3
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L34
        lc      r0,0
        sw      r0,-3(fp)
L34:
        lc      r0,0
        sw      r0,-15(fp)
        lc      r0,0
        sw      r0,-18(fp)
L35:
        lw      r0,-18(fp)
        push    r0
        lc      r0,4
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L36
        lw      r0,-18(fp)
        push    r0
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,0
        ceq     r0,r1
        brt     L38
        pop     r0
        push    r0
        mov     r1,r0
        lc      r0,1
        ceq     r0,r1
        brt     L39
        pop     r0
        bra     L40
L38:
        lw      r0,-15(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-15(fp)
        bra     L37
L39:
        lw      r0,-15(fp)
        push    r0
        lc      r0,10
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-15(fp)
        bra     L37
L40:
        lw      r0,-15(fp)
        push    r0
        lc      r0,100
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-15(fp)
        bra     L37
L37:
        lw      r0,-18(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-18(fp)
        bra     L35
L36:
        lw      r0,-15(fp)
        push    r0
        la      r0,211
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L42
        lc      r0,0
        sw      r0,-3(fp)
L42:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L44
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L12
L44:
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
        .byte   68,50,54,79,75,10,0
