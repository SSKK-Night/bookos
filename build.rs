use std::error::Error;
use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    // asm.s変更時にコンパイル処理が再び走るようにする
    println!("cargo:return-if-changed=src/asm.s");
    Build::new().file("src/asm.s").compile("asm");

    Ok(());
}