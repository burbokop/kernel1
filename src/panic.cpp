#include "panic.h"
#include <stdbool.h>
#include <stdint.h>

#if defined(__linux__) || defined(_WIN32) || defined(WIN32)

#include <stdio.h>
#include <stdlib.h>

struct pan
{};

void __panic__(const void *context, void (*cb)(const void *, pan *))
{
    pan p;
    cb(context, &p);
    putc('\n', stderr);
    fflush(stderr);
    abort();
}

void __panic_push__(pan *p, const char *message, size_t msize)
{
    for (size_t i = 0; i < msize; ++i) {
        putc(message[i], stderr);
    }
}

static void push_until_zero(pan *p, const char *message)
{
    for (size_t i = 0; message[i]; ++i) {
        putc(message[i], stderr);
    }
}

#else

#include "fake_libc/fake_stdio.h"
#include <NotoMono-Regular.h>

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;

struct screen
{
    uint8_t *base;
    uint16_t w;
    uint16_t h;
    uint8_t bpp;
    uint8_t (*color)(uint8_t);
};

struct carriage
{
    uint8_t column;
    uint8_t row;
};

struct font
{
    uint8_t count;
    uint8_t width;
    uint8_t height;
    const uint8_t *data;
};

struct pan
{
    screen s;
    carriage c;
    font f;
};

struct bitmap
{
    uint8_t width;
    uint8_t height;
    const uint8_t *data;
};

static uint8_t *vga_addr(const screen &s, int x, int y)
{
    if (x >= s.w || y >= s.h || x < 0 || y < 0) {
        return nullptr;
    }

    if (s.bpp != 8)
        return nullptr;
    return (uint8_t *) s.base + s.w * y + s.bpp * x / 8;
}

static void draw_bitmap(screen &s, const bitmap &bitmap, int dst_x, int dst_y)
{
    for (size_t y = 0; y < bitmap.height; ++y) {
        for (size_t x = 0; x < bitmap.width; ++x) {
            if (auto addr = vga_addr(s, x + dst_x, y + dst_y)) {
                *addr = s.color(bitmap.data[x + y * bitmap.width]);
            }
        }
    }
}

static void clear_screen(screen &s)
{
    for (int y = 0; y < s.h; ++y) {
        for (int x = 0; x < s.w; ++x) {
            *vga_addr(s, x, y) = (x % 8 + y % 8);
        }
    }
}

static void draw_char(screen &s, int column, int row, const font &f, char c)
{
    draw_bitmap(s,
                bitmap{.width = f.width,
                       .height = f.height,
                       .data = f.data + c * f.width * f.height},
                column * f.width,
                row * f.height);
}

static void append_char_to_end(screen &s, carriage &car, const font &f, char c)
{
    const uint8_t text_width = s.w / f.width;
    const uint8_t text_height = s.h / f.height;

    if (c == '\n') {
        car.column = 0;
        ++car.row;
    } else {
        draw_char(s, car.column, car.row, f, c);
        if (++car.column >= text_width) {
            car.column = 0;
            if (++car.row >= text_height) {
                car.row = 0;
            }
        }
    }
}

void __panic__(const void *context, void (*cb)(const void *, pan *))
{    
    /// VGA 320x200 256color Graphical mode (Only in processor's "Protected Mode")
    screen screen = {.base = reinterpret_cast<uint8_t *>(0xA0000),
                     .w = 320,
                     .h = 200,
                     .bpp = 8,
                     .color = [](uint8_t g) -> uint8_t { return g / 16 + 16; }};

    carriage carriage = {.column = 0, .row = 0};

    const font font = {.count = NotoMono_Regular_count,
                       .width = NotoMono_Regular_width,
                       .height = NotoMono_Regular_height,
                       .data = NotoMono_Regular};

    clear_screen(screen);
    putchar(2);
    auto p = pan{
        .s = screen,
        .c = carriage,
        .f = font,
    };
    cb(context, &p);
    putchar('\n');
}

void __panic_push__(pan *p, const char *message, size_t msize)
{
    for (size_t i = 0; i < msize; ++i) {
        putchar(message[i]);
        append_char_to_end(p->s, p->c, p->f, message[i]);
    }
}

static void push_until_zero(pan *p, const char *message)
{
    for (size_t i = 0; message[i]; ++i) {
        putchar(message[i]);
        append_char_to_end(p->s, p->c, p->f, message[i]);
    }
}

#endif

void panic(const char *message)
{
    __panic__(message, [](const void *m, pan *p) {
        push_until_zero(p, "paniced at '");
        push_until_zero(p, reinterpret_cast<const char *>(m));
        push_until_zero(p, "'");
    });
    while (true) {
    }
}
