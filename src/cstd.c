#include "cstd.h"

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <limits.h>

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
    if (__term_inited__) return;

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

size_t strlen(const char *str)
{
    size_t len = 0;
    while (str[len])
        len++;
    return len;
}

static int terminal_writestring(const char *data)
{
    return terminal_write(data, strlen(data));
}

static void reverse(char str[], int length)
{
    int start = 0;
    int end = length - 1;
    while (start < end) {
        char temp = str[start];
        str[start] = str[end];
        str[end] = temp;
        end--;
        start++;
    }
}

char *itoa(int num, char *str, int base)
{
    int i = 0;
    bool isNegative = false;

    /* Handle 0 explicitly, otherwise empty string is
     * printed for 0 */
    if (num == 0) {
        str[i++] = '0';
        str[i] = '\0';
        return str;
    }

    // In standard itoa(), negative numbers are handled
    // only with base 10. Otherwise numbers are
    // considered unsigned.
    if (num < 0 && base == 10) {
        isNegative = true;
        num = -num;
    }

    // Process individual digits
    while (num != 0) {
        int rem = num % base;
        str[i++] = (rem > 9) ? (rem - 10) + 'a' : rem + '0';
        num = num / base;
    }

    // If number is negative, append '-'
    if (isNegative)
        str[i++] = '-';

    str[i] = '\0'; // Append string terminator

    // Reverse the string
    reverse(str, i);

    return str;
}

float fminf(float x, float y) {
    return x < y ? x : y;
}

double fmin(double x, double y) {
    return x < y ? x : y;
}

long double fminl(long double x, long double y) {
    return x < y ? x : y;
}

float fmaxf(float x, float y) {
    return x > y ? x : y;
}

double fmax(double x, double y) {
    return x > y ? x : y;
}

long double fmaxl(long double x, long double y) {
    return x > y ? x : y;
}

float floorf(float num) {
    if (num >= LLONG_MAX || num <= LLONG_MIN || num != num) {
        // TODO handle large values, infinities and nan
        return num;
    }
    long long n = (long long)num;
    float d = (float)n;
    return (d == num || num >= 0)
        ? d
        : (d - 1);
}

double floor(double num) {
    if (num >= LLONG_MAX || num <= LLONG_MIN || num != num) {
        // TODO handle large values, infinities and nan
        return num;
    }
    long long n = (long long)num;
    double d = (double)n;
    return (d == num || num >= 0)
        ? d
        : (d - 1);
}

long double floorl(long double num) {
    if (num >= LLONG_MAX || num <= LLONG_MIN || num != num) {
        // TODO handle large values, infinities and nan
        return num;
    }
    long long n = (long long)num;
    long double d = (long double)n;
    return (d == num || num >= 0)
        ? d
        : (d - 1);
}

float fmodf(float x, float y) {
    return (x - y * floorf(x / y));
}

double fmod(double x, double y) {
    return (x - y * floor(x / y));
}

long double fmodl(long double x, long double y) {
    return (x - y * floorl(x / y));
}

void *memcpy(void *dest, const void *src, size_t n)
{
    for (size_t i = 0; i < n; i++) {
        ((char *) dest)[i] = ((char *) src)[i];
    }
    return dest;
}

void *memmove(void *dest, const void *src, size_t n)
{
    char *p_dest = (char*) dest;
    const char *p_src =( const char*)src;
    char *tmp  = (char*) malloc(sizeof(char) * n);
    if(NULL == tmp) {
        return NULL;
    } else {
        unsigned int i = 0;
        for(i = 0; i < n ; ++i) {
            *(tmp + i) = *(p_src + i);
        }
        for(i =0 ; i < n ; ++i) {
            *(p_dest + i) = *(tmp + i);
        }
        free(tmp);
    }
    return dest;
}

int memcmp(const void *s1, const void *s2, size_t n)
{
    unsigned char u1, u2;

    for (; n--; s1++, s2++) {
        u1 = *(unsigned char *) s1;
        u2 = *(unsigned char *) s2;
        if (u1 != u2) {
            return (u1 - u2);
        }
    }
    return 0;
}

void *memset(void *s, int c, size_t len)
{
    unsigned char *p = s;
    while (len--) {
        *p++ = (unsigned char) c;
    }
    return s;
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

static uint8_t heap[1024 * 1024];
static uint8_t *const out_of_heap = (heap + sizeof(heap) + 1);

typedef struct
{
    uint8_t *begin;
    uint8_t *end;
} block;

static block blocks[1024];
static bool heap_inited = false;

static void memdump()
{
    printf("memdump\n");
    for (size_t i = 0; i < sizeof(blocks) / sizeof(blocks[0]); ++i) {
        printf("  block: %d\n", i);
        if (blocks[i].end == out_of_heap) {
            printf("    state: deallocated\n");
        } else if (blocks[i].end == NULL) {
            printf("    state: ready to alloc\n");
        } else {
            printf("    state: allocated\n");
            printf("    begin: %d\n", blocks[i].begin);
            printf("    end: %d\n", blocks[i].end);
            printf("    size: %d\n", blocks[i].end - blocks[i].begin);
        }
        if (!blocks[i].end)
            break;
    }
}

void *malloc(size_t size)
{
    if (size == 0) {
        puts("malloc: error: trying to allocate zero size");
        return NULL;
    }

    if (!heap_inited) {
        for (size_t i = 0; i < sizeof(blocks) / sizeof(blocks[0]); ++i) {
            blocks[i].end = NULL;
        }
        heap_inited = true;
    }

    for (size_t i = 0; i < sizeof(blocks) / sizeof(blocks[0]); ++i) {
        if (blocks[i].end == NULL) {
            if (i == 0) {
                blocks[i].begin = heap;
            } else {
                blocks[i].begin = blocks[i - 1].end;
            }
            blocks[i].end = blocks[i].begin + size;

            if (blocks[i].end > (heap + sizeof(heap))) {
                puts("malloc: error: heap expired");
            }
            return blocks[i].begin;
        }
    }
    puts("malloc: error: max block count exited");
    return NULL;
}

void free(void *ptr)
{
    for (size_t i = 0; i < sizeof(blocks) / sizeof(blocks[0]); ++i) {
        if (blocks[i].begin == ptr) {
            blocks[i].end = out_of_heap; // make dealloced
        }
    }
    // todo shift dealloced blocks
}
