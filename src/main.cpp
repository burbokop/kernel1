#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "cstd.h"

extern "C" {
    void rust_main();
    size_t aaa();
    int memcmp(const void *s1, const void *s2, size_t n);
}

constexpr int pixelwidth = 1;
constexpr int pitch = 320;

    /* example for 320x200 VGA */
void putpixel(int pos_x, int pos_y, unsigned char VGA_COLOR)
{
    unsigned char *location = (unsigned char *) 0xA0000 + 320 * pos_y + pos_x;
    *location = VGA_COLOR;
}

static void fillrect(unsigned char *vram,
                         unsigned char r,
                         unsigned char g,
                         unsigned char b,
                         unsigned char w,
                         unsigned char h)
{
    unsigned char *where = vram;
    int i, j;

    for (i = 0; i < w; i++) {
        for (j = 0; j < h; j++) {
                //putpixel(vram, 64 + j, 64 + i, (r << 16) + (g << 8) + b);
                where[j * pixelwidth] = r;
                where[j * pixelwidth + 1] = g;
                where[j * pixelwidth + 2] = b;
        }
        where += pitch;
    }
}

int main()
{
        rust_main();
        const auto a = aaa();

        char buffer[256];

        itoa(a, buffer, 10);

        puts(buffer);
        puts("asasasasas");

        //putpixel(0, 0, VGA_COLOR_BLUE);
        //putpixel(1, 1, VGA_COLOR_CYAN);
        //putpixel(2, 2, VGA_COLOR_RED);
        //putpixel(4, 4, VGA_COLOR_MAGENTA);
        //putpixel(5, 5, VGA_COLOR_LIGHT_RED);

        return 0;
}
