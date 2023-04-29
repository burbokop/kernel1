#pragma once

/* Hardware text mode color constants. */
enum vga_color {
    VGA_COLOR_BLACK = 0,
    VGA_COLOR_BLUE = 1,
    VGA_COLOR_GREEN = 2,
    VGA_COLOR_CYAN = 3,
    VGA_COLOR_RED = 4,
    VGA_COLOR_MAGENTA = 5,
    VGA_COLOR_BROWN = 6,
    VGA_COLOR_LIGHT_GREY = 7,
    VGA_COLOR_DARK_GREY = 8,
    VGA_COLOR_LIGHT_BLUE = 9,
    VGA_COLOR_LIGHT_GREEN = 10,
    VGA_COLOR_LIGHT_CYAN = 11,
    VGA_COLOR_LIGHT_RED = 12,
    VGA_COLOR_LIGHT_MAGENTA = 13,
    VGA_COLOR_LIGHT_BROWN = 14,
    VGA_COLOR_WHITE = 15,
};

#if defined(__linux__) || defined(_WIN32) || defined(WIN32)

#include <stdlib.h>
#include <stdio.h>
#include <math.h>

#else

#include <stddef.h>

#if !(defined(__i386__) || defined(__x86_64__))
#error "This tutorial needs to be compiled with a ix86-elf or x86_64-elf compiler"
#endif

//#define LLONG_MAX	9223372036854775807LL
//#define LLONG_MIN	(-LLONG_MAX - 1LL)

#ifdef __cplusplus
extern "C" {
#endif

extern char *itoa(int num, char *str, int base);

float fminf(float x, float y);
double fmin(double x, double y);
long double fminl(long double x, long double y);

float fmaxf(float x, float y);
double fmax(double x, double y);
long double fmaxl(long double x, long double y);

float floorf(float arg);
double floor(double arg);
long double floorl(long double arg);

float fmodf(float x, float y);
double fmod(double x, double y);
long double fmodl(long double x, long double y);

extern void *memcpy(void *dest, const void *src, size_t n);
extern void *memmove(void *dest, const void *src, size_t n);
extern int memcmp(const void *s1, const void *s2, size_t n);
extern void *memset(void *s, int c, size_t len);

void *malloc(size_t size);
void free(void *ptr);

int printf(char const *fmt, ...);
extern int putchar(int ch);
extern int puts(const char *str);

#ifdef __cplusplus
}
#endif

#endif
