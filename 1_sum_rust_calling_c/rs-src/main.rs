use std::os::raw::c_int;

extern "C" {
    fn sum(x: c_int, y: c_int) -> c_int;
}

fn main() {
    // This must be unsafe because there's no way for Rust to know anything. You are responsible for
    // ensuring safety.
    let result = unsafe { sum(5, 3) };
    println!("Result: {result}");
}
