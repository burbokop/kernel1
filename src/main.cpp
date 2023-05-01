#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "fake_libc/fake_stdlib.h"
#include "panic.h"

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

//uint32_t cr0()
//{
//    volatile uint32_t result = 0;
//    asm("mov %%cr0, %%eax\n"
//        "mov %%eax, %0"
//        : "=r"(result)
//        :);
//    return result;
//}

#if defined(__linux__) || defined(_WIN32) || defined(WIN32)
int main()
{
    multiboot_header header;
#else
int main(multiboot_header header)
{
#endif
    //const auto a = cr0();
    //bool protected_mode = (a & 1);

    //if (protected_mode) {
    //    puts("Protected mode");
    //} else {
    //    puts("Real mode");
    //}

    //void *m1 = malloc(16);
    //void *m2 = malloc(8);

    //free(m1);
    //free(m2);

    //puts("start1");

    //printf("Protected mode: %b\n", protected_mode);
    //puts("start2");

    //const auto *a = "gogadoda";
    //__panic__(a, [](const void *a, panic *p) { __panic_push__(p, (const char *) a, 8); });

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

    //memcmp(NULL, NULL, 0);
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
