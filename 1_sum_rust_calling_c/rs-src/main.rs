use std::os::raw::c_int;

extern "C" {
    fn sum(x: c_int, y: c_int) -> c_int;
}

fn main() {
    let result = unsafe { sum(5, 3) };
    println!("Result: {result}");
}
