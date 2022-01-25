// build.rs

fn main() {
    cc::Build::new()
        .file("src/untrusted.c")
        .static_flag(true)
        .compile("untrusted");
    println!("cargo:rerun-if-changed=src/untrusted.c");
}
