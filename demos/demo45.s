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

        .globl  _malloc
_malloc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-6
        lw      r0,9(fp)
        lc      r1,3
        cls     r0,r1
        brf     L94
        lc      r0,3
        sw      r0,9(fp)
L94:
        lw      r0,9(fp)
        lc      r1,3
        push    r1
        push    r0
        la      r0,__tc24r_mod
        jal     r1,(r0)
        add     sp,6
        sw      r0,-3(fp)
        lw      r0,-3(fp)
        lc      r1,0
        ceq     r0,r1
        brt     L96
        lw      r0,9(fp)
        lc      r1,3
        add     r0,r1
        lw      r1,-3(fp)
        sub     r0,r1
        sw      r0,9(fp)
L96:
        la      r1,__heap_ptr
        lw      r0,0(r1)
        sw      r0,-6(fp)
        la      r1,__heap_ptr
        lw      r0,0(r1)
        lw      r1,9(fp)
        add     r0,r1
        la      r1,__heap_ptr
        sw      r0,0(r1)
        lw      r0,-6(fp)
        bra     L92
L92:
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
L97:
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
        lw      r1,12(fp)
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
L99:
        lw      r0,-9(fp)
        lw      r1,-3(fp)
        cls     r0,r1
        brf     L100
        lw      r0,-6(fp)
        lw      r1,-9(fp)
        add     r0,r1
        mov     r1,r0
        lc      r0,0
        sb      r0,0(r1)
        lw      r0,-9(fp)
        push    r0
        add     r0,1
        sw      r0,-9(fp)
        pop     r0
        bra     L99
L100:
        lw      r0,-6(fp)
        bra     L98
L98:
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
        bra     L101
L101:
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
L102:
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
        lc      r1,0
        cls     r0,r1
        brf     L105
        lc      r0,0
        lw      r1,9(fp)
        sub     r0,r1
        bra     L103
L105:
        lw      r0,9(fp)
        bra     L103
L103:
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
L107:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,32
        ceq     r0,r1
        brf     L108
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        bra     L107
L108:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,45
        ceq     r0,r1
        brf     L109
        lc      r0,1
        sw      r0,-6(fp)
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        bra     L110
L109:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,43
        ceq     r0,r1
        brf     L112
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
L112:
L110:
L113:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,48
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L115
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,57
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L115
        lc      r0,1
        bra     L116
L115:
        lc      r0,0
L116:
        ceq     r0,z
        brt     L114
        lw      r0,-3(fp)
        lc      r1,10
        mul     r0,r1
        push    r0
        lw      r0,9(fp)
        lbu     r0,0(r0)
        lc      r1,48
        sub     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-3(fp)
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        la      r2,L113
        jmp     (r2)
L114:
        lw      r0,-6(fp)
        ceq     r0,z
        brt     L118
        lc      r0,0
        lw      r1,-3(fp)
        sub     r0,r1
        bra     L106
L118:
        lw      r0,-3(fp)
        bra     L106
L106:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _strlen
_strlen:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,0
        sw      r0,-3(fp)
L120:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L121
        lw      r0,-3(fp)
        push    r0
        add     r0,1
        sw      r0,-3(fp)
        pop     r0
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        bra     L120
L121:
        lw      r0,-3(fp)
        bra     L119
L119:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _strcmp
_strcmp:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L123:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L127
        lw      r0,12(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L127
        lc      r0,1
        bra     L128
L127:
        lc      r0,0
L128:
        ceq     r0,z
        brt     L125
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lw      r0,12(fp)
        lbu     r0,0(r0)
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L125
        lc      r0,1
        bra     L126
L125:
        lc      r0,0
L126:
        ceq     r0,z
        brt     L124
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,12(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,12(fp)
        la      r2,L123
        jmp     (r2)
L124:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lw      r0,12(fp)
        lbu     r0,0(r0)
        mov     r1,r0
        pop     r0
        sub     r0,r1
        bra     L122
L122:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _strncmp
_strncmp:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,0
        sw      r0,-3(fp)
L130:
        lw      r0,-3(fp)
        lw      r1,15(fp)
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L136
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L136
        lc      r0,1
        bra     L137
L136:
        lc      r0,0
L137:
        ceq     r0,z
        brt     L134
        lw      r0,12(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L134
        lc      r0,1
        bra     L135
L134:
        lc      r0,0
L135:
        ceq     r0,z
        brt     L132
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lw      r0,12(fp)
        lbu     r0,0(r0)
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L132
        lc      r0,1
        bra     L133
L132:
        lc      r0,0
L133:
        ceq     r0,z
        brt     L131
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,12(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,12(fp)
        lw      r0,-3(fp)
        push    r0
        add     r0,1
        sw      r0,-3(fp)
        pop     r0
        la      r2,L130
        jmp     (r2)
L131:
        lw      r0,-3(fp)
        lw      r1,15(fp)
        ceq     r0,r1
        brf     L139
        lc      r0,0
        bra     L129
L139:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        lw      r0,12(fp)
        lbu     r0,0(r0)
        mov     r1,r0
        pop     r0
        sub     r0,r1
        bra     L129
L129:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _strcpy
_strcpy:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lw      r0,9(fp)
        sw      r0,-3(fp)
L141:
        lw      r0,12(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L142
        lw      r0,12(fp)
        lbu     r0,0(r0)
        push    r0
        lw      r0,9(fp)
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,12(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,12(fp)
        bra     L141
L142:
        lw      r0,9(fp)
        mov     r1,r0
        lc      r0,0
        sb      r0,0(r1)
        lw      r0,-3(fp)
        bra     L140
L140:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _strncpy
_strncpy:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-6
        lw      r0,9(fp)
        sw      r0,-3(fp)
        lc      r0,0
        sw      r0,-6(fp)
L144:
        lw      r0,-6(fp)
        lw      r1,15(fp)
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L146
        lw      r0,12(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L146
        lc      r0,1
        bra     L147
L146:
        lc      r0,0
L147:
        ceq     r0,z
        brt     L145
        lw      r0,12(fp)
        lbu     r0,0(r0)
        push    r0
        lw      r0,9(fp)
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,12(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,12(fp)
        lw      r0,-6(fp)
        push    r0
        add     r0,1
        sw      r0,-6(fp)
        pop     r0
        la      r2,L144
        jmp     (r2)
L145:
L148:
        lw      r0,-6(fp)
        lw      r1,15(fp)
        cls     r0,r1
        brf     L149
        lw      r0,9(fp)
        mov     r1,r0
        lc      r0,0
        sb      r0,0(r1)
        lw      r0,9(fp)
        lc      r1,1
        add     r0,r1
        sw      r0,9(fp)
        lw      r0,-6(fp)
        push    r0
        add     r0,1
        sw      r0,-6(fp)
        pop     r0
        bra     L148
L149:
        lw      r0,-3(fp)
        bra     L143
L143:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _memcpy
_memcpy:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-9
        lw      r0,9(fp)
        sw      r0,-3(fp)
        lw      r0,12(fp)
        sw      r0,-6(fp)
        lc      r0,0
        sw      r0,-9(fp)
L151:
        lw      r0,-9(fp)
        lw      r1,15(fp)
        cls     r0,r1
        brf     L152
        lw      r0,-6(fp)
        lw      r1,-9(fp)
        add     r0,r1
        lbu     r0,0(r0)
        push    r0
        lw      r0,-3(fp)
        lw      r1,-9(fp)
        add     r0,r1
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
        lw      r0,-9(fp)
        push    r0
        add     r0,1
        sw      r0,-9(fp)
        pop     r0
        bra     L151
L152:
        lw      r0,9(fp)
        bra     L150
L150:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _memset
_memset:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-6
        lw      r0,9(fp)
        sw      r0,-3(fp)
        lc      r0,0
        sw      r0,-6(fp)
L154:
        lw      r0,-6(fp)
        lw      r1,15(fp)
        cls     r0,r1
        brf     L155
        lw      r0,-3(fp)
        lw      r1,-6(fp)
        add     r0,r1
        mov     r1,r0
        lw      r0,12(fp)
        sb      r0,0(r1)
        lw      r0,-6(fp)
        push    r0
        add     r0,1
        sw      r0,-6(fp)
        pop     r0
        bra     L154
L155:
        lw      r0,9(fp)
        bra     L153
L153:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _memcmp
_memcmp:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-9
        lw      r0,9(fp)
        sw      r0,-3(fp)
        lw      r0,12(fp)
        sw      r0,-6(fp)
        lc      r0,0
        sw      r0,-9(fp)
L157:
        lw      r0,-9(fp)
        lw      r1,15(fp)
        cls     r0,r1
        brt     L161
        la      r2,L158
        jmp     (r2)
L161:
        lw      r0,-3(fp)
        lw      r1,-9(fp)
        add     r0,r1
        lbu     r0,0(r0)
        push    r0
        lw      r0,-6(fp)
        lw      r1,-9(fp)
        add     r0,r1
        lbu     r0,0(r0)
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        brt     L160
        lw      r0,-3(fp)
        lw      r1,-9(fp)
        add     r0,r1
        lbu     r0,0(r0)
        push    r0
        lw      r0,-6(fp)
        lw      r1,-9(fp)
        add     r0,r1
        lbu     r0,0(r0)
        mov     r1,r0
        pop     r0
        sub     r0,r1
        bra     L156
L160:
        lw      r0,-9(fp)
        push    r0
        add     r0,1
        sw      r0,-9(fp)
        pop     r0
        la      r2,L157
        jmp     (r2)
L158:
        lc      r0,0
        bra     L156
L156:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _make_nil
_make_nil:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,15
        push    r0
        la      r0,_malloc
        jal     r1,(r0)
        add     sp,3
        sw      r0,-3(fp)
        lc      r0,3
        push    r0
        lw      r0,-3(fp)
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lw      r0,-3(fp)
        bra     L162
L162:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _make_num
_make_num:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,15
        push    r0
        la      r0,_malloc
        jal     r1,(r0)
        add     sp,3
        sw      r0,-3(fp)
        lc      r0,0
        push    r0
        lw      r0,-3(fp)
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lw      r0,9(fp)
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lw      r0,-3(fp)
        bra     L163
L163:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _make_sym
_make_sym:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,15
        push    r0
        la      r0,_malloc
        jal     r1,(r0)
        add     sp,3
        sw      r0,-3(fp)
        lc      r0,1
        push    r0
        lw      r0,-3(fp)
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lw      r0,9(fp)
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lw      r0,-3(fp)
        bra     L164
L164:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _cons
_cons:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lc      r0,15
        push    r0
        la      r0,_malloc
        jal     r1,(r0)
        add     sp,3
        sw      r0,-3(fp)
        lc      r0,2
        push    r0
        lw      r0,-3(fp)
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lw      r0,9(fp)
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,9
        pop     r1
        add     r0,r1
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lw      r0,12(fp)
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,12
        pop     r1
        add     r0,r1
        mov     r1,r0
        pop     r0
        sw      r0,0(r1)
        lw      r0,-3(fp)
        bra     L165
L165:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _car
_car:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        push    r0
        lc      r0,9
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        bra     L166
L166:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _cdr
_cdr:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        push    r0
        lc      r0,12
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        bra     L167
L167:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _atom
_atom:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r0,0(r0)
        lc      r1,2
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        bra     L168
L168:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _null
_null:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        lw      r0,0(r0)
        lc      r1,3
        ceq     r0,r1
        mov     r0,c
        bra     L169
L169:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _print_val
_print_val:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lw      r0,9(fp)
        lw      r0,0(r0)
        lc      r1,0
        ceq     r0,r1
        brf     L171
        lw      r0,9(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_S0
        push    r0
        la      r0,___tc24r_printf1
        jal     r1,(r0)
        add     sp,6
        la      r2,L172
        jmp     (r2)
L171:
        lw      r0,9(fp)
        lw      r0,0(r0)
        lc      r1,1
        ceq     r0,r1
        brf     L173
        lw      r0,9(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_S1
        push    r0
        la      r0,___tc24r_printf1
        jal     r1,(r0)
        add     sp,6
        la      r2,L174
        jmp     (r2)
L173:
        lw      r0,9(fp)
        lw      r0,0(r0)
        lc      r1,3
        ceq     r0,r1
        brf     L175
        la      r0,_S2
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        la      r2,L176
        jmp     (r2)
L175:
        lw      r0,9(fp)
        lw      r0,0(r0)
        lc      r1,2
        ceq     r0,r1
        brt     L183
        la      r2,L178
        jmp     (r2)
L183:
        la      r0,_S3
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        push    r0
        lc      r0,9
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_print_val
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        push    r0
        lc      r0,12
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        sw      r0,-3(fp)
L179:
        lw      r0,-3(fp)
        lw      r0,0(r0)
        lc      r1,2
        ceq     r0,r1
        brf     L180
        la      r0,_S4
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        lw      r0,-3(fp)
        push    r0
        lc      r0,9
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_print_val
        jal     r1,(r0)
        add     sp,3
        lw      r0,-3(fp)
        push    r0
        lc      r0,12
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        sw      r0,-3(fp)
        bra     L179
L180:
        lw      r0,-3(fp)
        lw      r0,0(r0)
        lc      r1,3
        ceq     r0,r1
        brt     L182
        la      r0,_S5
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        lw      r0,-3(fp)
        push    r0
        la      r0,_print_val
        jal     r1,(r0)
        add     sp,3
L182:
        la      r0,_S6
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
L178:
L176:
L174:
L172:
L170:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _peek
_peek:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la      r1,__input
        lw      r0,0(r1)
        la      r1,__pos
        lw      r1,0(r1)
        add     r0,r1
        lbu     r0,0(r0)
        bra     L184
L184:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _next
_next:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        la      r1,__input
        lw      r0,0(r1)
        la      r1,__pos
        lw      r1,0(r1)
        add     r0,r1
        lbu     r0,0(r0)
        sw      r0,-3(fp)
        la      r1,__pos
        lw      r0,0(r1)
        push    r0
        add     r0,1
        la      r1,__pos
        sw      r0,0(r1)
        pop     r0
        lw      r0,-3(fp)
        bra     L185
L185:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _skip_ws
_skip_ws:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L187:
        la      r0,_peek
        jal     r1,(r0)
        lc      r1,32
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L191
        la      r0,_peek
        jal     r1,(r0)
        lc      r1,10
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L191
        lc      r0,0
        bra     L192
L191:
        lc      r0,1
L192:
        ceq     r0,z
        brf     L189
        la      r0,_peek
        jal     r1,(r0)
        lc      r1,9
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brf     L189
        lc      r0,0
        bra     L190
L189:
        lc      r0,1
L190:
        ceq     r0,z
        brt     L188
        la      r0,_next
        jal     r1,(r0)
        la      r2,L187
        jmp     (r2)
L188:
L186:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _read_list
_read_list:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-6
        la      r0,_skip_ws
        jal     r1,(r0)
        la      r0,_peek
        jal     r1,(r0)
        lc      r1,41
        ceq     r0,r1
        brf     L195
        la      r0,_next
        jal     r1,(r0)
        la      r1,_NIL
        lw      r0,0(r1)
        bra     L193
L195:
        la      r0,_read_val
        jal     r1,(r0)
        sw      r0,-3(fp)
        la      r0,_read_list
        jal     r1,(r0)
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        push    r0
        lw      r0,-3(fp)
        push    r0
        la      r0,_cons
        jal     r1,(r0)
        add     sp,6
        bra     L193
L193:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _read_val
_read_val:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-31
        la      r0,_skip_ws
        jal     r1,(r0)
        la      r0,_peek
        jal     r1,(r0)
        sw      r0,-3(fp)
        lw      r0,-3(fp)
        lc      r1,48
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L201
        lw      r0,-3(fp)
        lc      r1,57
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L201
        lc      r0,1
        bra     L202
L201:
        lc      r0,0
L202:
        ceq     r0,z
        brt     L228
        la      r2,L199
        jmp     (r2)
L228:
        lw      r0,-3(fp)
        lc      r1,45
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L205
        la      r1,__input
        lw      r0,0(r1)
        push    r0
        la      r1,__pos
        lw      r0,0(r1)
        lc      r1,1
        add     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        lbu     r0,0(r0)
        lc      r1,48
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L205
        lc      r0,1
        bra     L206
L205:
        lc      r0,0
L206:
        ceq     r0,z
        brt     L203
        la      r1,__input
        lw      r0,0(r1)
        push    r0
        la      r1,__pos
        lw      r0,0(r1)
        lc      r1,1
        add     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        lbu     r0,0(r0)
        lc      r1,57
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L203
        lc      r0,1
        bra     L204
L203:
        lc      r0,0
L204:
        ceq     r0,z
        brf     L199
        lc      r0,0
        bra     L200
L199:
        lc      r0,1
L200:
        ceq     r0,z
        brf     L227
        la      r2,L198
        jmp     (r2)
L227:
        lc      r0,1
        sw      r0,-6(fp)
        lw      r0,-3(fp)
        lc      r1,45
        ceq     r0,r1
        brf     L208
        lc      r0,1
        push    r0
        lc      r0,0
        pop     r1
        sub     r0,r1
        sw      r0,-6(fp)
        la      r0,_next
        jal     r1,(r0)
L208:
        lc      r0,0
        sw      r0,-9(fp)
L209:
        la      r0,_peek
        jal     r1,(r0)
        lc      r1,48
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L211
        la      r0,_peek
        jal     r1,(r0)
        lc      r1,57
        cls     r1,r0
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L211
        lc      r0,1
        bra     L212
L211:
        lc      r0,0
L212:
        ceq     r0,z
        brt     L210
        lw      r0,-9(fp)
        lc      r1,10
        mul     r0,r1
        push    r0
        la      r0,_next
        jal     r1,(r0)
        lc      r1,48
        sub     r0,r1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,-9(fp)
        la      r2,L209
        jmp     (r2)
L210:
        lw      r0,-9(fp)
        lw      r1,-6(fp)
        mul     r0,r1
        push    r0
        la      r0,_make_num
        jal     r1,(r0)
        add     sp,3
        la      r2,L196
        jmp     (r2)
L198:
        lw      r0,-3(fp)
        lc      r1,40
        ceq     r0,r1
        brf     L214
        la      r0,_next
        jal     r1,(r0)
        la      r0,_read_list
        jal     r1,(r0)
        la      r2,L196
        jmp     (r2)
L214:
        lc      r0,0
        sw      r0,-28(fp)
L215:
        la      r0,_peek
        jal     r1,(r0)
        lc      r1,0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L225
        la      r0,_peek
        jal     r1,(r0)
        lc      r1,32
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L225
        lc      r0,1
        bra     L226
L225:
        lc      r0,0
L226:
        ceq     r0,z
        brt     L223
        la      r0,_peek
        jal     r1,(r0)
        lc      r1,41
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L223
        lc      r0,1
        bra     L224
L223:
        lc      r0,0
L224:
        ceq     r0,z
        brt     L221
        la      r0,_peek
        jal     r1,(r0)
        lc      r1,40
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L221
        lc      r0,1
        bra     L222
L221:
        lc      r0,0
L222:
        ceq     r0,z
        brt     L219
        la      r0,_peek
        jal     r1,(r0)
        lc      r1,10
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L219
        lc      r0,1
        bra     L220
L219:
        lc      r0,0
L220:
        ceq     r0,z
        brt     L217
        lw      r0,-28(fp)
        lc      r1,15
        cls     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L217
        lc      r0,1
        bra     L218
L217:
        lc      r0,0
L218:
        ceq     r0,z
        brt     L216
        la      r0,_next
        jal     r1,(r0)
        push    r0
        lc      r0,-25
        add     r0,fp
        lw      r1,-28(fp)
        add     r0,r1
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
        lw      r0,-28(fp)
        push    r0
        add     r0,1
        sw      r0,-28(fp)
        pop     r0
        la      r2,L215
        jmp     (r2)
L216:
        lc      r0,-25
        add     r0,fp
        lw      r1,-28(fp)
        add     r0,r1
        mov     r1,r0
        lc      r0,0
        sb      r0,0(r1)
        lw      r0,-28(fp)
        lc      r1,1
        add     r0,r1
        push    r0
        la      r0,_malloc
        jal     r1,(r0)
        add     sp,3
        sw      r0,-31(fp)
        lc      r0,-25
        add     r0,fp
        push    r0
        lw      r0,-31(fp)
        push    r0
        la      r0,_strcpy
        jal     r1,(r0)
        add     sp,6
        lw      r0,-31(fp)
        push    r0
        la      r0,_make_sym
        jal     r1,(r0)
        add     sp,3
        bra     L196
L196:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _read_str
_read_str:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        la      r1,__input
        sw      r0,0(r1)
        lc      r0,0
        la      r1,__pos
        sw      r0,0(r1)
        la      r0,_read_val
        jal     r1,(r0)
        bra     L229
L229:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _env_lookup
_env_lookup:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
L231:
        lw      r0,12(fp)
        lw      r0,0(r0)
        lc      r1,2
        ceq     r0,r1
        brt     L235
        la      r2,L232
        jmp     (r2)
L235:
        lw      r0,12(fp)
        push    r0
        lc      r0,9
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        sw      r0,-3(fp)
        lw      r0,9(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,9
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brf     L234
        lw      r0,-3(fp)
        push    r0
        lc      r0,12
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        bra     L230
L234:
        lw      r0,12(fp)
        push    r0
        lc      r0,12
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        sw      r0,12(fp)
        la      r2,L231
        jmp     (r2)
L232:
        la      r1,_NIL
        lw      r0,0(r1)
        bra     L230
L230:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _env_bind
_env_bind:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,15(fp)
        push    r0
        lw      r0,12(fp)
        push    r0
        lw      r0,9(fp)
        push    r0
        la      r0,_cons
        jal     r1,(r0)
        add     sp,6
        push    r0
        la      r0,_cons
        jal     r1,(r0)
        add     sp,6
        bra     L236
L236:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _evlis
_evlis:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        push    r0
        la      r0,_null
        jal     r1,(r0)
        add     sp,3
        ceq     r0,z
        brt     L239
        la      r1,_NIL
        lw      r0,0(r1)
        bra     L237
L239:
        lw      r0,12(fp)
        push    r0
        lw      r0,9(fp)
        push    r0
        la      r0,_cdr
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_evlis
        jal     r1,(r0)
        add     sp,6
        push    r0
        lw      r0,12(fp)
        push    r0
        lw      r0,9(fp)
        push    r0
        la      r0,_car
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        push    r0
        la      r0,_cons
        jal     r1,(r0)
        add     sp,6
        bra     L237
L237:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _eval
_eval:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-18
        lw      r0,9(fp)
        lw      r0,0(r0)
        lc      r1,0
        ceq     r0,r1
        brf     L242
        lw      r0,9(fp)
        la      r2,L240
        jmp     (r2)
L242:
        lw      r0,9(fp)
        lw      r0,0(r0)
        lc      r1,3
        ceq     r0,r1
        brf     L244
        lw      r0,9(fp)
        la      r2,L240
        jmp     (r2)
L244:
        lw      r0,9(fp)
        lw      r0,0(r0)
        lc      r1,1
        ceq     r0,r1
        brf     L246
        lw      r0,12(fp)
        push    r0
        lw      r0,9(fp)
        push    r0
        la      r0,_env_lookup
        jal     r1,(r0)
        add     sp,6
        la      r2,L240
        jmp     (r2)
L246:
        lw      r0,9(fp)
        push    r0
        la      r0,_car
        jal     r1,(r0)
        add     sp,3
        sw      r0,-3(fp)
        lw      r0,9(fp)
        push    r0
        la      r0,_cdr
        jal     r1,(r0)
        add     sp,3
        sw      r0,-6(fp)
        lw      r0,-3(fp)
        lw      r0,0(r0)
        lc      r1,1
        ceq     r0,r1
        brt     L286
        la      r2,L248
        jmp     (r2)
L286:
        la      r0,_S7
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brf     L250
        lw      r0,-6(fp)
        push    r0
        la      r0,_car
        jal     r1,(r0)
        add     sp,3
        la      r2,L240
        jmp     (r2)
L250:
        la      r0,_S8
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brt     L285
        la      r2,L252
        jmp     (r2)
L285:
        lw      r0,12(fp)
        push    r0
        lw      r0,-6(fp)
        push    r0
        la      r0,_car
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        push    r0
        la      r0,_null
        jal     r1,(r0)
        add     sp,3
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L255
        lw      r0,-9(fp)
        lw      r0,0(r0)
        lc      r1,0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L257
        lw      r0,-9(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        lc      r1,0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L257
        lc      r0,1
        bra     L258
L257:
        lc      r0,0
L258:
        ceq     r0,z
        mov     r0,c
        ceq     r0,z
        brt     L255
        lc      r0,1
        bra     L256
L255:
        lc      r0,0
L256:
        ceq     r0,z
        brt     L253
        lw      r0,12(fp)
        push    r0
        lw      r0,-6(fp)
        push    r0
        la      r0,_cdr
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_car
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        la      r2,L240
        jmp     (r2)
        bra     L254
L253:
        lw      r0,12(fp)
        push    r0
        lw      r0,-6(fp)
        push    r0
        la      r0,_cdr
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_cdr
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_car
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        la      r2,L240
        jmp     (r2)
L254:
L252:
L248:
        lw      r0,12(fp)
        push    r0
        lw      r0,-6(fp)
        push    r0
        la      r0,_evlis
        jal     r1,(r0)
        add     sp,6
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        push    r0
        la      r0,_car
        jal     r1,(r0)
        add     sp,3
        sw      r0,-15(fp)
        lw      r0,-12(fp)
        push    r0
        la      r0,_cdr
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_car
        jal     r1,(r0)
        add     sp,3
        sw      r0,-18(fp)
        lw      r0,-3(fp)
        lw      r0,0(r0)
        lc      r1,1
        ceq     r0,r1
        brt     L284
        la      r2,L260
        jmp     (r2)
L284:
        la      r0,_S9
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brf     L262
        lw      r0,-15(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        lw      r0,-18(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        mov     r1,r0
        pop     r0
        add     r0,r1
        push    r0
        la      r0,_make_num
        jal     r1,(r0)
        add     sp,3
        la      r2,L240
        jmp     (r2)
L262:
        la      r0,_S10
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brf     L264
        lw      r0,-15(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        lw      r0,-18(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        mov     r1,r0
        pop     r0
        sub     r0,r1
        push    r0
        la      r0,_make_num
        jal     r1,(r0)
        add     sp,3
        la      r2,L240
        jmp     (r2)
L264:
        la      r0,_S11
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brf     L266
        lw      r0,-15(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        lw      r0,-18(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        mov     r1,r0
        pop     r0
        mul     r0,r1
        push    r0
        la      r0,_make_num
        jal     r1,(r0)
        add     sp,3
        la      r2,L240
        jmp     (r2)
L266:
        la      r0,_S12
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brf     L268
        lw      r0,-15(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        lw      r0,-18(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        mov     r1,r0
        pop     r0
        push    r1
        push    r0
        la      r0,__tc24r_div
        jal     r1,(r0)
        add     sp,6
        push    r0
        la      r0,_make_num
        jal     r1,(r0)
        add     sp,3
        la      r2,L240
        jmp     (r2)
L268:
        la      r0,_S13
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brf     L270
        lw      r0,-18(fp)
        push    r0
        lw      r0,-15(fp)
        push    r0
        la      r0,_cons
        jal     r1,(r0)
        add     sp,6
        la      r2,L240
        jmp     (r2)
L270:
        la      r0,_S14
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brf     L272
        lw      r0,-15(fp)
        push    r0
        la      r0,_car
        jal     r1,(r0)
        add     sp,3
        la      r2,L240
        jmp     (r2)
L272:
        la      r0,_S15
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brf     L274
        lw      r0,-15(fp)
        push    r0
        la      r0,_cdr
        jal     r1,(r0)
        add     sp,3
        la      r2,L240
        jmp     (r2)
L274:
        la      r0,_S16
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brt     L283
        la      r2,L276
        jmp     (r2)
L283:
        lw      r0,-15(fp)
        lw      r0,0(r0)
        lc      r1,0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L279
        lw      r0,-18(fp)
        lw      r0,0(r0)
        lc      r1,0
        ceq     r0,r1
        mov     r0,c
        ceq     r0,z
        brt     L279
        lc      r0,1
        bra     L280
L279:
        lc      r0,0
L280:
        ceq     r0,z
        brt     L278
        lw      r0,-15(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        lw      r0,-18(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        mov     r1,r0
        pop     r0
        ceq     r0,r1
        mov     r0,c
        push    r0
        la      r0,_make_num
        jal     r1,(r0)
        add     sp,3
        la      r2,L240
        jmp     (r2)
L278:
        la      r1,_NIL
        lw      r0,0(r1)
        bra     L240
L276:
        la      r0,_S17
        push    r0
        lw      r0,-3(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brf     L282
        lw      r0,-15(fp)
        push    r0
        la      r0,_atom
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_make_num
        jal     r1,(r0)
        add     sp,3
        bra     L240
L282:
L260:
        la      r1,_NIL
        lw      r0,0(r1)
        bra     L240
L240:
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
        add     sp,-30
        lc      r0,1
        sw      r0,-3(fp)
        la      r0,_make_nil
        jal     r1,(r0)
        la      r1,_NIL
        sw      r0,0(r1)
        la      r1,_NIL
        lw      r0,0(r1)
        push    r0
        la      r0,_S18
        push    r0
        la      r0,_read_str
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        push    r0
        la      r0,_print_val
        jal     r1,(r0)
        add     sp,3
        la      r0,_S19
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        lw      r0,-6(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        lc      r1,42
        ceq     r0,r1
        brt     L289
        lc      r0,0
        sw      r0,-3(fp)
L289:
        la      r1,_NIL
        lw      r0,0(r1)
        push    r0
        la      r0,_S20
        push    r0
        la      r0,_read_str
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        sw      r0,-9(fp)
        lw      r0,-9(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        lc      r1,42
        ceq     r0,r1
        brt     L291
        lc      r0,0
        sw      r0,-3(fp)
L291:
        la      r1,_NIL
        lw      r0,0(r1)
        push    r0
        la      r0,_S21
        push    r0
        la      r0,_read_str
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        sw      r0,-12(fp)
        lw      r0,-12(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        lc      r1,42
        ceq     r0,r1
        brt     L293
        lc      r0,0
        sw      r0,-3(fp)
L293:
        la      r1,_NIL
        lw      r0,0(r1)
        push    r0
        la      r0,_S22
        push    r0
        la      r0,_read_str
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        sw      r0,-15(fp)
        lw      r0,-15(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        lc      r1,42
        ceq     r0,r1
        brt     L295
        lc      r0,0
        sw      r0,-3(fp)
L295:
        la      r1,_NIL
        lw      r0,0(r1)
        push    r0
        la      r0,_S23
        push    r0
        la      r0,_read_str
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        sw      r0,-18(fp)
        lw      r0,-18(fp)
        push    r0
        la      r0,_print_val
        jal     r1,(r0)
        add     sp,3
        la      r0,_S24
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        la      r0,_S25
        push    r0
        lw      r0,-18(fp)
        push    r0
        lc      r0,6
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        push    r0
        la      r0,_strcmp
        jal     r1,(r0)
        add     sp,6
        lc      r1,0
        ceq     r0,r1
        brt     L297
        lc      r0,0
        sw      r0,-3(fp)
L297:
        la      r1,_NIL
        lw      r0,0(r1)
        push    r0
        la      r0,_S26
        push    r0
        la      r0,_read_str
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        sw      r0,-21(fp)
        lw      r0,-21(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        lc      r1,42
        ceq     r0,r1
        brt     L299
        lc      r0,0
        sw      r0,-3(fp)
L299:
        la      r1,_NIL
        lw      r0,0(r1)
        push    r0
        la      r0,_S27
        push    r0
        la      r0,_read_str
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        sw      r0,-24(fp)
        lw      r0,-24(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        lc      r1,42
        ceq     r0,r1
        brt     L301
        lc      r0,0
        sw      r0,-3(fp)
L301:
        la      r1,_NIL
        lw      r0,0(r1)
        push    r0
        la      r0,_S28
        push    r0
        la      r0,_read_str
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        sw      r0,-27(fp)
        lw      r0,-27(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        lc      r1,42
        ceq     r0,r1
        brt     L303
        lc      r0,0
        sw      r0,-3(fp)
L303:
        la      r1,_NIL
        lw      r0,0(r1)
        push    r0
        la      r0,_S29
        push    r0
        la      r0,_read_str
        jal     r1,(r0)
        add     sp,3
        push    r0
        la      r0,_eval
        jal     r1,(r0)
        add     sp,6
        sw      r0,-30(fp)
        lw      r0,-30(fp)
        push    r0
        la      r0,_print_val
        jal     r1,(r0)
        add     sp,3
        la      r0,_S30
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        lw      r0,-30(fp)
        push    r0
        lc      r0,3
        pop     r1
        add     r0,r1
        lw      r0,0(r0)
        lc      r1,42
        ceq     r0,r1
        brt     L305
        lc      r0,0
        sw      r0,-3(fp)
L305:
        lw      r0,-3(fp)
        ceq     r0,z
        brt     L307
        la      r0,_S31
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        lc      r0,42
        bra     L287
L307:
        la      r0,_S32
        push    r0
        la      r0,___tc24r_printf0
        jal     r1,(r0)
        add     sp,3
        lc      r0,0
        bra     L287
L287:
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
_NIL:
        .word   0
__input:
        .word   0
__pos:
        .word   0
_S0:
        .byte   37,100,0
_S1:
        .byte   37,115,0
_S2:
        .byte   110,105,108,0
_S3:
        .byte   40,0
_S4:
        .byte   32,0
_S5:
        .byte   32,46,32,0
_S6:
        .byte   41,0
_S7:
        .byte   113,117,111,116,101,0
_S8:
        .byte   105,102,0
_S9:
        .byte   43,0
_S10:
        .byte   45,0
_S11:
        .byte   42,0
_S12:
        .byte   47,0
_S13:
        .byte   99,111,110,115,0
_S14:
        .byte   99,97,114,0
_S15:
        .byte   99,100,114,0
_S16:
        .byte   101,113,0
_S17:
        .byte   97,116,111,109,0
_S18:
        .byte   40,43,32,52,48,32,50,41,0
_S19:
        .byte   10,0
_S20:
        .byte   40,45,32,53,48,32,56,41,0
_S21:
        .byte   40,42,32,54,32,55,41,0
_S22:
        .byte   40,43,32,40,42,32,53,32,56,41,32,50,41,0
_S23:
        .byte   40,113,117,111,116,101,32,104,101,108,108,111,41,0
_S24:
        .byte   10,0
_S25:
        .byte   104,101,108,108,111,0
_S26:
        .byte   40,105,102,32,49,32,52,50,32,48,41,0
_S27:
        .byte   40,105,102,32,48,32,57,57,32,52,50,41,0
_S28:
        .byte   40,99,97,114,32,40,99,111,110,115,32,52,50,32,57,57,41,41,0
_S29:
        .byte   40,43,32,40,43,32,49,48,32,50,48,41,32,40,43,32,53,32,55,41,41,0
_S30:
        .byte   10,0
_S31:
        .byte   68,52,53,79,75,10,0
_S32:
        .byte   70,65,73,76,10,0
