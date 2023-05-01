#include "fake_stdio.h"

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>

#include "fake_stdlib.h"

void panic(const char *message);

static inline uint8_t vga_entry_color(enum vga_color fg, enum vga_color bg)
{
    return fg | bg << 4;
}

static inline uint16_t vga_entry(unsigned char uc, uint8_t color)
{
    return (uint16_t) uc | (uint16_t) color << 8;
}

static const size_t VGA_WIDTH = 80;
static const size_t VGA_HEIGHT = 25;

static size_t terminal_row;
static size_t terminal_column;
static uint8_t terminal_color;
static uint16_t *terminal_buffer;

static bool __term_inited__ = false;
static void terminal_initialize(void)
{
    if (__term_inited__)
        return;

    terminal_row = 0;
    terminal_column = 0;
    terminal_color = vga_entry_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    terminal_buffer = (uint16_t *) 0xB8000;
    for (size_t y = 0; y < VGA_HEIGHT; y++) {
        for (size_t x = 0; x < VGA_WIDTH; x++) {
            const size_t index = y * VGA_WIDTH + x;
            terminal_buffer[index] = vga_entry(' ', terminal_color);
        }
    }
    __term_inited__ = true;
}

static void terminal_setcolor(uint8_t color)
{
    terminal_color = color;
}

static void terminal_putentryat(char c, uint8_t color, size_t x, size_t y)
{
    const size_t index = y * VGA_WIDTH + x;
    terminal_buffer[index] = vga_entry(c, color);
}

static void terminal_putchar(char c)
{
    if (c == '\n') {
        terminal_column = 0;
        ++terminal_row;
    } else {
        terminal_putentryat(c, terminal_color, terminal_column, terminal_row);
        if (++terminal_column == VGA_WIDTH) {
            terminal_column = 0;
            if (++terminal_row == VGA_HEIGHT)
                terminal_row = 0;
        }
    }
}

static int terminal_write(const char *data, size_t size)
{
    for (size_t i = 0; i < size; i++)
        terminal_putchar(data[i]);

    return size;
}

static int terminal_writestring(const char *data)
{
    return terminal_write(data, strlen(data));
}

int nputs(const char *str, size_t n)
{
    terminal_initialize();
    const int count = terminal_write(str, n);
    terminal_putchar('\n');
    return count + 1;
}

int puts(const char *str)
{
    terminal_initialize();
    return nputs(str, strlen(str));
}

int putchar(int ch)
{
    terminal_initialize();
    terminal_putchar(ch);
    return ch;
}

static void ftoa_fixed(char *buffer, double value);
static void ftoa_sci(char *buffer, double value);

int vprintf(char const *fmt, va_list arg)
{
    if (!fmt) {
        panic("vprintf: NIL fmt");
        return 0;
    }

    terminal_initialize();

    int int_temp;
    char char_temp;
    char *string_temp;
    double double_temp;

    char ch;
    int length = 0;

    char buffer[512];

    while (ch = *fmt++) {
        if ('%' == ch) {
            switch (ch = *fmt++) {
            /* %% - print out a single %    */
            case '%':
                terminal_putchar('%');
                length++;
                break;

            /* %c: print out a character    */
            case 'c':
                char_temp = va_arg(arg, int);
                terminal_putchar(char_temp);
                length++;
                break;

            /* %s: print out a string       */
            case 's':
                string_temp = va_arg(arg, char *);
                terminal_writestring(string_temp);
                length += strlen(string_temp);
                break;

            /* %d: print out an int         */
            case 'd':
                int_temp = va_arg(arg, int);
                itoa(int_temp, buffer, 10);
                terminal_writestring(buffer);
                length += strlen(buffer);
                break;

            /* %x: print out an int in hex  */
            case 'x':
                int_temp = va_arg(arg, int);
                itoa(int_temp, buffer, 16);
                terminal_writestring(buffer);
                length += strlen(buffer);
                break;

            case 'f':
                double_temp = va_arg(arg, double);
                ftoa_fixed(buffer, double_temp);
                terminal_writestring(buffer);
                length += strlen(buffer);
                break;

            case 'e':
                double_temp = va_arg(arg, double);
                ftoa_sci(buffer, double_temp);
                terminal_writestring(buffer);
                length += strlen(buffer);
                break;
            }
        } else {
            terminal_putchar(ch);
            length++;
        }
    }
    return length;
}

int printf(char const *fmt, ...)
{
    va_list arg;
    int length;

    va_start(arg, fmt);
    length = vprintf(fmt, arg);
    va_end(arg);
    return length;
}

static int normalize(double *val)
{
    int exponent = 0;
    double value = *val;

    while (value >= 1.0) {
        value /= 10.0;
        ++exponent;
    }

    while (value < 0.1) {
        value *= 10.0;
        --exponent;
    }
    *val = value;
    return exponent;
}

static void ftoa_fixed(char *buffer, double value)
{
    int exponent = 0;
    int places = 0;
    static const int width = 4;

    if (value == 0.0) {
        buffer[0] = '0';
        buffer[1] = '\0';
        return;
    }

    if (value < 0.0) {
        *buffer++ = '-';
        value = -value;
    }

    exponent = normalize(&value);

    while (exponent > 0) {
        int digit = value * 10;
        *buffer++ = digit + '0';
        value = value * 10 - digit;
        ++places;
        --exponent;
    }

    if (places == 0)
        *buffer++ = '0';

    *buffer++ = '.';

    while (exponent < 0 && places < width) {
        *buffer++ = '0';
        --exponent;
        ++places;
    }

    while (places < width) {
        int digit = value * 10.0;
        *buffer++ = digit + '0';
        value = value * 10.0 - digit;
        ++places;
    }
    *buffer = '\0';
}

void ftoa_sci(char *buffer, double value)
{
    int exponent = 0;
    static const int width = 4;

    if (value == 0.0) {
        buffer[0] = '0';
        buffer[1] = '\0';
        return;
    }

    if (value < 0.0) {
        *buffer++ = '-';
        value = -value;
    }

    exponent = normalize(&value);

    int digit = value * 10.0;
    *buffer++ = digit + '0';
    value = value * 10.0 - digit;
    --exponent;

    *buffer++ = '.';

    for (int i = 0; i < width; i++) {
        int digit = value * 10.0;
        *buffer++ = digit + '0';
        value = value * 10.0 - digit;
    }

    *buffer++ = 'e';
    itoa(exponent, buffer, 10);
}
