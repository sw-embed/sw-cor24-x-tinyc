        .text

        .globl  _start
_start:
        la      r0,_main
        jal     r1,(r0)
_halt:
        bra     _halt

        .globl  __putc_uart
__putc_uart:
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

        .globl  _putchar
_putchar:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        bra     L3
L3:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _getchar
_getchar:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L5:
        la      r0,16711937
        lbu     r0,0(r0)
        lc      r1,1
        and     r0,r1
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L6
        bra     L5
L6:
        la      r0,16711936
        lbu     r0,0(r0)
        bra     L4
L4:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _getc
_getc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la      r0,_getchar
        jal     r1,(r0)
        bra     L7
L7:
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
L9:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L10
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        bra     L9
L10:
        lc      r0,10
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        bra     L8
L8:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  __print_int
__print_int:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-11
        lw      r0,9(fp)
        lc      r1,0
        cls     r0,r1
        brf     L13
        lc      r0,45
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        lw      r1,9(fp)
        sub     r0,r1
        sw      r0,9(fp)
L13:
        lw      r0,9(fp)
        lc      r1,0
        ceq     r0,r1
        brf     L15
        lc      r0,48
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        la      r2,L11
        jmp     (r2)
L15:
        lc      r0,0
        sw      r0,-11(fp)
L16:
        lw      r0,9(fp)
        lc      r1,0
        cls     r1,r0
        brt     L20
        la      r2,L17
        jmp     (r2)
L20:
        lc      r0,48
        push    r0
        lw      r0,9(fp)
        lc      r1,10
        push    r1
        push    r0
        la      r0,__tc24r_mod
        jal     r1,(r0)
        add     sp,6
        mov     r1,r0
        pop     r0
        add     r0,r1
        push    r0
        lc      r0,-8
        add     r0,fp
        lw      r1,-11(fp)
        add     r0,r1
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
        lw      r0,9(fp)
        lc      r1,10
        push    r1
        push    r0
        la      r0,__tc24r_div
        jal     r1,(r0)
        add     sp,6
        sw      r0,9(fp)
        lw      r0,-11(fp)
        push    r0
        add     r0,1
        sw      r0,-11(fp)
        pop     r0
        la      r2,L16
        jmp     (r2)
L17:
L18:
        lw      r0,-11(fp)
        lc      r1,0
        cls     r1,r0
        brf     L19
        lw      r0,-11(fp)
        push    r0
        add     r0,-1
        sw      r0,-11(fp)
        pop     r0
        lc      r0,-8
        add     r0,fp
        lw      r1,-11(fp)
        add     r0,r1
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        bra     L18
L19:
L11:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  __print_hex
__print_hex:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-12
        lw      r0,9(fp)
        lc      r1,0
        ceq     r0,r1
        brf     L23
        lc      r0,48
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        la      r2,L21
        jmp     (r2)
L23:
        lc      r0,0
        sw      r0,-9(fp)
L24:
        lw      r0,9(fp)
        lc      r1,0
        cls     r1,r0
        brt     L30
        la      r2,L25
        jmp     (r2)
L30:
        lw      r0,9(fp)
        lc      r1,15
        and     r0,r1
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        lc      r1,10
        cls     r0,r1
        brf     L26
        lc      r0,48
        lw      r1,-12(fp)
        add     r0,r1
        push    r0
        lc      r0,-6
        add     r0,fp
        lw      r1,-9(fp)
        add     r0,r1
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
        bra     L27
L26:
        lc      r0,87
        lw      r1,-12(fp)
        add     r0,r1
        push    r0
        lc      r0,-6
        add     r0,fp
        lw      r1,-9(fp)
        add     r0,r1
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
L27:
        lw      r0,9(fp)
        lc      r1,4
        sra     r0,r1
        sw      r0,9(fp)
        lw      r0,-9(fp)
        push    r0
        add     r0,1
        sw      r0,-9(fp)
        pop     r0
        la      r2,L24
        jmp     (r2)
L25:
L28:
        lw      r0,-9(fp)
        lc      r1,0
        cls     r1,r0
        brf     L29
        lw      r0,-9(fp)
        push    r0
        add     r0,-1
        sw      r0,-9(fp)
        pop     r0
        lc      r0,-6
        add     r0,fp
        lw      r1,-9(fp)
        add     r0,r1
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        bra     L28
L29:
L21:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  __print_str
__print_str:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L32:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L33
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        bra     L32
L33:
L31:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  __fmt_one
__fmt_one:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lc      r1,100
        ceq     r0,r1
        brf     L35
        lw      r0,12(fp)
        push    r0
        la      r0,__print_int
        jal     r1,(r0)
        add     sp,3
        la      r2,L36
        jmp     (r2)
L35:
        lw      r0,9(fp)
        lc      r1,120
        ceq     r0,r1
        brf     L37
        lw      r0,12(fp)
        push    r0
        la      r0,__print_hex
        jal     r1,(r0)
        add     sp,3
        la      r2,L38
        jmp     (r2)
L37:
        lw      r0,9(fp)
        lc      r1,99
        ceq     r0,r1
        brf     L39
        lw      r0,12(fp)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        bra     L40
L39:
        lw      r0,9(fp)
        lc      r1,115
        ceq     r0,r1
        brf     L41
        lw      r0,12(fp)
        push    r0
        la      r0,__print_str
        jal     r1,(r0)
        add     sp,3
        bra     L42
L41:
        lw      r0,9(fp)
        lc      r1,37
        ceq     r0,r1
        brf     L43
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        bra     L44
L43:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L44:
L42:
L40:
L38:
L36:
L34:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  ___tc24r_printf0
___tc24r_printf0:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L46:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L52
        la      r2,L47
        jmp     (r2)
L52:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,37
        ceq     r0,r1
        brf     L48
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,37
        ceq     r0,r1
        brf     L50
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        bra     L51
L50:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L51:
        bra     L49
L48:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L49:
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L46
        jmp     (r2)
L47:
        lc      r0,0
        bra     L45
L45:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  ___tc24r_printf1
___tc24r_printf1:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,0
        sw      r0,-3(fp)
L54:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L63
        la      r2,L55
        jmp     (r2)
L63:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,37
        ceq     r0,r1
        brt     L62
        la      r2,L56
        jmp     (r2)
L62:
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,37
        ceq     r0,r1
        brf     L58
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        bra     L59
L58:
        lw      r0,-3(fp)
        lc      r1,0
        ceq     r0,r1
        brf     L60
        lw      r0,12(fp)
        push    r0
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__fmt_one
        jal     r1,(r0)
        add     sp,6
        lw      r0,-3(fp)
        push    r0
        add     r0,1
        sw      r0,-3(fp)
        pop     r0
        bra     L61
L60:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L61:
L59:
        bra     L57
L56:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L57:
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L54
        jmp     (r2)
L55:
        lc      r0,0
        bra     L53
L53:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  ___tc24r_printf2
___tc24r_printf2:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,0
        sw      r0,-3(fp)
L65:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L76
        la      r2,L66
        jmp     (r2)
L76:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,37
        ceq     r0,r1
        brt     L75
        la      r2,L67
        jmp     (r2)
L75:
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,37
        ceq     r0,r1
        brf     L69
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L70
        jmp     (r2)
L69:
        lw      r0,-3(fp)
        lc      r1,0
        ceq     r0,r1
        brf     L71
        lw      r0,12(fp)
        push    r0
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__fmt_one
        jal     r1,(r0)
        add     sp,6
        lw      r0,-3(fp)
        push    r0
        add     r0,1
        sw      r0,-3(fp)
        pop     r0
        bra     L72
L71:
        lw      r0,-3(fp)
        lc      r1,1
        ceq     r0,r1
        brf     L73
        lw      r0,15(fp)
        push    r0
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__fmt_one
        jal     r1,(r0)
        add     sp,6
        lw      r0,-3(fp)
        push    r0
        add     r0,1
        sw      r0,-3(fp)
        pop     r0
        bra     L74
L73:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L74:
L72:
L70:
        bra     L68
L67:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L68:
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L65
        jmp     (r2)
L66:
        lc      r0,0
        bra     L64
L64:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  ___tc24r_printf3
___tc24r_printf3:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,0
        sw      r0,-3(fp)
L78:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L91
        la      r2,L79
        jmp     (r2)
L91:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,37
        ceq     r0,r1
        brt     L90
        la      r2,L80
        jmp     (r2)
L90:
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,37
        ceq     r0,r1
        brf     L82
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L83
        jmp     (r2)
L82:
        lw      r0,-3(fp)
        lc      r1,0
        ceq     r0,r1
        brf     L84
        lw      r0,12(fp)
        push    r0
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__fmt_one
        jal     r1,(r0)
        add     sp,6
        lw      r0,-3(fp)
        push    r0
        add     r0,1
        sw      r0,-3(fp)
        pop     r0
        la      r2,L85
        jmp     (r2)
L84:
        lw      r0,-3(fp)
        lc      r1,1
        ceq     r0,r1
        brf     L86
        lw      r0,15(fp)
        push    r0
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__fmt_one
        jal     r1,(r0)
        add     sp,6
        lw      r0,-3(fp)
        push    r0
        add     r0,1
        sw      r0,-3(fp)
        pop     r0
        bra     L87
L86:
        lw      r0,-3(fp)
        lc      r1,2
        ceq     r0,r1
        brf     L88
        lw      r0,18(fp)
        push    r0
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__fmt_one
        jal     r1,(r0)
        add     sp,6
        lw      r0,-3(fp)
        push    r0
        add     r0,1
        sw      r0,-3(fp)
        pop     r0
        bra     L89
L88:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L89:
L87:
L85:
L83:
        bra     L81
L80:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L81:
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L78
        jmp     (r2)
L79:
        lc      r0,0
        bra     L77
L77:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _add
_add:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r1,12(fp)
        add     r0,r1
        bra     L92
L92:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _sub
_sub:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r1,12(fp)
        sub     r0,r1
        bra     L93
L93:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _apply
_apply:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,15(fp)
        push    r0
        lw      r0,12(fp)
        push    r0
        lw      r0,9(fp)
        jal     r1,(r0)
        add     sp,6
        bra     L94
L94:
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
        add     sp,-9
        lc      r0,20
        push    r0
        lc      r0,30
        push    r0
        la      r0,_add
        push    r0
        la      r0,_apply
        jal     r1,(r0)
        add     sp,9
        sw      r0,-3(fp)
        lw      r0,-3(fp)
        lc      r1,50
        ceq     r0,r1
        brt     L97
        lc      r0,1
        la      r2,L95
        jmp     (r2)
L97:
        lc      r0,20
        push    r0
        lc      r0,30
        push    r0
        la      r0,_sub
        push    r0
        la      r0,_apply
        jal     r1,(r0)
        add     sp,9
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,10
        ceq     r0,r1
        brt     L99
        lc      r0,2
        bra     L95
L99:
        lc      r0,3
        push    r0
        lw      r0,-3(fp)
        push    r0
        la      r0,_add
        push    r0
        la      r0,_apply
        jal     r1,(r0)
        add     sp,9
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        lc      r1,53
        ceq     r0,r1
        brt     L101
        lc      r0,3
        bra     L95
L101:
        la      r0,_S0
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        lc      r0,53
        bra     L95
L95:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

__tc24r_div:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r1,12(fp)
        lc      r2,0
__tc24r_div_lp:
        cls     r0,r1
        brt     __tc24r_div_dn
        sub     r0,r1
        add     r2,1
        bra     __tc24r_div_lp
__tc24r_div_dn:
        mov     r0,r2
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
__tc24r_mod:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r1,12(fp)
__tc24r_mod_lp:
        cls     r0,r1
        brt     __tc24r_mod_dn
        sub     r0,r1
        bra     __tc24r_mod_lp
__tc24r_mod_dn:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_S0:
        .byte   68,53,51,79,75,10,0
