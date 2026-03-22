; led.s -- LED output via MMIO
; C equivalent:
;   void led_on(void)  { *(volatile char *)0xFF0000 = 0; }  // active-low
;   void led_off(void) { *(volatile char *)0xFF0000 = 1; }
;   int main() { led_on(); return 0; }
; Expected: LED turns on (active-low: writing 0 turns LED on)

        .text

        .globl  _led_on
_led_on:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la      r1,0xFF0000     ; GPIO address
        lc      r0,0            ; 0 = LED on (active-low)
        sb      r0,0(r1)
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _led_off
_led_off:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        la      r1,0xFF0000     ; GPIO address
        lc      r0,1            ; 1 = LED off (active-low)
        sb      r0,0(r1)
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
        la      r0,_led_on
        jal     r1,(r0)         ; call led_on()
        lc      r0,0
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
