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

        .globl  _strlen
_strlen:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,0
        sw      r0,-3(fp)
L7:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L8
        lw      r0,-3(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-3(fp)
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        bra     L7
L8:
        lw      r0,-3(fp)
        bra     L6
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
        add     sp,-21
        lc      r0,1
        sw      r0,-3(fp)
        la      r0,255
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        push    r0
        la      r0,255
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L11
        lc      r0,0
        sw      r0,-3(fp)
L11:
        lc      r0,42
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        push    r0
        lc      r0,42
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L13
        lc      r0,0
        sw      r0,-3(fp)
L13:
        la      r0,_S0
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        push    r0
        la      r0,_strlen
        jal     r1,(r0)
        add     sp,3
        sw      r0,-15(fp)
        lw      r0,-15(fp)
        push    r0
        lc      r0,5
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L15
        lc      r0,0
        sw      r0,-3(fp)
L15:
        lw      r0,-12(fp)
        sw      r0,-18(fp)
        lw      r0,-18(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-18(fp)
        lw      r0,-18(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,101
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L17
        lc      r0,0
        sw      r0,-3(fp)
L17:
        lw      r0,-12(fp)
        push    r0
        lc      r0,4
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-21(fp)
        lw      r0,-21(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,111
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L19
        lc      r0,0
        sw      r0,-3(fp)
L19:
        lw      r0,-3(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L21
        la      r0,_S1
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
L21:
        lw      r0,-3(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L23
        lc      r0,42
        bra     L9
L23:
        lc      r0,0
        bra     L9
L9:
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
