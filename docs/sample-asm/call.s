; call.s -- Function call with argument on stack
; C equivalent:
;   int add(int a, int b) { return a + b; }
;   int main() { return add(2, 5); }
; Expected: r0 = 7 at halt

        .text

        .globl  _add
_add:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)        ; first argument (a = 2)
        lw      r1,12(fp)       ; second argument (b = 5)
        add     r0,r1           ; r0 = a + b
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
        lc      r0,5            ; push b=5 (rightmost arg first)
        push    r0
        lc      r0,2            ; push a=2
        push    r0
        la      r0,_add
        jal     r1,(r0)         ; call add(2, 5)
        add     sp,6            ; clean up 2 arguments
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
