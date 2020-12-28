fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-link-search=lib");
        println!("cargo:rustc-link-search=C:/dev/libs/SDL2_all/lib/x64");
    }
}
