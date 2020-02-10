#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::cmp::Ordering;

extern {
    fn asm_exit(exit_code: isize);
    fn asm_open(path: *const u8, flags: usize, mode: usize) -> isize;
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

fn print_int(n: u64) {
    let mut digits:[u8; 20] = [0; 20];
    digits[0] = '0' as u8;
    let mut x = n;
    let mut i = 0;
    while x > 0 {
        digits[i] = ((x % 10) as u8) + ('0' as u8);
        x /= 10;
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

fn read_line(line: &mut [u8]) -> Option<usize> {
    let mut bytes_read = 0;
    let mut too_long = false;
    loop {
        match read_byte(1) {
            Some(c) => {
                if bytes_read >= line.len() {
                    too_long = true;
                } else {
                    line[bytes_read] = c;
                    bytes_read += 1;
                }
                if c == '\n' as u8 {
                    break;
                }
            },
            None => return None,
        }
    }
    if too_long {
        None
    } else {
        Some(bytes_read)
    }
}

fn is_digit(c: u8) -> bool {
    '0' as u8 <= c && c <= '9' as u8
}

fn read_int_line() -> Option<u32> {
    let mut line: [u8; 11] = [0; 11];
    let bytes_read = match read_line(&mut line) {
        Some(v) if v >= 2 => v,
        Some(_) => return None,
        None => return None
    };
    let mut x = 0u64;
    for i in 0..(bytes_read - 1) {
        if is_digit(line[i]) {
            x = 10 * x + ((line[i] - ('0' as u8)) as u64);
        } else {
            return None;
        }
    }
    if x <= u32::max_value() as u64 {
        Some(x as u32)
    } else {
        None
    }
}

fn random_int(min: u32, max: u32) -> u32 {
    let path = "/dev/urandom\0";
    let fd = unsafe { asm_open(path.as_ptr(), 0, 0) as usize };
    let mut random_bytes: [u8; 64] = [0; 64];
    for i in 0..random_bytes.len() {
        random_bytes[i] = read_byte(fd).unwrap();
    }
    let diff = max - min;
    let mut x = 0;
    for i in 0..random_bytes.len() {
        x = ((256 * x) + (random_bytes[i] as u64)) % (diff as u64);
    }
    min + (x as u32)
}

#[no_mangle]
fn main() {
    print_str("Guess the number!\n");

    let secret_number = random_int(1, 100 + 1);

    loop {
        print_str("Please input your guess.\n");

        let guess: u32 = match read_int_line() {
            Some(num) => num,
            None => continue,
        };

        print_str("You guessed: "); print_int(guess as u64); println();

        match guess.cmp(&secret_number) {
            Ordering::Less => print_str("Too small!\n"),
            Ordering::Greater => print_str("Too big!\n"),
            Ordering::Equal => {
                print_str("You win!\n");
                break;
            }
        }
    }
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

