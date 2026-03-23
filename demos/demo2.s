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

        .globl  _led_off
_led_off:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la      r0,16711680
        mov     r1,r0
        lc      r0,1
        sb      r0,0(r1)
L1:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _uart_putc
_uart_putc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L3:
        la      r0,16711937
        lbu     r0,0(r0)
        la      r1,128
        and     r0,r1
        ceq     r0,z
        brf     L5
        la      r2,L4
        jmp     (r2)
L5:
        la      r2,L3
        jmp     (r2)
L4:
        la      r0,16711936
        mov     r1,r0
        lw      r0,9(fp)
        sb      r0,0(r1)
L2:
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
        lc      r0,65
        sw      r0,-6(fp)
        lc      r0,66
        sw      r0,-9(fp)
        lw      r0,-6(fp)
        lc      r1,65
        ceq     r0,r1
        brf     L9
        la      r2,L8
        jmp     (r2)
L9:
        lc      r0,0
        sw      r0,-3(fp)
L8:
        lw      r0,-9(fp)
        lc      r1,66
        ceq     r0,r1
        brf     L12
        la      r2,L11
        jmp     (r2)
L12:
        lc      r0,0
        sw      r0,-3(fp)
L11:
        lc      r0,123
        sw      r0,-12(fp)
        lc      r0,-12
        add     r0,fp
        sw      r0,-15(fp)
        lw      r0,-15(fp)
        lw      r0,0(r0)
        lc      r1,123
        ceq     r0,r1
        brf     L15
        la      r2,L14
        jmp     (r2)
L15:
        lc      r0,0
        sw      r0,-3(fp)
L14:
        lw      r0,-15(fp)
        mov     r1,r0
        la      r0,456
        sw      r0,0(r1)
        lw      r0,-12(fp)
        la      r1,456
        ceq     r0,r1
        brf     L18
        la      r2,L17
        jmp     (r2)
L18:
        lc      r0,0
        sw      r0,-3(fp)
L17:
        lc      r0,77
        sw      r0,-18(fp)
        lc      r0,-18
        add     r0,fp
        sw      r0,-21(fp)
        lw      r0,-21(fp)
        lbu     r0,0(r0)
        lc      r1,77
        ceq     r0,r1
        brf     L21
        la      r2,L20
        jmp     (r2)
L21:
        lc      r0,0
        sw      r0,-3(fp)
L20:
        la      r0,_led_on
        jal     r1,(r0)
        lw      r0,-3(fp)
        lc      r1,1
        ceq     r0,r1
        brt     L24
        la      r2,L23
        jmp     (r2)
L24:
        lc      r0,79
        push    r0
        la      r0,_uart_putc
        jal     r1,(r0)
        add     sp,3
        lc      r0,75
        push    r0
        la      r0,_uart_putc
        jal     r1,(r0)
        add     sp,3
        lc      r0,10
        push    r0
        la      r0,_uart_putc
        jal     r1,(r0)
        add     sp,3
L23:
        lw      r0,-3(fp)
        lc      r1,1
        ceq     r0,r1
        brt     L27
        la      r2,L26
        jmp     (r2)
L27:
        lc      r0,42
        la      r2,L6
        jmp     (r2)
L26:
        lc      r0,0
        la      r2,L6
        jmp     (r2)
L6:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
