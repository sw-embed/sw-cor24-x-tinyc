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
        brf     L3
        la      r2,L2
        jmp     (r2)
L3:
        la      r2,L1
        jmp     (r2)
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
L5:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L7
        la      r2,L6
        jmp     (r2)
L7:
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
        la      r2,L5
        jmp     (r2)
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
        add     sp,-3
        lc      r0,1
        sw      r0,-3(fp)
        lc      r0,1
        ceq     r0,z
        brf     L13
        la      r2,L11
        jmp     (r2)
L13:
        lc      r0,2
        ceq     r0,z
        brf     L14
        la      r2,L11
        jmp     (r2)
L14:
        lc      r0,1
        la      r2,L12
        jmp     (r2)
L11:
        lc      r0,0
L12:
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L15
        la      r2,L10
        jmp     (r2)
L15:
        lc      r0,0
        sw      r0,-3(fp)
L10:
        lc      r0,0
        ceq     r0,z
        brf     L20
        la      r2,L18
        jmp     (r2)
L20:
        lc      r0,1
        ceq     r0,z
        brf     L21
        la      r2,L18
        jmp     (r2)
L21:
        lc      r0,1
        la      r2,L19
        jmp     (r2)
L18:
        lc      r0,0
L19:
        ceq     r0,z
        brf     L22
        la      r2,L17
        jmp     (r2)
L22:
        lc      r0,0
        sw      r0,-3(fp)
L17:
        lc      r0,1
        ceq     r0,z
        brt     L27
        la      r2,L25
        jmp     (r2)
L27:
        lc      r0,0
        ceq     r0,z
        brt     L28
        la      r2,L25
        jmp     (r2)
L28:
        lc      r0,0
        la      r2,L26
        jmp     (r2)
L25:
        lc      r0,1
L26:
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L29
        la      r2,L24
        jmp     (r2)
L29:
        lc      r0,0
        sw      r0,-3(fp)
L24:
        lc      r0,0
        ceq     r0,z
        brt     L34
        la      r2,L32
        jmp     (r2)
L34:
        lc      r0,0
        ceq     r0,z
        brt     L35
        la      r2,L32
        jmp     (r2)
L35:
        lc      r0,0
        la      r2,L33
        jmp     (r2)
L32:
        lc      r0,1
L33:
        ceq     r0,z
        brf     L36
        la      r2,L31
        jmp     (r2)
L36:
        lc      r0,0
        sw      r0,-3(fp)
L31:
        lc      r0,0
        ceq     r0,z
        brt     L41
        la      r2,L39
        jmp     (r2)
L41:
        lc      r0,5
        ceq     r0,z
        brt     L42
        la      r2,L39
        jmp     (r2)
L42:
        lc      r0,0
        la      r2,L40
        jmp     (r2)
L39:
        lc      r0,1
L40:
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L43
        la      r2,L38
        jmp     (r2)
L43:
        lc      r0,0
        sw      r0,-3(fp)
L38:
        lc      r0,1
        ceq     r0,z
        brf     L50
        la      r2,L48
        jmp     (r2)
L50:
        lc      r0,0
        ceq     r0,z
        brf     L51
        la      r2,L48
        jmp     (r2)
L51:
        lc      r0,1
        la      r2,L49
        jmp     (r2)
L48:
        lc      r0,0
L49:
        ceq     r0,z
        brt     L52
        la      r2,L46
        jmp     (r2)
L52:
        lc      r0,1
        ceq     r0,z
        brt     L53
        la      r2,L46
        jmp     (r2)
L53:
        lc      r0,0
        la      r2,L47
        jmp     (r2)
L46:
        lc      r0,1
L47:
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L54
        la      r2,L45
        jmp     (r2)
L54:
        lc      r0,0
        sw      r0,-3(fp)
L45:
        lw      r0,-3(fp)
        ceq     r0,z
        brf     L57
        la      r2,L56
        jmp     (r2)
L57:
        la      r0,_S0
        push    r0
        la      r0,_puts
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        la      r2,L8
        jmp     (r2)
L56:
        lc      r0,0
        la      r2,L8
        jmp     (r2)
L8:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   68,49,49,79,75,10,0
