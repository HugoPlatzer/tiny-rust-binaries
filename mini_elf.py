#!/usr/bin/python3
"""
Create a minimal executable ELF file:
This takes an input ELF file, extracts the .text segment
and compiles it into a new, minimal ELF file,
containing only the ELF header, one program header
and the text segment.
For the program not to segfault, the .text segment
of the original program must be linked so it works
when mapped to start at virtual address 0x400000.
"""

from sys import stdin, stdout, stderr
pdbg = lambda msg: print(msg, file=stderr)


def from_little_endian(raw_bytes):
    x = 0
    for b in raw_bytes[::-1]:
        x = 0x100 * x + b
    return x

def to_little_endian(number, nbytes):
    bytestr = bytes()
    while number > 0:
        bytestr += bytes([number % 256])
        number //= 256
    if len(bytestr) > nbytes:
        raise Exception("number too big")
    bytestr += bytes([0x00] * (nbytes - len(bytestr)))
    return bytestr


# read ELF header, parse entry point and program header start,
# number of program headers
indata = stdin.buffer.read()
elf_header = indata[:64]
entry_point_raw = elf_header[0x18:0x20]
pdbg("entry point: " + str(from_little_endian(entry_point_raw)))
phdr_offset = from_little_endian(elf_header[0x20:0x28])
pdbg("phdr offset: " + str(phdr_offset))
num_phdrs = from_little_endian(elf_header[0x38:0x3a])
pdbg("number of phdrs: " + str(num_phdrs))

# iterate over program headers, find the first one
# which has a nonzero size and the executable flag set
# this contains the boundaries of the .text segment
program_start, program_size = None, None
for i in range(num_phdrs):
    # ELF64 - program headers are 56 bytes long
    phdr_start = phdr_offset + 56 * i
    phdr_end = phdr_offset + 56 * (i + 1)
    phdr = indata[phdr_start:phdr_end]
    phdr_flags = from_little_endian(phdr[4:8])
    phdr_program_start = from_little_endian(phdr[8:16])
    phdr_program_size = from_little_endian(phdr[40:48])
    # 0x4 - read, 0x2 - write, 0x1 - exec
    if phdr_flags & 0x1 and phdr_program_size > 0:
        program_start = phdr_program_start
        program_size = phdr_program_size
        break

if program_start is None:
    raise Exception("found no suitable program header")

pdbg("program start: " + str(program_start)
     + ", program size: " + str(program_size))
# extract the .text segment
program_data = indata[program_start:program_start + program_size]


# write ELF header
data = bytes()
data += bytes([0x7f, 0x45, 0x4c, 0x46]) # magic 4 bytes
data += bytes([0x02]) # 64 bit format
data += bytes([0x01]) # little endian
data += bytes([0x01]) # current ELF version
data += bytes([0x00]) # OS ABI - System V
data += bytes([0x00]) # ABI version
data += bytes([0x00, 0x00, 0x00, 0x00,  # padding
               0x00, 0x00, 0x00])
data += bytes([0x02, 0x00]) # object file type - EXEC
data += bytes([0x3e, 0x00]) # target architecture - AMD64
data += bytes([0x01, 0x00, 0x00, 0x00]) # ELF version
data += entry_point_raw # entry point of the original program (8 bytes)
data += bytes([0x40, 0x00, 0x00, 0x00,  # program header offset
               0x00, 0x00, 0x00, 0x00]) # starts right after the
                                        # ELF header
data += bytes([0x00, 0x00, 0x00, 0x00,  # section header offset
               0x00, 0x00, 0x00, 0x00]) # there are no section headers
data += bytes([0x00, 0x00, 0x00, 0x00]) # machine specific flags
data += bytes([0x40, 0x00]) # size of this header - 64 bytes
data += bytes([0x38, 0x00]) # program header size - 56 bytes
data += bytes([0x01, 0x00]) # number of program headers - 1
data += bytes([0x40, 0x00]) # section header size - 64 bytes
data += bytes([0x00, 0x00]) # number of section headers - 0
data += bytes([0x00, 0x00]) # index of section name table

# write program header
data += bytes([0x01, 0x00, 0x00, 0x00]) # segment type - LOAD
data += bytes([0x07, 0x00, 0x00, 0x00]) # flags - RWX
data += bytes([0x78, 0x00, 0x00, 0x00,
               0x00, 0x00, 0x00, 0x00]) # offset of loadable data
                                        # within the ELF file
                                        # data starts after section
                                        # and program header (120 bytes)
data += bytes([0x78, 0x00, 0x40, 0x00,  # virtual address of
               0x00, 0x00, 0x00, 0x00]) # loadable data, aligned
                                        # with offset (mod 4096)
data += bytes([0x78, 0x00, 0x40, 0x00,  # physical address of
               0x00, 0x00, 0x00, 0x00]) # loadable data
                                        # use same as virtual
data += to_little_endian(program_size, 8) # size of loadable data
                                          # within the ELF file
                                          # same as for original program
data += to_little_endian(program_size, 8) # size of loadable data
                                          # when in memory, same value
data += bytes([0x00, 0x10, 0x00, 0x00,  # alignment of segment
               0x00, 0x00, 0x00, 0x00]) # in memory - 4KB

# finally, paste the code of the original program
data += program_data
stdout.buffer.write(data)
