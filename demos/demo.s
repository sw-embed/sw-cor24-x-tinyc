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
        la      r2,L0
        jmp     (r2)
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
        brt     L4
        la      r2,L3
        jmp     (r2)
L4:
        lc      r0,1
        la      r2,L1
        jmp     (r2)
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
        la      r2,L1
        jmp     (r2)
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
        srl     r0,r1
        sw      r0,-15(fp)
        lw      r0,-15(fp)
        la      r2,L5
        jmp     (r2)
L5:
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
        brt     L9
        la      r2,L7
        jmp     (r2)
L9:
        lc      r0,1
        la      r1,_counter
        sw      r0,0(r1)
        la      r2,L8
        jmp     (r2)
L7:
        lc      r0,0
        la      r1,_counter
        sw      r0,0(r1)
L8:
        lc      r0,0
        sw      r0,-12(fp)
        lc      r0,1
        sw      r0,-15(fp)
L10:
        lw      r0,-15(fp)
        lc      r1,5
        cls     r1,r0
        brf     L12
        la      r2,L11
        jmp     (r2)
L12:
        lw      r0,-12(fp)
        lw      r1,-15(fp)
        add     r0,r1
        sw      r0,-12(fp)
        lw      r0,-15(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-15(fp)
        la      r2,L10
        jmp     (r2)
L11:
        lc      r0,6
        sw      r0,-18(fp)
L13:
        lw      r0,-18(fp)
        lc      r1,10
        cls     r1,r0
        brf     L16
        la      r2,L15
        jmp     (r2)
L16:
        lw      r0,-12(fp)
        lw      r1,-18(fp)
        add     r0,r1
        sw      r0,-12(fp)
L14:
        lw      r0,-18(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-18(fp)
        la      r2,L13
        jmp     (r2)
L15:
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
        brf     L19
        la      r2,L18
        jmp     (r2)
L19:
        lc      r0,0
        sw      r0,-36(fp)
L18:
        lw      r0,-12(fp)
        lc      r1,55
        ceq     r0,r1
        brf     L22
        la      r2,L21
        jmp     (r2)
L22:
        lc      r0,0
        sw      r0,-36(fp)
L21:
        lw      r0,-21(fp)
        lc      r1,13
        ceq     r0,r1
        brf     L25
        la      r2,L24
        jmp     (r2)
L25:
        lc      r0,0
        sw      r0,-36(fp)
L24:
        la      r1,_counter
        lw      r0,0(r1)
        lc      r1,1
        ceq     r0,r1
        brf     L28
        la      r2,L27
        jmp     (r2)
L28:
        lc      r0,0
        sw      r0,-36(fp)
L27:
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
        brf     L31
        la      r2,L30
        jmp     (r2)
L31:
        lc      r0,0
        sw      r0,-36(fp)
L30:
        lw      r0,-33(fp)
        lc      r1,1
        ceq     r0,r1
        brf     L34
        la      r2,L33
        jmp     (r2)
L34:
        lc      r0,0
        sw      r0,-36(fp)
L33:
        lw      r0,-36(fp)
        lc      r1,1
        ceq     r0,r1
        brt     L37
        la      r2,L36
        jmp     (r2)
L37:
        lc      r0,42
        la      r2,L6
        jmp     (r2)
L36:
        lc      r0,0
        la      r2,L6
        jmp     (r2)
L6:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_counter:
        .word   0
