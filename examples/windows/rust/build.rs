

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=static=libbz2");
    println!("cargo:rustc-link-search=native=./");
}
