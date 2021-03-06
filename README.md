# tiny-rust-binaries: Tiny amd64 ELF executables using nightly Rust

This shows a way to create executables of minimal binary size for various
toy programs written in Rust:

| **Program** | **Description** |
| -- | -- |
| hello | prints `Hello, world!` |
| guess | Guess a random number from 1 to 100 |
| bytes | Print the number of occurrences for all 256 possible byte values, sorted by ascending frequency |

Various techniques are employed to minimize the executable size:

- Not using the Rust standard library, only Rust core.
Interaction with the system happens by syscalls from assembly code
(see `src/calls.asm`).
- Using codegen options `lto, opt-level=z`.
- A linker script that discards unnecessary sections and packs all code
into one executable segment. This segment is aligned to be compatible
with the next step, which is the reason why the binaries generated by the
Rust compiler (in `target/self/release`) will segfault when trying to
directly execute them.
- A minifier program that extracts the code segment from the ELF file
generated by the Rust compiler and puts it into a minimal ELF file with
just one program header and no section headers (only 120 bytes of header
overhead).

## Sizes

Each of the toy programs was implemented in Rust (`src/bin/*.rs`),
and also in C (`src_c/*.c`), trying to make the code as similar as possible.
Both programs were subjected to the same linking and size minimization process.
These are the executable and source file sizes when building on my system:

| **Program** | **Executable size for C (bytes)** | **Executable size for Rust (bytes)** | **Rust is larger by ...** |
| --: | --: | --: | --: |
| hello | 225 | 239 | 6.2% |
| guess | 935 | 1068 | 14.2% |
| bytes | 1004 | 1183 | 17.8% |
| **Program** | **Source size for C (bytes)** | **Source size for Rust (bytes)** | **Rust is larger by ...** |
| hello | 196 | 775 | 295.4% |
| guess | 2869 | 3780 | 31.8% |
| bytes | 2916 | 3560 | 22.1% |

## Prerequisites

- amd64 system running Linux
- gcc toolchain
- nightly Rust toolchain
- cargo-xbuild
- nasm

## How to build

Just run `./build.sh`, the binaries are then found in the `binaries/` directory.
For the Rust programs they are named `binaries/rust_*`, for the C programs
`binaries/c_*`.

For the `bytes` program, you need to give a file as input to get statistics.
You could run it on its own binary file, like this:
`binaries/rust_bytes < binaries/rust_bytes`.
