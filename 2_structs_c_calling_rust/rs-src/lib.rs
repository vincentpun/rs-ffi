use std::os::raw::c_char;

#[repr(C)]
pub struct Point {
    x: f32,
    y: f32,
}

#[repr(C)]
pub struct Size {
    width: f32,
    height: f32,
}

#[repr(C)]
pub struct Rect {
    origin: Point,
    size: Size,
    description: *const c_char,
}

#[no_mangle]
unsafe extern "C" fn rect_area(rect: *const Rect) -> f32 {
    (*rect).size.width * (*rect).size.height
}

#[no_mangle]
extern "C" fn new_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    description: *const c_char,
) -> *mut Rect {
    Box::into_raw(Box::new(Rect {
        origin: Point { x, y },
        size: Size { width, height },
        description,
    }))
}

#[no_mangle]
extern "C" fn free_rect(p: *mut Rect) {
    if p.is_null() {
        return;
    }

    // Dropping will free the memory.
    unsafe {
        Box::from_raw(p);
    }
}
