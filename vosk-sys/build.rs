fn main() {
    // This could be added as a #[link] attribute in the "extern" block, but
    // this way the bindings don't have to be modified manually at all
    println!("cargo:rustc-link-lib=libvosk");
}
