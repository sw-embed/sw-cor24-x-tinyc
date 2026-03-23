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
        la      r2,L6
        jmp     (r2)
L7:
        lc      r0,10
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        la      r2,L5
        jmp     (r2)
L5:
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
        brf     L12
        la      r2,L11
        jmp     (r2)
L12:
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
L11:
        lw      r0,9(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L15
        la      r2,L14
        jmp     (r2)
L15:
        lc      r0,48
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        la      r2,L9
        jmp     (r2)
L14:
        lc      r0,0
        sw      r0,-11(fp)
L16:
        lw      r0,9(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brf     L18
        la      r2,L17
        jmp     (r2)
L18:
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
        la      r2,L16
        jmp     (r2)
L17:
L19:
        lw      r0,-11(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brf     L21
        la      r2,L20
        jmp     (r2)
L21:
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
        la      r2,L19
        jmp     (r2)
L20:
L9:
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
        brf     L25
        la      r2,L24
        jmp     (r2)
L25:
        lc      r0,48
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        la      r2,L22
        jmp     (r2)
L24:
        lc      r0,0
        sw      r0,-9(fp)
L26:
        lw      r0,9(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brf     L28
        la      r2,L27
        jmp     (r2)
L28:
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
        brf     L31
        la      r2,L29
        jmp     (r2)
L31:
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
        la      r2,L30
        jmp     (r2)
L29:
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
L30:
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
        la      r2,L26
        jmp     (r2)
L27:
L32:
        lw      r0,-9(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        brf     L34
        la      r2,L33
        jmp     (r2)
L34:
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
        la      r2,L32
        jmp     (r2)
L33:
L22:
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
L36:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L38
        la      r2,L37
        jmp     (r2)
L38:
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
        la      r2,L36
        jmp     (r2)
L37:
L35:
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
        brf     L42
        la      r2,L40
        jmp     (r2)
L42:
        lw      r0,12(fp)
        push    r0
        la      r0,__print_int
        jal     r1,(r0)
        add     sp,3
        la      r2,L41
        jmp     (r2)
L40:
        lw      r0,9(fp)
        push    r0
        lc      r0,120
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L45
        la      r2,L43
        jmp     (r2)
L45:
        lw      r0,12(fp)
        push    r0
        la      r0,__print_hex
        jal     r1,(r0)
        add     sp,3
        la      r2,L44
        jmp     (r2)
L43:
        lw      r0,9(fp)
        push    r0
        lc      r0,99
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L48
        la      r2,L46
        jmp     (r2)
L48:
        lw      r0,12(fp)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L47
        jmp     (r2)
L46:
        lw      r0,9(fp)
        push    r0
        lc      r0,115
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L51
        la      r2,L49
        jmp     (r2)
L51:
        lw      r0,12(fp)
        push    r0
        la      r0,__print_str
        jal     r1,(r0)
        add     sp,3
        la      r2,L50
        jmp     (r2)
L49:
        lw      r0,9(fp)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L54
        la      r2,L52
        jmp     (r2)
L54:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L53
        jmp     (r2)
L52:
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
L53:
L50:
L47:
L44:
L41:
L39:
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
L56:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L58
        la      r2,L57
        jmp     (r2)
L58:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L61
        la      r2,L59
        jmp     (r2)
L61:
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
        brf     L64
        la      r2,L62
        jmp     (r2)
L64:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L63
        jmp     (r2)
L62:
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
L63:
        la      r2,L60
        jmp     (r2)
L59:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L60:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L56
        jmp     (r2)
L57:
        lc      r0,0
        la      r2,L55
        jmp     (r2)
L55:
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
L66:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L68
        la      r2,L67
        jmp     (r2)
L68:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L71
        la      r2,L69
        jmp     (r2)
L71:
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
        brf     L74
        la      r2,L72
        jmp     (r2)
L74:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L73
        jmp     (r2)
L72:
        lw      r0,-3(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L77
        la      r2,L75
        jmp     (r2)
L77:
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
        la      r2,L76
        jmp     (r2)
L75:
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
L76:
L73:
        la      r2,L70
        jmp     (r2)
L69:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L70:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L66
        jmp     (r2)
L67:
        lc      r0,0
        la      r2,L65
        jmp     (r2)
L65:
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
L79:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L81
        la      r2,L80
        jmp     (r2)
L81:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L84
        la      r2,L82
        jmp     (r2)
L84:
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
        brf     L87
        la      r2,L85
        jmp     (r2)
L87:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L86
        jmp     (r2)
L85:
        lw      r0,-3(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L90
        la      r2,L88
        jmp     (r2)
L90:
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
        la      r2,L89
        jmp     (r2)
L88:
        lw      r0,-3(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L93
        la      r2,L91
        jmp     (r2)
L93:
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
        la      r2,L92
        jmp     (r2)
L91:
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
L92:
L89:
L86:
        la      r2,L83
        jmp     (r2)
L82:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L83:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L79
        jmp     (r2)
L80:
        lc      r0,0
        la      r2,L78
        jmp     (r2)
L78:
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
L95:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brf     L97
        la      r2,L96
        jmp     (r2)
L97:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lc      r0,37
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L100
        la      r2,L98
        jmp     (r2)
L100:
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
        brf     L103
        la      r2,L101
        jmp     (r2)
L103:
        lc      r0,37
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
        la      r2,L102
        jmp     (r2)
L101:
        lw      r0,-3(fp)
        push    r0
        lc      r0,0
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L106
        la      r2,L104
        jmp     (r2)
L106:
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
        la      r2,L105
        jmp     (r2)
L104:
        lw      r0,-3(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L109
        la      r2,L107
        jmp     (r2)
L109:
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
        la      r2,L108
        jmp     (r2)
L107:
        lw      r0,-3(fp)
        push    r0
        lc      r0,2
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L112
        la      r2,L110
        jmp     (r2)
L112:
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
        la      r2,L111
        jmp     (r2)
L110:
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
L111:
L108:
L105:
L102:
        la      r2,L99
        jmp     (r2)
L98:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,__putc_uart
        jal     r1,(r0)
        add     sp,3
L99:
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L95
        jmp     (r2)
L96:
        lc      r0,0
        la      r2,L94
        jmp     (r2)
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
        add     sp,-3
        lc      r0,1
        sw      r0,-3(fp)
        la      r0,_S0
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        lc      r0,39
        push    r0
        la      r0,_S1
        push    r0
        la      r0,___tc24r_printf1
        jal     r1,(r0)
        add     sp,6
        lc      r0,75
        push    r0
        lc      r0,79
        push    r0
        la      r0,_S2
        push    r0
        la      r0,___tc24r_printf2
        jal     r1,(r0)
        add     sp,9
        la      r0,_S3
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        lw      r0,-3(fp)
        ceq     r0,z
        brf     L116
        la      r2,L115
        jmp     (r2)
L116:
        lc      r0,42
        la      r2,L113
        jmp     (r2)
L115:
        lc      r0,0
        la      r2,L113
        jmp     (r2)
L113:
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
        .byte   68,0
_S1:
        .byte   37,100,0
_S2:
        .byte   37,99,37,99,0
_S3:
        .byte   10,0
