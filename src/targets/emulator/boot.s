    .intel_syntax noprefix
    .text
    .globl _start, syscall5
_start:
    xor rbp, rbp /* xoring a value with itself = 0 */
    pop rdi /* rdi = argc */
    /* the pop instruction already added 8 to rsp */
    mov rsi, rsp /* rest of the stack as an array of char ptr */

    /* zero the las 4 bits of rsp, aligning it to 16 bytes
    same as "and rsp,0xfffffffffffffff0" because negative
    numbers are represented as
    max_unsigned_value + 1 - abs(negative_num) */
    and rsp, -16
    call main
    mov ebx, eax
    mov eax, 1
    int 0x80
    ret

syscall5:
    mov rax, rdi
    mov rdi, rsi
    mov rsi, rdx
    mov rdx, rcx
    mov r10, r8
    mov r8, r9
    syscall
    ret
