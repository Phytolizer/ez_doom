fn main() {
    if cfg!(windows) {
        println!("Adding SDL2 to dependencies");
        println!("cargo:rustc-link-search=C:\\dev\\libs\\SDL2\\lib\\x64");
    }
}
