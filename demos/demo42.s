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
        push    r0
        la      r0,128
        mov     r1,r0
        pop     r0
        and     r0,r1
        ceq     r0,z
        brf     L3
        la      r2,L2
        jmp     (r2)
L3:
        la      r2,L1
        jmp     (r2)
L2:
        lw      r0,9(fp)
        push    r0
        la      r0,16711936
        mov     r1,r0
        pop     r0
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
        la      r2,L4
        jmp     (r2)
L4:
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
L6:
        la      r0,16711937
        lbu     r0,0(r0)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        and     r0,r1
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L8
        la      r2,L7
        jmp     (r2)
L8:
        la      r2,L6
        jmp     (r2)
L7:
        la      r0,16711936
        lbu     r0,0(r0)
        la      r2,L5
        jmp     (r2)
L5:
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
        la      r2,L9
        jmp     (r2)
L9:
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
L11:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L13
        la      r2,L12
        jmp     (r2)
L13:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L11
        jmp     (r2)
L12:
        lc      r0,10
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        la      r2,L10
        jmp     (r2)
L10:
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
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L17
        la      r2,L16
        jmp     (r2)
L17:
        lc      r0,45
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        push    r0
        lw      r0,9(fp)
        mov     r1,r0
        pop     r0
        sub     r0,r1
        sw      r0,9(fp)
L16:
        lw      r0,9(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L20
        la      r2,L19
        jmp     (r2)
L20:
        lc      r0,48
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        la      r2,L14
        jmp     (r2)
L19:
        lc      r0,0
        sw      r0,-11(fp)
L21:
        lw      r0,9(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brf     L23
        la      r2,L22
        jmp     (r2)
L23:
        lc      r0,48
        push    r0
        lw      r0,9(fp)
        push    r0
        lc      r0,10
        mov     r1,r0
        pop     r0
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
        push    r0
        lw      r0,-11(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
        lw      r0,9(fp)
        push    r0
        lc      r0,10
        mov     r1,r0
        pop     r0
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
        la      r2,L21
        jmp     (r2)
L22:
L24:
        lw      r0,-11(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brf     L26
        la      r2,L25
        jmp     (r2)
L26:
        lw      r0,-11(fp)
        push    r0
        add     r0,-1
        sw      r0,-11(fp)
        pop     r0
        lc      r0,-8
        add     r0,fp
        push    r0
        lw      r0,-11(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L24
        jmp     (r2)
L25:
L14:
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
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L30
        la      r2,L29
        jmp     (r2)
L30:
        lc      r0,48
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        la      r2,L27
        jmp     (r2)
L29:
        lc      r0,0
        sw      r0,-9(fp)
L31:
        lw      r0,9(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brf     L33
        la      r2,L32
        jmp     (r2)
L33:
        lw      r0,9(fp)
        push    r0
        lc      r0,15
        mov     r1,r0
        pop     r0
        and     r0,r1
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        push    r0
        lc      r0,10
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L36
        la      r2,L34
        jmp     (r2)
L36:
        lc      r0,48
        push    r0
        lw      r0,-12(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        push    r0
        lc      r0,-6
        add     r0,fp
        push    r0
        lw      r0,-9(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
        la      r2,L35
        jmp     (r2)
L34:
        lc      r0,87
        push    r0
        lw      r0,-12(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        push    r0
        lc      r0,-6
        add     r0,fp
        push    r0
        lw      r0,-9(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
L35:
        lw      r0,9(fp)
        push    r0
        lc      r0,4
        mov     r1,r0
        pop     r0
        srl     r0,r1
        sw      r0,9(fp)
        lw      r0,-9(fp)
        push    r0
        add     r0,1
        sw      r0,-9(fp)
        pop     r0
        la      r2,L31
        jmp     (r2)
L32:
L37:
        lw      r0,-9(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brf     L39
        la      r2,L38
        jmp     (r2)
L39:
        lw      r0,-9(fp)
        push    r0
        add     r0,-1
        sw      r0,-9(fp)
        pop     r0
        lc      r0,-6
        add     r0,fp
        push    r0
        lw      r0,-9(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L37
        jmp     (r2)
L38:
L27:
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
L41:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L43
        la      r2,L42
        jmp     (r2)
L43:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L41
        jmp     (r2)
L42:
L40:
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
        push    r0
        lc      r0,100
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L47
        la      r2,L45
        jmp     (r2)
L47:
        lw      r0,12(fp)
        push    r0
        la      r0,__print_int
        jal     r1,(r0)
        add     sp,3
        la      r2,L46
        jmp     (r2)
L45:
        lw      r0,9(fp)
        push    r0
        lc      r0,120
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L50
        la      r2,L48
        jmp     (r2)
L50:
        lw      r0,12(fp)
        push    r0
        la      r0,__print_hex
        jal     r1,(r0)
        add     sp,3
        la      r2,L49
        jmp     (r2)
L48:
        lw      r0,9(fp)
        push    r0
        lc      r0,99
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L53
        la      r2,L51
        jmp     (r2)
L53:
        lw      r0,12(fp)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L52
        jmp     (r2)
L51:
        lw      r0,9(fp)
        push    r0
        lc      r0,115
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L56
        la      r2,L54
        jmp     (r2)
L56:
        lw      r0,12(fp)
        push    r0
        la      r0,__print_str
        jal     r1,(r0)
        add     sp,3
        la      r2,L55
        jmp     (r2)
L54:
        lw      r0,9(fp)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L59
        la      r2,L57
        jmp     (r2)
L59:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L58
        jmp     (r2)
L57:
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
L58:
L55:
L52:
L49:
L46:
L44:
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
L61:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L63
        la      r2,L62
        jmp     (r2)
L63:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L66
        la      r2,L64
        jmp     (r2)
L66:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L69
        la      r2,L67
        jmp     (r2)
L69:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L68
        jmp     (r2)
L67:
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
L68:
        la      r2,L65
        jmp     (r2)
L64:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L65:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L61
        jmp     (r2)
L62:
        lc      r0,0
        la      r2,L60
        jmp     (r2)
L60:
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
L71:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L73
        la      r2,L72
        jmp     (r2)
L73:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L76
        la      r2,L74
        jmp     (r2)
L76:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L79
        la      r2,L77
        jmp     (r2)
L79:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L78
        jmp     (r2)
L77:
        lw      r0,-3(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L82
        la      r2,L80
        jmp     (r2)
L82:
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
        la      r2,L81
        jmp     (r2)
L80:
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
L81:
L78:
        la      r2,L75
        jmp     (r2)
L74:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L75:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L71
        jmp     (r2)
L72:
        lc      r0,0
        la      r2,L70
        jmp     (r2)
L70:
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
L84:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L86
        la      r2,L85
        jmp     (r2)
L86:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L89
        la      r2,L87
        jmp     (r2)
L89:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L92
        la      r2,L90
        jmp     (r2)
L92:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L91
        jmp     (r2)
L90:
        lw      r0,-3(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L95
        la      r2,L93
        jmp     (r2)
L95:
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
        la      r2,L94
        jmp     (r2)
L93:
        lw      r0,-3(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L98
        la      r2,L96
        jmp     (r2)
L98:
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
        la      r2,L97
        jmp     (r2)
L96:
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
L97:
L94:
L91:
        la      r2,L88
        jmp     (r2)
L87:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L88:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L84
        jmp     (r2)
L85:
        lc      r0,0
        la      r2,L83
        jmp     (r2)
L83:
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
L100:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L102
        la      r2,L101
        jmp     (r2)
L102:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L105
        la      r2,L103
        jmp     (r2)
L105:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L108
        la      r2,L106
        jmp     (r2)
L108:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L107
        jmp     (r2)
L106:
        lw      r0,-3(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L111
        la      r2,L109
        jmp     (r2)
L111:
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
        la      r2,L110
        jmp     (r2)
L109:
        lw      r0,-3(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L114
        la      r2,L112
        jmp     (r2)
L114:
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
        la      r2,L113
        jmp     (r2)
L112:
        lw      r0,-3(fp)
        push    r0
        lc      r0,2
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L117
        la      r2,L115
        jmp     (r2)
L117:
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
        la      r2,L116
        jmp     (r2)
L115:
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
L116:
L113:
L110:
L107:
        la      r2,L104
        jmp     (r2)
L103:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L104:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L100
        jmp     (r2)
L101:
        lc      r0,0
        la      r2,L99
        jmp     (r2)
L99:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _malloc
_malloc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-6
        lw      r0,9(fp)
        push    r0
        lc      r0,3
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L121
        la      r2,L120
        jmp     (r2)
L121:
        lc      r0,3
        sw      r0,9(fp)
L120:
        lw      r0,9(fp)
        push    r0
        lc      r0,3
        mov     r1,r0
        pop     r0
        push    r1
        push    r0
        la      r0,__tc24r_mod
        jal     r1,(r0)
        add     sp,6
        sw      r0,-3(fp)
        lw      r0,-3(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L124
        la      r2,L123
        jmp     (r2)
L124:
        lw      r0,9(fp)
        push    r0
        lc      r0,3
        mov     r1,r0
        pop     r0
        add     r0,r1
        push    r0
        lw      r0,-3(fp)
        mov     r1,r0
        pop     r0
        sub     r0,r1
        sw      r0,9(fp)
L123:
        la      r1,__heap_ptr
        lw      r0,0(r1)
        sw      r0,-6(fp)
        la      r1,__heap_ptr
        lw      r0,0(r1)
        push    r0
        lw      r0,9(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        la      r1,__heap_ptr
        sw      r0,0(r1)
        lw      r0,-6(fp)
        la      r2,L118
        jmp     (r2)
L118:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _free
_free:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L125:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _calloc
_calloc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-9
        lw      r0,9(fp)
        push    r0
        lw      r0,12(fp)
        mov     r1,r0
        pop     r0
        mul     r0,r1
        sw      r0,-3(fp)
        lw      r0,-3(fp)
        push    r0
        la      r0,_malloc
        jal     r1,(r0)
        add     sp,3
        sw      r0,-6(fp)
        lc      r0,0
        sw      r0,-9(fp)
L127:
        lw      r0,-9(fp)
        push    r0
        lw      r0,-3(fp)
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L129
        la      r2,L128
        jmp     (r2)
L129:
        lc      r0,0
        push    r0
        lw      r0,-6(fp)
        push    r0
        lw      r0,-9(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
        lw      r0,-9(fp)
        push    r0
        add     r0,1
        sw      r0,-9(fp)
        pop     r0
        la      r2,L127
        jmp     (r2)
L128:
        lw      r0,-6(fp)
        la      r2,L126
        jmp     (r2)
L126:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _realloc
_realloc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,12(fp)
        push    r0
        la      r0,_malloc
        jal     r1,(r0)
        add     sp,3
        la      r2,L130
        jmp     (r2)
L130:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _exit
_exit:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        _exit_halt:
        bra _exit_halt
L131:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _abs
_abs:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L135
        la      r2,L134
        jmp     (r2)
L135:
        lc      r0,0
        push    r0
        lw      r0,9(fp)
        mov     r1,r0
        pop     r0
        sub     r0,r1
        la      r2,L132
        jmp     (r2)
L134:
        lw      r0,9(fp)
        la      r2,L132
        jmp     (r2)
L132:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _atoi
_atoi:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-6
        lc      r0,0
        sw      r0,-3(fp)
        lc      r0,0
        sw      r0,-6(fp)
L137:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,32
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L139
        la      r2,L138
        jmp     (r2)
L139:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L137
        jmp     (r2)
L138:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,45
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L142
        la      r2,L140
        jmp     (r2)
L142:
        lc      r0,1
        sw      r0,-6(fp)
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L141
        jmp     (r2)
L140:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,43
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L145
        la      r2,L144
        jmp     (r2)
L145:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
L144:
L141:
L146:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,48
        mov     r1,r0
        pop     r0
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L150
        la      r2,L148
        jmp     (r2)
L150:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,57
        mov     r1,r0
        pop     r0
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L151
        la      r2,L148
        jmp     (r2)
L151:
        lc      r0,1
        la      r2,L149
        jmp     (r2)
L148:
        lc      r0,0
L149:
        ceq     r0,z
        brf     L152
        la      r2,L147
        jmp     (r2)
L152:
        lw      r0,-3(fp)
        push    r0
        lc      r0,10
        mov     r1,r0
        pop     r0
        mul     r0,r1
        push    r0
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,48
        mov     r1,r0
        pop     r0
        sub     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-3(fp)
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L146
        jmp     (r2)
L147:
        lw      r0,-6(fp)
        ceq     r0,z
        brf     L155
        la      r2,L154
        jmp     (r2)
L155:
        lc      r0,0
        push    r0
        lw      r0,-3(fp)
        mov     r1,r0
        pop     r0
        sub     r0,r1
        la      r2,L136
        jmp     (r2)
L154:
        lw      r0,-3(fp)
        la      r2,L136
        jmp     (r2)
L136:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _sum_list
_sum_list:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,0
        sw      r0,-3(fp)
L157:
        lw      r0,9(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L159
        la      r2,L158
        jmp     (r2)
L159:
        lw      r0,-3(fp)
        push    r0
        lw      r0,9(fp)
        lw      r0,0(r0)
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-3(fp)
        lw      r0,9(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        sw      r0,9(fp)
        la      r2,L157
        jmp     (r2)
L158:
        lw      r0,-3(fp)
        la      r2,L156
        jmp     (r2)
L156:
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
        lc      r0,10
        push    r0
        lc      r0,-9
        add     r0,fp
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,-15
        add     r0,fp
        push    r0
        lc      r0,-6
        add     r0,fp
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,20
        push    r0
        lc      r0,-15
        add     r0,fp
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,-21
        add     r0,fp
        push    r0
        lc      r0,-12
        add     r0,fp
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,12
        push    r0
        lc      r0,-21
        add     r0,fp
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,0
        push    r0
        lc      r0,-18
        add     r0,fp
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lc      r0,-6
        add     r0,fp
        lw      r0,0(r0)
        lw      r0,0(r0)
        push    r0
        lc      r0,20
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L163
        la      r2,L162
        jmp     (r2)
L163:
        lc      r0,0
        sw      r0,-3(fp)
L162:
        lc      r0,-6
        add     r0,fp
        lw      r0,0(r0)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        lw      r0,0(r0)
        push    r0
        lc      r0,12
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L166
        la      r2,L165
        jmp     (r2)
L166:
        lc      r0,0
        sw      r0,-3(fp)
L165:
        lc      r0,-9
        add     r0,fp
        push    r0
        la      r0,_sum_list
        jal     r1,(r0)
        add     sp,3
        push    r0
        lc      r0,42
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brf     L169
        la      r2,L168
        jmp     (r2)
L169:
        lc      r0,0
        sw      r0,-3(fp)
L168:
        lw      r0,-3(fp)
        ceq     r0,z
        brf     L172
        la      r2,L171
        jmp     (r2)
L172:
        la      r0,_S0
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        la      r2,L160
        jmp     (r2)
L171:
        lc      r0,0
        la      r2,L160
        jmp     (r2)
L160:
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
__heap_ptr:
        .word   524288
_S0:
        .byte   68,52,50,79,75,10,0
