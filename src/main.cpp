#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "cstd.h"

extern "C" {
    void rust_main();
    size_t aaa();
    int memcmp(const void *s1, const void *s2, size_t n);
}

struct graphics_header
{
    long flag0;
    long w;
    long h;
    long bpp;
} __attribute__ ((aligned (4)));

struct multiboot_header {
    long magic;
    long flags;
    long checksum;

    long flag0;
    long flag1;
    long flag2;
    long flag3;
    long flag4;

    graphics_header graphics;
} __attribute__ ((aligned (4)));

void* vga_addr(const graphics_header& graphics, int x, int y)
{
    if(graphics.bpp != 8) return nullptr;
    return (unsigned char*) 0xA0000
        + graphics.w * y
        + graphics.bpp * x / 8;
}

int main(multiboot_header header)
{

    puts("start");
    printf("m:%d, f:%d, c:%d, w:%d, h:%d, b:%d",
        header.magic,
        header.flags,
        header.checksum,
        header.graphics.w,
        header.graphics.h,
        header.graphics.bpp);

    rust_main();
    /*
    const auto a = aaa();

    char buffer[256];

    itoa(a, buffer, 10);

    puts(buffer);
    puts("asasasasas");
    */

    for (int y = 0; y < header.graphics.h; ++y) {
        for (int x = 0; x < header.graphics.w; ++x) {
            *(unsigned char*) vga_addr(header.graphics, x, y) = x + y;
        }
    }

    return 0;
}
