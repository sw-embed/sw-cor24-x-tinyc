; globals.s -- Global variable read/write
; C equivalent:
;   int x = 10;
;   int main() { x = x + 5; return x; }
; Expected: r0 = 15 at halt

        .text

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la      r1,_x           ; load address of x
        lw      r0,0(r1)        ; r0 = x (10)
        add     r0,5            ; r0 = 15
        sw      r0,0(r1)        ; x = 15
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
_x:
        .word   10
