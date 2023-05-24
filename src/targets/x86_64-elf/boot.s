/* Declare constants for the multiboot header. */
.set ALIGN,    1<<0             /* align loaded modules on page boundaries */
.set MEMINFO,  1<<1             /* provide memory map */
.set FLAGS,    ALIGN | MEMINFO  /* this is the Multiboot 'flag' field */
.set MAGIC,        0xe85250d6       /* 'magic number' lets bootloader find the header */
.set ARCHITECTURE, 0                /* 32-bit (protected) mode of i386 */
.set HEADER_LEN,   header_end - header_start
.set CHECKSUM,     -(MAGIC + ARCHITECTURE + HEADER_LEN) /* checksum of above, to prove we are multiboot */

/*
Declare a multiboot header that marks the program as a kernel. These are magic
values that are documented in the multiboot standard. The bootloader will
search for this signature in the first 8 KiB of the kernel file, aligned at a
64-bit boundary. The signature is in its own section so the header can be
forced to be within the first 8 KiB of the kernel file.
*/
.section .multiboot2
.align 8
header_start:
.long MAGIC
.long ARCHITECTURE
.long HEADER_LEN
.long CHECKSUM
# Tags
.word 0     # Type
.word 0 #FLAGS
.long 8     # Size
header_end:

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
    call main
    cli
1:	hlt
    jmp 1b

.size _start, . - _start
