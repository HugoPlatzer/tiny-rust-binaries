#!/bin/sh
set -e

build_dir="targer/self/release"
bin_dir="binaries"
minifier="./mini_elf.py"

rm -rf $bin_dir
if [ "$#" -eq 1 ] && [ "$1" = "--clean" ]; then
    cargo clean
fi
cargo xbuild --release

mkdir $bin_dir
for build_file in $(find target/self/release -maxdepth 1 -executable -type f); do
    bin_file=$bin_dir/$(basename $build_file)
    echo $build_file "->" $bin_file
    $minifier < $build_file > $bin_file
    chmod +x $bin_file
done
