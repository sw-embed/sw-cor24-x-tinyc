// Send characters via UART TX
void putc(int c) {
    while (*(char *)0xFF0101 & 0x80) {}
    *(char *)0xFF0100 = c;
}

int main() {
    putc(72);   // 'H'
    putc(105);  // 'i'
    putc(33);   // '!'
    putc(10);   // newline
    return 0;
}
