// Hello World -- string constants + pointer iteration + UART TX

void putc(int c) {
    *(char *)0xFF0100 = c;
}

void puts(char *s) {
    while (*s) {
        putc(*s);
        s = s + 1;
    }
}

int main() {
    puts("Hello, COR24!\n");
    return 0;
}
