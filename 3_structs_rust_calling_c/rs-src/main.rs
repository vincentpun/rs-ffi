use std::ffi::CString;
use std::os::raw::{c_char, c_float};

#[repr(C)]
struct Point {
    x: c_float,
    y: c_float,
}

#[repr(C)]
struct Size {
    width: c_float,
    height: c_float,
}

#[repr(C)]
struct Rect {
    origin: Point,
    size: Size,
    description: *const c_char,
}

extern "C" {
    fn rect_area(rect: *mut Rect) -> c_float;
    fn new_rect(
        x: c_float,
        y: c_float,
        width: c_float,
        height: c_float,
        description: CString,
    ) -> *mut Rect;
    fn free_rect(rect: *mut Rect);
}

fn main() {
    // This must be unsafe because there's no way for Rust to know anything. You are responsible for
    // ensuring safety.
    let description = CString::new("My rectangle!").unwrap();
    unsafe {
        let rect = new_rect(10.0, 10.0, 50.0, 100.0, description);
        let area = rect_area(rect);

        println!("Area = {area}");
        println!("x = {}", (*rect).origin.x);
        println!("y = {}", (*rect).origin.y);
        println!(
            "Description = {}",
            CString::from_raw((*rect).description as *mut c_char)
                .to_str()
                .unwrap()
        );

        free_rect(rect);

        // THIS IS TOTALLY BAD! Which is why we're in unsafe land.
        println!("{}", (*rect).origin.x);
    }
}
