// build.rs

extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/foo.c")
        .compile("foo");
    
    cc::Build::new()
        .include("src")
        .file("src/rtkcmn.c")
        .compile("rtklib");
}