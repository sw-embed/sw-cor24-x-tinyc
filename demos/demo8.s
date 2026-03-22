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
        lc      r0,0
        push    r0
        la      r0,16711680
        mov     r1,r0
        pop     r0
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
        push    r0
        la      r0,128
        mov     r1,r0
        pop     r0
        and     r0,r1
        ceq     r0,z
        brt     L3
        bra     L2
L3:
        lw      r0,9(fp)
        push    r0
        la      r0,16711936
        mov     r1,r0
        pop     r0
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
L5:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L6
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
        bra     L5
L6:
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
        add     sp,-18
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,42
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        push    r0
        lc      r0,42
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L9
        lc      r0,0
        sw      r0,-3(fp)
L9:
        la      r0,_led_on
        jal     r1,(r0)
        la      r0,_S0
        sw      r0,-9(fp)
        lc      r0,0
        sw      r0,-12(fp)
        lw      r0,-9(fp)
        sw      r0,-15(fp)
L10:
        lw      r0,-15(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L11
        lw      r0,-12(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-12(fp)
        lw      r0,-15(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-15(fp)
        bra     L10
L11:
        lw      r0,-12(fp)
        push    r0
        lc      r0,27
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
        lc      r0,99
        sw      r0,-18(fp)
        lw      r0,-18(fp)
        push    r0
        lc      r0,99
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
        lw      r0,-3(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L17
        la      r0,_S1
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L7
L17:
        lc      r0,0
        bra     L7
L7:
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
