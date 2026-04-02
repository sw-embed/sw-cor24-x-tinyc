#pragma once

// tc24r freestanding stdarg.h (stub)
// Variadic functions are not supported on COR24.

typedef int va_list;

#define va_start(ap, last)
#define va_end(ap)
#define va_arg(ap, type) ((type)0)
#define va_copy(dest, src)
