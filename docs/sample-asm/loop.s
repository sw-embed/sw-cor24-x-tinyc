; loop.s -- While loop
; C equivalent: int main() { int i=0; while(i<5) i=i+1; return i; }
; Expected: r0 = 5 at halt

        .text

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lc      r2,0            ; i = 0 (use r2 as register variable)
.loop:
        lc      r0,5
        clu     r2,r0           ; i < 5? (unsigned)
        brf     .done           ; if not, exit loop
        add     r2,1            ; i = i + 1
        bra     .loop
.done:
        mov     r0,r2           ; return i
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
