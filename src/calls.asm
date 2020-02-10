global asm_exit
global asm_open
global asm_write
global asm_read
global asm_panic
global _start
extern main

; rdi - exitcode
asm_exit:
    mov rax, 60
    syscall

; rdi - pathname (null-terminated string)
; rsi - flags, rdx - accessrights
; returns: file descriptor or error code (negative)
asm_open:
    mov rax, 2
    syscall
    ret

; rdi - fd, rsi - flags, rdx - accessrights
; returns: number of bytes read or error code (negative)
asm_read:
    mov rax, 0
    syscall
    ret

; rdi - fd, rsi - flags, rdx - accessrights
; returns: number of bytes read or error code (negative)
asm_write:
    mov rax, 1
    syscall
    ret

; 16-byte align the stack (prevents segfaults with SSE instructions)
; then call main
_start:
    and rsp, 0xfffffffffffffff0
    call main

; infinite loop for the rust panic handler
asm_panic:
    jmp asm_panic
