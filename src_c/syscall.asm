global asm_exit
global asm_open
global asm_write
global asm_read

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

