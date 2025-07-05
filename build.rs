fn main() {
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rustc-link-arg=-Tmemory.x");
}
