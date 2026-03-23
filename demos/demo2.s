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
        brt     L4
        bra     L3
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
        brt     L7
        lc      r0,0
        sw      r0,-3(fp)
L7:
        lw      r0,-9(fp)
        lc      r1,66
        ceq     r0,r1
        brt     L9
        lc      r0,0
        sw      r0,-3(fp)
L9:
        lc      r0,123
        sw      r0,-12(fp)
        lc      r0,-12
        add     r0,fp
        sw      r0,-15(fp)
        lw      r0,-15(fp)
        lw      r0,0(r0)
        lc      r1,123
        ceq     r0,r1
        brt     L11
        lc      r0,0
        sw      r0,-3(fp)
L11:
        lw      r0,-15(fp)
        mov     r1,r0
        la      r0,456
        sw      r0,0(r1)
        lw      r0,-12(fp)
        la      r1,456
        ceq     r0,r1
        brt     L13
        lc      r0,0
        sw      r0,-3(fp)
L13:
        lc      r0,77
        sw      r0,-18(fp)
        lc      r0,-18
        add     r0,fp
        sw      r0,-21(fp)
        lw      r0,-21(fp)
        lbu     r0,0(r0)
        lc      r1,77
        ceq     r0,r1
        brt     L15
        lc      r0,0
        sw      r0,-3(fp)
L15:
        la      r0,_led_on
        jal     r1,(r0)
        lw      r0,-3(fp)
        lc      r1,1
        ceq     r0,r1
        brf     L17
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
L17:
        lw      r0,-3(fp)
        lc      r1,1
        ceq     r0,r1
        brf     L19
        lc      r0,42
        bra     L5
L19:
        lc      r0,0
        bra     L5
L5:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
