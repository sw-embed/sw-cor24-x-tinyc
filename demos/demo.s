        .text

        .globl  _start
_start:
        la      r0,_main
        jal     r1,(r0)
_halt:
        bra     _halt

        .globl  _add
_add:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r1,12(fp)
        add     r0,r1
        bra     L0
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _fib
_fib:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lc      r1,2
        cls     r0,r1
        brf     L3
        lc      r0,1
        bra     L1
L3:
        lw      r0,9(fp)
        lc      r1,1
        sub     r0,r1
        push    r0
        la      r0,_fib
        jal     r1,(r0)
        add     sp,3
        push    r0
        lw      r0,9(fp)
        lc      r1,2
        sub     r0,r1
        push    r0
        la      r0,_fib
        jal     r1,(r0)
        add     sp,3
        mov     r1,r0
        pop     r0
        add     r0,r1
        bra     L1
L1:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _bitops
_bitops:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-15
        lw      r0,9(fp)
        lc      r1,15
        and     r0,r1
        sw      r0,-3(fp)
        lw      r0,-3(fp)
        lc      r1,32
        or      r0,r1
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,7
        xor     r0,r1
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        lc      r1,1
        shl     r0,r1
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        lc      r1,1
        sra     r0,r1
        sw      r0,-15(fp)
        lw      r0,-15(fp)
        bra     L4
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
        add     sp,-36
        lc      r0,42
        sw      r0,-3(fp)
        la      r0,1000
        sw      r0,-6(fp)
        lc      r0,25
        push    r0
        lc      r0,17
        push    r0
        la      r0,_add
        jal     r1,(r0)
        add     sp,6
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        lw      r1,-3(fp)
        ceq     r0,r1
        brf     L6
        lc      r0,1
        la      r1,_counter
        sw      r0,0(r1)
        bra     L7
L6:
        lc      r0,0
        la      r1,_counter
        sw      r0,0(r1)
L7:
        lc      r0,0
        sw      r0,-12(fp)
        lc      r0,1
        sw      r0,-15(fp)
L8:
        lw      r0,-15(fp)
        lc      r1,5
        cls     r1,r0
        brt     L9
        lw      r0,-12(fp)
        lw      r1,-15(fp)
        add     r0,r1
        sw      r0,-12(fp)
        lw      r0,-15(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-15(fp)
        bra     L8
L9:
        lc      r0,6
        sw      r0,-18(fp)
L10:
        lw      r0,-18(fp)
        lc      r1,10
        cls     r1,r0
        brt     L12
        lw      r0,-12(fp)
        lw      r1,-18(fp)
        add     r0,r1
        sw      r0,-12(fp)
L11:
        lw      r0,-18(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-18(fp)
        bra     L10
L12:
        lc      r0,6
        push    r0
        la      r0,_fib
        jal     r1,(r0)
        add     sp,3
        sw      r0,-21(fp)
        lw      r0,-6(fp)
        push    r0
        la      r0,_bitops
        jal     r1,(r0)
        add     sp,3
        sw      r0,-24(fp)
        lc      r0,1
        push    r0
        lc      r0,0
        pop     r1
        sub     r0,r1
        sw      r0,-27(fp)
        lc      r0,0
        lc      r1,-1
        xor     r0,r1
        sw      r0,-30(fp)
        lc      r0,0
        ceq     r0,z
        mov     r0,c
        sw      r0,-33(fp)
        lc      r0,1
        sw      r0,-36(fp)
        lw      r0,-9(fp)
        lc      r1,42
        ceq     r0,r1
        brt     L14
        lc      r0,0
        sw      r0,-36(fp)
L14:
        lw      r0,-12(fp)
        lc      r1,55
        ceq     r0,r1
        brt     L16
        lc      r0,0
        sw      r0,-36(fp)
L16:
        lw      r0,-21(fp)
        lc      r1,13
        ceq     r0,r1
        brt     L18
        lc      r0,0
        sw      r0,-36(fp)
L18:
        la      r1,_counter
        lw      r0,0(r1)
        lc      r1,1
        ceq     r0,r1
        brt     L20
        lc      r0,0
        sw      r0,-36(fp)
L20:
        lw      r0,-27(fp)
        push    r0
        lc      r0,1
        push    r0
        lc      r0,0
        pop     r1
        sub     r0,r1
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        brt     L22
        lc      r0,0
        sw      r0,-36(fp)
L22:
        lw      r0,-33(fp)
        lc      r1,1
        ceq     r0,r1
        brt     L24
        lc      r0,0
        sw      r0,-36(fp)
L24:
        lw      r0,-36(fp)
        lc      r1,1
        ceq     r0,r1
        brf     L26
        lc      r0,42
        bra     L5
L26:
        lc      r0,0
        bra     L5
L5:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_counter:
        .word   0
