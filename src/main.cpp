#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "cstd.h"

struct graphics_header
{
    long flag0;
    long w;
    long h;
    long bpp;
} __attribute__((aligned(4)));

struct multiboot_header
{
    long magic;
    long flags;
    long checksum;

    long flag0;
    long flag1;
    long flag2;
    long flag3;
    long flag4;

    graphics_header graphics;
} __attribute__((aligned(4)));

extern "C" {
void rust_main(multiboot_header header);
size_t aaa();
int memcmp(const void *s1, const void *s2, size_t n);

void __panic__(const char *, size_t);
}

void *vga_addr(const graphics_header &graphics, int x, int y)
{
    if(graphics.bpp != 8) return nullptr;
    return (unsigned char *) 0xA0000 + graphics.w * y + graphics.bpp * x / 8;
}

void set_bit(uint8_t *addr, size_t offset, bool bit)
{
    auto addr_mod = offset % 8;
    auto addr_floor = offset - addr_mod;
    if (bit) {
        *(addr + addr_floor) |= uint8_t(1) << addr_mod;
    } else {
        *(addr + addr_floor) &= uint8_t(0) << addr_mod;
    }
}

int main(multiboot_header header)
{
    //puts("start");
    //printf("m:%d, f:%d, c:%d, w:%d, h:%d, b:%d\n",
    //       header.magic,
    //       header.flags,
    //       header.checksum,
    //       header.graphics.w,
    //       header.graphics.h,
    //       header.graphics.bpp);

    //const char *aaa = "gogadoda";
    //const auto a = __rm_panic__(aaa, 8);

    //printf("a: %d\n", a);

    //const char a[] = "gogadoda-aboba-1234567890-ryba\0";

    //__panic__(a, sizeof(a) - 1);
    //double x = 2;
    //for (size_t i = 0; i < 10000; ++i) {
    //    for (size_t j = 0; j < 2000; ++j) {
    //        x *= x * x * x * x / 10000.;
    //        floor(x + x * x - x);
    //    }
    //    //x /= 1000000.;
    //    //floor(x + x * x - x);
    //}
    rust_main(header);

    memcmp(NULL, NULL, 0);
    /*
    const auto a = aaa();


    char buffer[256];

    itoa(a, buffer, 10);

    puts(buffer);
    puts("asasasasas");
    */

    //int c = 0;

    //for (int y = 0; y < header.graphics.h; ++y) {
    //    for (int x = 0; x < header.graphics.w; ++x) {
    //        //set_bit((uint8_t *) 0xA0000, header.graphics.w * y + x, y < x);
    //        *(unsigned char *) vga_addr(header.graphics, x, y) = x + y;

    //        if (c >= 256) {
    //            c = 0;
    //        }
    //    }
    //}

    //struct A
    //{
    //    const char *data;
    //    size_t i;
    //};
    //A aa = {.data = a, .i = 0};

    //__panic__(&aa, [](void *ctx) {
    //    auto aa = (A *) ctx;
    //    return aa->data[aa->i++];
    //});

    return 0;
}
