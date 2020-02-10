void asm_exit(int exitcode);
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

void print_int(unsigned long long n, int base, int min_chars, char pad_char) {
    char chars[64];
    int i, digit;
    
    for (i = 0; i < 64; i++) {
        chars[i] = 0;
    }
    chars[0] = '0';
    for (i = 1; i < min_chars; i++) {
        chars[i] = pad_char;
    }
    
    i = 0;
    while (n > 0) {
        digit = n % base;
        if (digit >= 10) {
            chars[i] = 'a' + digit - 10;
        } else {
            chars[i] = '0' + digit;
        }
        n /= base;
        i++;
    }
    
    for (i = 63; i >= 0; i--) {
        if (chars[i] > 0) {
            print_byte(chars[i]);
        }
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

typedef struct {
    unsigned char byte;
    long long count;
} CountForByte;

int is_less(CountForByte a, CountForByte b) {
    if (a.count < b.count) {
        return 1;
    } else if (a.count > b.count) {
        return 0;
    } else {
        return (a.byte < b.byte);
    }
}

void insertion_sort(CountForByte *stats, int len) {
    int i, j;
    CountForByte tmp;
    
    for (i = 1; i < len; i++) {
        for (j = i - 1; j >= 0; j--) {
            if (is_less(stats[j + 1], stats[j])) {
                tmp = stats[j];
                stats[j] = stats[j + 1];
                stats[j + 1] = tmp;
            }
        }
    }
}

void print_stats(CountForByte *stats, int len) {
    int i;
    long long total_count, divisor, rel_frequency;
    
    total_count = 0;
    for (i = 0; i < len; i++) {
        total_count += stats[i].count;
    }
    if (total_count > 0) {
        divisor = total_count;
    } else {
        divisor = 1;
    }
    
    for (i = 0; i < len; i++) {
        rel_frequency = stats[i].count * 1000 / divisor;
        print_int(stats[i].byte, 16, 2, '0');
        print_str(": ", 2);
        print_int(stats[i].count, 10, 10, ' ');
        print_str(" (", 2);
        print_int(rel_frequency / 10, 10, 2, ' ');
        print_str(".", 1);
        print_int(rel_frequency % 10, 10, 1, ' ');
        print_str("%)", 2);
        println();
    }
    
    print_int(total_count, 10, 1, ' ');
    print_str(" bytes", 6);
    println();
}

void _start() {
    int i;
    int c;
    
    CountForByte stats[256];
    for (i = 0; i < 256; i++) {
        stats[i].byte = i;
        stats[i].count = 0;
    }
    while (1) {
        c = read_byte(0);
        if (c == -1) {
            break;
        }
        stats[c].count++;
    }
    
    insertion_sort(stats, 256);
    print_stats(stats, 256);
    asm_exit(0);
}
