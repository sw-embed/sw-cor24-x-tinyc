int counter = 0;

int add(int a, int b) {
    return a + b;
}

int fib(int n) {
    if (n < 2) { return 1; }
    return fib(n - 1) + fib(n - 2);
}

int bitops(int x) {
    int a = x & 15;
    int b = a | 32;
    int c = b ^ 7;
    int d = c << 1;
    int e = d >> 1;
    return e;
}

int main() {
    int small = 42;
    int large = 1000;

    int sum = add(17, 25);

    if (sum == small) {
        counter = 1;
    } else {
        counter = 0;
    }

    int total = 0;
    int i = 1;
    while (i <= 5) {
        total = total + i;
        i = i + 1;
    }

    for (int j = 6; j <= 10; j = j + 1) {
        total = total + j;
    }

    int f = fib(6);

    int b = bitops(large);

    int neg = -1;
    int bitnot = ~0;
    int lognot = !0;

    int ok = 1;
    if (sum != 42) { ok = 0; }
    if (total != 55) { ok = 0; }
    if (f != 13) { ok = 0; }
    if (counter != 1) { ok = 0; }
    if (neg != -1) { ok = 0; }
    if (lognot != 1) { ok = 0; }

    if (ok == 1) {
        return 42;
    }
    return 0;
}
