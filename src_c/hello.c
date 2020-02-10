void asm_exit(unsigned long long r);
long long asm_write(unsigned long long fd, void *buf, unsigned long long  bytes);

void _start() {
    asm_write(1, "Hello, world!\n", 14);
    asm_exit(0);
}
