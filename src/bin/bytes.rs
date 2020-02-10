#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern {
    fn asm_exit(exit_code: isize);
    fn asm_write(fd: usize, buf: *const u8, nbytes: usize) -> isize;
    fn asm_read(fd: usize, buf: *const u8, nbytes: usize) -> isize;
    fn asm_panic();
}

fn exit(exit_code: isize) {
    unsafe { asm_exit(exit_code); }
}

fn print_byte(c: u8) {
    unsafe { asm_write(1, &c, 1); }
}

fn println() {
    print_byte('\n' as u8);
}

fn print_str(s: &str) {
    unsafe { asm_write(1, s.as_ptr(), s.len()); }
}

fn print_int(n: u64, base: u8, min_chars: u8, pad_char: u8) {
    let mut digits:[u8; 64] = [0; 64];
    digits[0] = '0' as u8;
    for i in 1..(min_chars as usize) {
        digits[i] = pad_char;
    }
    let mut x = n;
    let mut i = 0;
    while x > 0 {
        let value = (x % (base as u64)) as u8;
        digits[i] = if value >= 10 {
            value - 10 + ('a' as u8)
        } else {
            value + ('0' as u8)
        };
        x /= base as u64;
        i += 1;
    }
    for digit in digits.iter().filter(|&&d| d > 0).rev() {
        print_byte(*digit);
    }
}

fn read_byte(fd: usize) -> Option<u8> {
    let c: u8 = 0;
    if unsafe { asm_read(fd, &c, 1) } == 1 {
        Some(c)
    } else {
        None
    }
}

//~ fn insertion_sort<T, F>(v: &mut [T], is_less: F)
    //~ where F: Fn(&T, &T) -> bool,
//~ {
    //~ for i in 1..v.len() {
        //~ for j in (0..i).rev() {
            //~ if is_less(&v[j + 1], &v[j]) {
                //~ v.swap(j, j + 1);
            //~ }
        //~ }
    //~ }
//~ }

fn insertion_sort(stats: &mut [CountForByte]) {
    for i in 1..stats.len() {
        for j in (0..i).rev() {
            if is_less(stats[j + 1], stats[j]) {
                stats.swap(j, j + 1);
            }
        }
    }
}

#[derive(Copy, Clone)]
struct CountForByte {
    byte: u8,
    count: u64
}

// primary: ascending order on count
// secondary: ascending order on byte
fn is_less(a: CountForByte, b: CountForByte) -> bool {
    if a.count < b.count {
        true
    } else if a.count > b.count {
        false
    } else {
        a.byte < b.byte
    }
}

#[no_mangle]
fn main() {
    let mut stats = [CountForByte {byte: 0, count: 0}; 256];
    for i in 0 .. stats.len() {
        stats[i].byte = i as u8;
    }
    loop {
        match read_byte(0) {
            Some(c) => stats[c as usize].count += 1,
            None => break
        }
    }
    insertion_sort(&mut stats);
    print_stats(&stats);
    exit(0);
}

fn print_stats(stats: &[CountForByte]) {
    let total_count: u64 = stats.iter().map(|bs| bs.count).sum();
    let divisor = if total_count == 0 {1} else {total_count};
    for byte_stats in stats {
        let rel_frequency = byte_stats.count * 1000 / divisor;
        print_int(byte_stats.byte as u64, 16, 2, '0' as u8);
        print_str(": ");
        print_int(byte_stats.count as u64, 10, 10, ' ' as u8);
        print_str(" (");
        print_int(rel_frequency / 10, 10, 2, ' ' as u8);
        print_str(".");
        print_int(rel_frequency % 10, 10, 1, ' ' as u8);
        print_str("%)");
        println();
    }
    print_int(total_count, 10, 1, ' ' as u8);
    print_str(" bytes");
    println();
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
