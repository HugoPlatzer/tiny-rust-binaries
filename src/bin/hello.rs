#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern {
    fn asm_exit(exit_code: isize);
    fn asm_write(fd: usize, buf: *const u8, nbytes: usize) -> isize;
    fn asm_panic();
}

fn exit(exit_code: isize) {
    unsafe { asm_exit(exit_code); }
}

fn print_str(s: &str) {
    unsafe { asm_write(1, s.as_ptr(), s.len()); }
}

#[no_mangle]
fn main() {
    print_str("Hello, world!\n");
    exit(0);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // When using LTO optimization, this panic handler is being
    // be optimized away if it only contains the loop.
    // This leads to undefined behavior in case of a panic.
    // We can prevent this by calling external (assembly)
    // code from the panic handler.
    unsafe { asm_panic(); }
    loop {}
}
