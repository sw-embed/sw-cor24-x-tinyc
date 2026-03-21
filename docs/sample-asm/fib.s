; fib.s -- Recursive Fibonacci (MakerLisp C compiler output)
; C equivalent:
;   int fib(int n) {
;       if (n < 2) return 1;
;       return fib(n-1) + fib(n-2);
;   }
; This is actual MakerLisp compiler output -- the gold standard
; for what cc24 should produce.

        .text

        .globl  _fib
_fib:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lw      r2,9(fp)
; line 7, file "fib.c"
        lc      r0,2
        cls     r2,r0
        brf     L17
; line 8, file "fib.c"
        lc      r0,1
        bra     L16
L17:
; line 11, file "fib.c"
        mov     r0,r2
        add     r0,-1
        push    r0
        la      r0,_fib
        jal     r1,(r0)
        add     sp,3
        sw      r0,-3(fp)
        mov     r0,r2
        add     r0,-2
        push    r0
        la      r0,_fib
        jal     r1,(r0)
        add     sp,3
        lw      r1,-3(fp)
        add     r0,r1
L16:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
