#pragma once

// tc24r freestanding string.h

#define NULL 0

int strlen(char *s) {
    int len = 0;
    while (*s) {
        len++;
        s = s + 1;
    }
    return len;
}

int strcmp(char *a, char *b) {
    while (*a && *b && *a == *b) {
        a = a + 1;
        b = b + 1;
    }
    return *a - *b;
}

int strncmp(char *a, char *b, int n) {
    int i = 0;
    while (i < n && *a && *b && *a == *b) {
        a = a + 1;
        b = b + 1;
        i++;
    }
    if (i == n) return 0;
    return *a - *b;
}

char *strcpy(char *dst, char *src) {
    char *ret = dst;
    while (*src) {
        *dst = *src;
        dst = dst + 1;
        src = src + 1;
    }
    *dst = 0;
    return ret;
}

char *strncpy(char *dst, char *src, int n) {
    char *ret = dst;
    int i = 0;
    while (i < n && *src) {
        *dst = *src;
        dst = dst + 1;
        src = src + 1;
        i++;
    }
    while (i < n) {
        *dst = 0;
        dst = dst + 1;
        i++;
    }
    return ret;
}

void *memcpy(void *dst, void *src, int n) {
    char *d = (char *)dst;
    char *s = (char *)src;
    int i = 0;
    while (i < n) {
        d[i] = s[i];
        i++;
    }
    return dst;
}

void *memset(void *s, int c, int n) {
    char *p = (char *)s;
    int i = 0;
    while (i < n) {
        p[i] = c;
        i++;
    }
    return s;
}

int memcmp(void *a, void *b, int n) {
    char *pa = (char *)a;
    char *pb = (char *)b;
    int i = 0;
    while (i < n) {
        if (pa[i] != pb[i]) return pa[i] - pb[i];
        i++;
    }
    return 0;
}
