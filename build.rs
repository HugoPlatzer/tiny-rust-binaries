use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    //let nasm_command = "/home/stud1/hplatzer/progs/nasm-2.14/nasm";
    let nasm_command = "nasm";
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new(nasm_command)
        .args(&["-f", "elf64", "src/calls.asm", "-o"])
        .arg(&format!("{}/calls.o", out_dir))
        .status().unwrap();
    Command::new("ar")
        .args(&["crus", "libcalls.a", "calls.o"])
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();
    println!("cargo:rerun-if-changed=src/calls.asm");
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=calls");
    println!("cargo:rustc-env=RUST_LOG=debug");
}

