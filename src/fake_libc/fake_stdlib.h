#pragma once

#if defined(__linux__) || defined(_WIN32) || defined(WIN32)

#include <math.h>
#include <stdlib.h>
#include <string.h>

#else

#include <stddef.h>

#if !(defined(__i386__) || defined(__x86_64__))
#error "This file needs to be compiled with a ix86-elf or x86_64-elf compiler"
#endif

//#define LLONG_MAX	9223372036854775807LL
//#define LLONG_MIN	(-LLONG_MAX - 1LL)

#ifdef __cplusplus
extern "C" {
#endif

extern char *itoa(int num, char *str, int base);

extern float fminf(float x, float y);
extern double fmin(double x, double y);
extern long double fminl(long double x, long double y);

extern float fmaxf(float x, float y);
extern double fmax(double x, double y);
extern long double fmaxl(long double x, long double y);

extern float floorf(float arg);
extern double floor(double arg);
extern long double floorl(long double arg);

extern float fmodf(float x, float y);
extern double fmod(double x, double y);
extern long double fmodl(long double x, long double y);

extern void *memcpy(void *dest, const void *src, size_t n);
extern void *memmove(void *dest, const void *src, size_t n);
extern int memcmp(const void *s1, const void *s2, size_t n);
extern void *memset(void *s, int c, size_t len);

extern void *malloc(size_t size);
extern void *calloc(size_t num, size_t size);

extern void free(void *ptr);
extern void *realloc(void *ptr, size_t new_size);

#ifdef __cplusplus
}
#endif

#endif
