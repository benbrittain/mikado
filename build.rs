use std::error::Error;

use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    // TODO(bwb): Find a way to make this cleaner
    // Assemble `asm.s` file into a .o
    Build::new()
        .compiler("riscv64-linux-gnu-gcc")
        .flag("-march=rv32imac")
        .flag("-mabi=ilp32")
        .file("src/asm.s")
        .compile("asm");

    // Rebuild if non-rust files are changed
    println!("cargo:rerun-if-changed=src/asm.s");
    println!("cargo:rerun-if-changed=src/linker.ld");

    Ok(())
}
