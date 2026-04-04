void putc(int c) { *(char *)0xFF0100 = c; }
void puts(const char *s) { while (*s) { putc(*s); s = s + 1; } }

struct Point { int x; int y; };

int main(void) {
    int a = (int){10};

    int *p = &(int){20};
    int b = *p;

    int *arr = (int[]){5, 15, 10};
    int c = arr[0] + arr[1] + arr[2];

    struct Point *pt = &(struct Point){5, 0};
    int d = pt->x + pt->y;

    puts("D60");

    return a + b + c + d - 5;
}
