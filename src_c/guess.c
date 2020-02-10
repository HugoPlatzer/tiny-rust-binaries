void asm_exit(int exitcode);
int asm_open(char *path, long long flags, long long mode);
int asm_write(long long fd, char *buf, long long nbytes);
int asm_read(long long fd, char *buf, long long nbytes);

void print_byte(char c) {
    asm_write(1, &c, 1);
}

void println() {
    print_byte('\n');
}

void print_str(char *str, int len) {
    asm_write(1, str, len);
}

void print_int(unsigned long long i) {
    char chars[20];
    int k;
    
    if (i == 0) {
        print_byte('0');
        return;
    }
    k = 0;
    while (i > 0) {
        chars[k] = '0' + (i % 10);
        i /= 10;
        k += 1;
    }
    while (k > 0) {
        print_byte(chars[k - 1]);
        k -= 1;
    }
}

int read_byte(int fd) {
    unsigned char c;
    if (asm_read(fd, &c, 1) == 1) {
        return c;
    } else {
        return -1;
    }
}

int read_line(char* line, int len) {
    int bytes_read = 0;
    int too_long = 0;
    int c;
    while (1) {
        c = read_byte(1);
        if (c == -1) {
            return -1;
        }
        if (bytes_read >= len) {
            too_long = 1;
        } else {
            line[bytes_read] = c;
            bytes_read++;
        }
        if (c == '\n') {
            break;
        }
    }
    if (too_long) {
        return -1;
    } else {
        return bytes_read;
    }
}

int is_digit(char c) {
    return ('0' <= c && c <= '9');
}

long long read_int_line() {
    char line[11];
    int bytes_read;
    int i;
    long long x;
    
    bytes_read = read_line(line, 11);
    if (bytes_read < 2) {
        return -1;
    }
    x = 0;
    for (i = 0; i < bytes_read - 1; i++) {
        if (is_digit(line[i])) {
            x = 10 * x + line[i] - '0';
        } else {
            return -1;
        }
    }
    if (x < 4294967296) {
        return x;
    } else {
        return -1;
    }
}

int random_int(int min, int max) {
    unsigned char bytes[64];
    int fd;
    int i;
    int diff;
    int x;
    
    fd = asm_open("/dev/urandom", 0, 0);
    for (i = 0; i < 64; i++) {
        bytes[i] = read_byte(fd);
    }
    diff = max - min;
    x = 0;
    for (i = 0; i < 64; i++) {
        x = (256 * x + bytes[i]) % diff;
    }
    return min + x;
}

void _start() {
    int secret_number;
    long long guess;
    
    print_str("Guess the number!\n", 18);
    
    secret_number = random_int(1, 100 + 1);
    while (1) {
        print_str("Please input your guess.\n", 25);
        guess = read_int_line();
        if (guess == -1) {
            continue;
        }
        print_str("You guessed: ", 13); print_int(guess); println();
        
        if (guess < secret_number) {
            print_str("Too small!\n", 11);
        } else if (guess > secret_number) {
            print_str("Too big!\n", 9);
        } else {
            print_str("You win!\n", 9);
            break;
        }
    }
    asm_exit(0);
}
