include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    // This must be unsafe because there's no way for Rust to know anything. You are responsible for
    // ensuring safety.
    let result = unsafe { sum(5, 3) };
    println!("Result: {result}");
}
