#!/bin/sh

ASMFILE="syscall.asm"
CFILES="hello.c guess.c bytes.c"
LINKSCRIPT="../link-script"
MINIFIER="../mini_elf.py"

rm -rf build
mkdir build
objfile_asm=build/$(basename $ASMFILE .asm).o
nasm -f elf64 -o $objfile_asm $ASMFILE
for cfile in $CFILES; do
    execfile=build/$(basename $cfile .c)
    objfile=build/$(basename $cfile .c).o
    gcc -Os -c -fomit-frame-pointer -fno-exceptions \
        -fno-asynchronous-unwind-tables -o $objfile $cfile
    ld -T $LINKSCRIPT --omagic -o $execfile $objfile $objfile_asm
done
