// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    if !(Command::new("/root/.local/xPacks/@xpack-dev-tools/arm-none-eabi-gcc/13.2.1-1.1.1/.content/arm-none-eabi/bin/as").args(&["src/dispatch.S", "-mcpu=cortex-m0", "-mthumb", "-o"]).arg(&format!("{}/dispatch.o", out_dir)).status().unwrap().success()) { 
      panic!("failed");
    }
    if !(Command::new("/root/.local/xPacks/@xpack-dev-tools/arm-none-eabi-gcc/13.2.1-1.1.1/.content/arm-none-eabi/bin/ar").args(&["crus", "libasm.a", "dispatch.o"]).current_dir(&Path::new(&out_dir)).status().unwrap().success()) {
      panic!("failed");
    } 
    if !(Command::new("/root/.local/xPacks/@xpack-dev-tools/arm-none-eabi-gcc/13.2.1-1.1.1/.content/arm-none-eabi/bin/as").args(&["src/boot/asm.S", "-march=armv6s-m", "-o"]).arg(&format!("{}/asm.o", out_dir)).status().unwrap().success()) { 
      panic!("failed");
    }
    if !(Command::new("/root/.local/xPacks/@xpack-dev-tools/arm-none-eabi-gcc/13.2.1-1.1.1/.content/arm-none-eabi/bin/ar").args(&["crus", "libasm.a", "asm.o"]).current_dir(&Path::new(&out_dir)).status().unwrap().success()) {
      panic!("failed");
    } 
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=asm");
    println!("cargo:rustc-link-arg=-Tsrc/linker/pico_memmap.ld");

}
