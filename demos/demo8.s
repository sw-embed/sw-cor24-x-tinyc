        .text

        .globl  _start
_start:
        la      r0,_main
        jal     r1,(r0)
_halt:
        bra     _halt

        .globl  _led_on
_led_on:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la      r0,16711680
        mov     r1,r0
        lc      r0,0
        sb      r0,0(r1)
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _putc
_putc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L2:
        la      r0,16711937
        lbu     r0,0(r0)
        la      r1,128
        and     r0,r1
        ceq     r0,z
        brf     L4
        la      r2,L3
        jmp     (r2)
L4:
        la      r2,L2
        jmp     (r2)
L3:
        la      r0,16711936
        mov     r1,r0
        lw      r0,9(fp)
        sb      r0,0(r1)
L1:
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
L6:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L8
        la      r2,L7
        jmp     (r2)
L8:
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
        la      r2,L6
        jmp     (r2)
L7:
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
        add     sp,-18
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,42
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,42
        ceq     r0,r1
        brf     L12
        la      r2,L11
        jmp     (r2)
L12:
        lc      r0,0
        sw      r0,-3(fp)
L11:
        la      r0,_led_on
        jal     r1,(r0)
        la      r0,_S0
        sw      r0,-9(fp)
        lc      r0,0
        sw      r0,-12(fp)
        lw      r0,-9(fp)
        sw      r0,-15(fp)
L13:
        lw      r0,-15(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L15
        la      r2,L14
        jmp     (r2)
L15:
        lw      r0,-12(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-12(fp)
        lw      r0,-15(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,-15(fp)
        la      r2,L13
        jmp     (r2)
L14:
        lw      r0,-12(fp)
        lc      r1,27
        ceq     r0,r1
        brf     L18
        la      r2,L17
        jmp     (r2)
L18:
        lc      r0,0
        sw      r0,-3(fp)
L17:
        lc      r0,99
        sw      r0,-18(fp)
        lw      r0,-18(fp)
        lc      r1,99
        ceq     r0,r1
        brf     L21
        la      r2,L20
        jmp     (r2)
L21:
        lc      r0,0
        sw      r0,-3(fp)
L20:
        lw      r0,-3(fp)
        lc      r1,1
        ceq     r0,r1
        brt     L24
        la      r2,L23
        jmp     (r2)
L24:
        la      r0,_S1
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        la      r2,L9
        jmp     (r2)
L23:
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
_S0:
        .byte   65,78,83,87,69,82,32,105,115,32,110,111,116,32,114,101,112,108,97,99,101,100,32,104,101,114,101,0
_S1:
        .byte   68,56,79,75,10,0
