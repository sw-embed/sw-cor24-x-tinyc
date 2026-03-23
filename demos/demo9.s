        .text

        .globl  _start
_start:
        la      r0,_main
        jal     r1,(r0)
_halt:
        bra     _halt

        .globl  _uart_isr
_uart_isr:
        push    r0
        push    r1
        push    r2
        mov     r2,c
        push    r2
        push    fp
        mov     fp,sp
        la      r0,16711936
        lbu     r0,0(r0)
        la      r1,_rx_char
        sw      r0,0(r1)
L0:
        mov     sp,fp
        pop     fp
        pop     r2
        clu     z,r2
        pop     r2
        pop     r1
        pop     r0
        jmp     (ir)

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la r0,_uart_isr
        mov iv,r0
        la      r0,16711696
        mov     r1,r0
        lc      r0,1
        sb      r0,0(r1)
L2:
        la      r1,_rx_char
        lw      r0,0(r1)
        lc      r1,0
        ceq     r0,r1
        brt     L4
        la      r2,L3
        jmp     (r2)
L4:
        la      r2,L2
        jmp     (r2)
L3:
        la      r1,_rx_char
        lw      r0,0(r1)
        la      r2,L1
        jmp     (r2)
L1:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_rx_char:
        .word   0
