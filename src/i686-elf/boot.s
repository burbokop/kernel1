/* Declare constants for the multiboot header. */
.set ALIGN,    1<<0             /* align loaded modules on page boundaries */
.set MEMINFO,  1<<1             /* provide memory map */
.set GRAPH,    0<<2
.set FLAGS,    ALIGN | MEMINFO | GRAPH /* this is the Multiboot 'flag' field */
.set MAGIC,    0x1BADB002       /* 'magic number' lets bootloader find the header */
.set CHECKSUM, -(MAGIC + FLAGS) /* checksum of above, to prove we are multiboot */

/*
Declare a multiboot header that marks the program as a kernel. These are magic
values that are documented in the multiboot standard. The bootloader will
search for this signature in the first 8 KiB of the kernel file, aligned at a
32-bit boundary. The signature is in its own section so the header can be
forced to be within the first 8 KiB of the kernel file.
*/
.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

.long 0
.long 0
.long 0
.long 0
.long 0

.long 0   # type (0 = linear, 1 = EGA text)
.long 320 # width
.long 200 # height
.long 8   # bpp

.section .bss
.align 16
stack_bottom:
.skip 16384 # 16 KiB
stack_top:

.section .text
.global _start
.type _start, @function
_start:
    mov $stack_top, %esp

    # pushing multiboot header
    push .multiboot + 4 * 11 # MAGIC
    push .multiboot + 4 * 10 # FLAGS
    push .multiboot + 4 * 9  # CHECKSUM

    push .multiboot + 4 * 8  # flag0
    push .multiboot + 4 * 7  # flag1
    push .multiboot + 4 * 6  # flag2
    push .multiboot + 4 * 5  # flag3
    push .multiboot + 4 * 4  # flag4

    # pushing graphics header
    push .multiboot + 4 * 3  # gflag0
    push .multiboot + 4 * 2  # width
    push .multiboot + 4 * 1  # height
    push .multiboot + 4 * 0  # bpp

    call main
    cli
1:	hlt
    jmp 1b

.size _start, . - _start
