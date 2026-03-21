; pointer.s -- Pointer dereference
; C equivalent:
;   int main() { int x = 1; int *p = &x; return *p; }
; Expected: r0 = 1 at halt

        .text

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-6           ; two locals: x at fp-3, p at fp-6
        lc      r0,1
        sw      r0,-3(fp)       ; x = 1
        mov     r0,fp
        add     r0,-3           ; r0 = &x (address of x)
        sw      r0,-6(fp)       ; p = &x
        lw      r0,-6(fp)       ; r0 = p
        lw      r0,0(r0)        ; r0 = *p = 1
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
