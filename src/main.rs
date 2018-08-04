extern {
    fn foo_function(x:i32) -> i32;
    fn satid2no(id:&str) -> i32;
    fn satno(sys:i32, prn:i32) -> i32;
}

fn main() {
    let f = unsafe {
        foo_function(3)
    };
    let n = unsafe {
        satid2no("E04")
    };
    println!("Hello, world {}", f);
    println!("id to num {}", n);
}
