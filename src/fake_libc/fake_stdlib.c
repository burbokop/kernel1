#include "fake_stdlib.h"

#include <dumb/alloc.h>
#include <limits.h>
#include <stdbool.h>
#include <stdint.h>

void panic(const char *message);

size_t strlen(const char *str)
{
    size_t len = 0;
    while (str[len])
        len++;
    return len;
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
    if (isNegative) {
        str[i++] = '-';
    }

    str[i] = '\0'; // Append string terminator

    // Reverse the string
    reverse(str, i);

    return str;
}
extern int printf(char const *fmt, ...);

static size_t aaaa = 0;
static size_t bbbb = 0;

float fminf(float x, float y)
{
    float a = x < y ? x : y;
    if (aaaa > 90)
        printf("fminf[%d](%f, %f) = %f\n", aaaa++, x, y, a);
    return a;
}

//double fmin(double x, double y) {
//    return x < y ? x : y;
//}

//long double fminl(long double x, long double y)
//{
//    return x < y ? x : y;
//}

float fmaxf(float x, float y)
{
    float a = x > y ? x : y;
    if (aaaa > 90)
        printf("fmaxf[%d](%f, %f) = %f\n", aaaa++, x, y, a);
    return a;
}

//double fmax(double x, double y) {
//    return x > y ? x : y;
//}

//long double fmaxl(long double x, long double y) {
//    return x > y ? x : y;
//}

float floorf(float num) {
    if (num >= ((double) LLONG_MAX - 1) || num <= LLONG_MIN || num != num) {
        // TODO handle large values, infinities and nan
        return num;
    }
    long long n = (long long)num;
    float d = (float)n;
    return (d == num || num >= 0)
        ? d
        : (d - 1);
}

//double floor(double num) {
//    if (num >= ((double) LLONG_MAX - 1) || num <= LLONG_MIN || num != num) {
//        // TODO handle large values, infinities and nan
//        return num;
//    }
//    long long n = (long long)num;
//    double d = (double)n;
//    return (d == num || num >= 0)
//        ? d
//        : (d - 1);
//}

//long double floorl(long double num) {
//    if (num >= LLONG_MAX || num <= LLONG_MIN || num != num) {
//        // TODO handle large values, infinities and nan
//        return num;
//    }
//    long long n = (long long)num;
//    long double d = (long double)n;
//    return (d == num || num >= 0)
//        ? d
//        : (d - 1);
//}

float fmodf(float x, float y)
{
    float a = (x - y * floorf(x / y));

    printf("fmodf(%f, %f) = %f", x, y, a);
    return a;
}

//double fmod(double x, double y) {
//    return (x - y * floor(x / y));
//}
//
//long double fmodl(long double x, long double y)
//{
//    return (x - y * floorl(x / y));
//}

//void *memcpy(void *dest, const void *src, size_t n)
//{
//    for (size_t i = 0; i < n; i++) {
//        ((char *) dest)[i] = ((char *) src)[i];
//    }
//    return dest;
//}

void *memmove(void *dest, const void *src, size_t n)
{
    char *p_dest = (char *) dest;
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

extern void *memcpy(void *dest, const void *src, size_t n)
{
    return dumb_memcpy(dest, src, n);
}

DUMB_NEW_ALLOCATOR_IN_DATA_SECTION(heap, 1024 * 1024, 1024, panic)

void *malloc(size_t size)
{
    dumb_alloc alloc = heap();
    return dumb_malloc(&alloc, size);
}

void *calloc(size_t num, size_t size)
{
    const size_t mem_size = num * size;
    dumb_alloc alloc = heap();
    return memset(dumb_malloc(&alloc, mem_size), 0, mem_size);
}

void free(void *ptr)
{
    dumb_alloc alloc = heap();
    return dumb_free(&alloc, ptr);
}

void *realloc(void *ptr, size_t new_size)
{
    dumb_alloc alloc = heap();
    return dumb_realloc(&alloc, ptr, new_size);
}

/*
static uint8_t heap[1024 * 1024 * 1024];
static uint8_t *const out_of_heap = (heap + sizeof(heap) + 1);

typedef struct
{
    uint8_t *begin;
    uint8_t *end;
} block;

static block blocks[1024];
static bool heap_inited = false;

static void memdump(const char *comment);

void *malloc(size_t size)
{
    if (size == 0) {
        //panic("fake malloc: error: trying to allocate zero size");
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
                panic("fake malloc: error: heap expired");
            }
            return blocks[i].begin;
        }
    }
    panic("fake malloc: error: max block count exited");
    return NULL;
}

//void *realloc( void *ptr, size_t new_size );

static void move_block_to_left(uint8_t *dst, block *b)
{
    if (b->begin > b->end) {
        panic("fake free: error: block.begin > block.end");
    }
    const size_t size = b->end - b->begin;
    for (size_t o = 0; o < size; ++o) {
        dst[o] = b->begin[o];
    }
    b->begin = dst;
    b->end = dst + size;
}

static void __dealloc__(block *b)
{
    b->end = out_of_heap; // make dealloced
    memdump("before shift");
    block *prev_alloced_block = NULL;
    for (size_t i = 0; (i < sizeof(blocks) / sizeof(blocks[0])) && blocks[i].end; ++i) {
        if (blocks[i].end != out_of_heap) {
            if (prev_alloced_block && blocks[i].begin != prev_alloced_block->end) {
                move_block_to_left(prev_alloced_block->end, &blocks[i]);
            } else if (blocks[i].begin != heap) {
                move_block_to_left(heap, &blocks[i]);
            }
            prev_alloced_block = &blocks[i];
        }
    }
    memdump("after shift");
}

void free(void *ptr)
{
    for (size_t i = 0; i < sizeof(blocks) / sizeof(blocks[0]); ++i) {
        if (blocks[i].begin == ptr) {
            __dealloc__(&blocks[i]);
            return;
        }
    }
}

void *realloc(void *ptr, size_t new_size)
{
    if (!ptr) {
        return malloc(new_size);
    }
    for (size_t i = 0; i < sizeof(blocks) / sizeof(blocks[0]); ++i) {
        if (blocks[i].begin == ptr) {
            if (blocks[i].end == NULL) {
                panic("fake realloc: error: ptr is not allocated");
            } else if (blocks[i].end == out_of_heap) {
                panic("fake realloc: error: ptr already freed");
            }

            size_t size = blocks[i].begin - blocks[i].end;
            if (new_size > size) {
                void *new_memory = malloc(new_size);
                memcpy(new_memory, ptr, size);
                __dealloc__(&blocks[i]);
                return new_memory;
            } else {
                return ptr;
            }
        }
    }
    panic("fake realloc: error: block corresponding to ptr not found");
    return NULL;
}

#include <stdio.h>
static void memdump(const char *comment)
{
    printf("memdump (%s)\n", comment);
    for (size_t i = 0; i < sizeof(blocks) / sizeof(blocks[0]); ++i) {
        printf("  block: %ld\n", i);
        if (blocks[i].end == out_of_heap) {
            printf("    state: deallocated\n");
        } else if (blocks[i].end == NULL) {
            printf("    state: ready to alloc\n");
        } else {
            printf("    state: allocated\n");
            printf("    begin: %p\n", blocks[i].begin);
            printf("    end: %p\n", blocks[i].end);
            printf("    size: %ld\n", blocks[i].end - blocks[i].begin);
        }
        if (!blocks[i].end)
            break;
    }
}
*/
