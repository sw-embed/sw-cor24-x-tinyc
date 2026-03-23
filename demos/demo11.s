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
        add     sp,-3
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,1
        ceq     r0,z
        brt     L9
        lc      r0,2
        ceq     r0,z
        brt     L9
        lc      r0,1
        bra     L10
L9:
        lc      r0,0
L10:
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L8
        lc      r0,0
        sw      r0,-3(fp)
L8:
        lc      r0,0
        ceq     r0,z
        brt     L13
        lc      r0,1
        ceq     r0,z
        brt     L13
        lc      r0,1
        bra     L14
L13:
        lc      r0,0
L14:
        ceq     r0,z
        brt     L12
        lc      r0,0
        sw      r0,-3(fp)
L12:
        lc      r0,1
        ceq     r0,z
        brf     L17
        lc      r0,0
        ceq     r0,z
        brf     L17
        lc      r0,0
        bra     L18
L17:
        lc      r0,1
L18:
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L16
        lc      r0,0
        sw      r0,-3(fp)
L16:
        lc      r0,0
        ceq     r0,z
        brf     L21
        lc      r0,0
        ceq     r0,z
        brf     L21
        lc      r0,0
        bra     L22
L21:
        lc      r0,1
L22:
        ceq     r0,z
        brt     L20
        lc      r0,0
        sw      r0,-3(fp)
L20:
        lc      r0,0
        ceq     r0,z
        brf     L25
        lc      r0,5
        ceq     r0,z
        brf     L25
        lc      r0,0
        bra     L26
L25:
        lc      r0,1
L26:
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L24
        lc      r0,0
        sw      r0,-3(fp)
L24:
        lc      r0,1
        ceq     r0,z
        brt     L31
        lc      r0,0
        ceq     r0,z
        brt     L31
        lc      r0,1
        bra     L32
L31:
        lc      r0,0
L32:
        ceq     r0,z
        brf     L29
        lc      r0,1
        ceq     r0,z
        brf     L29
        lc      r0,0
        bra     L30
L29:
        lc      r0,1
L30:
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L28
        lc      r0,0
        sw      r0,-3(fp)
L28:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L34
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L6
L34:
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
        .byte   68,49,49,79,75,10,0
