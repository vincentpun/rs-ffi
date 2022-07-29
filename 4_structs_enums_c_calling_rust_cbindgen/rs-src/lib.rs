use std::ffi::CString;
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
    rect_type: RectType,
}

#[repr(C)]
pub enum RectType {
    Normal,
    Rounded(f32),
}

#[no_mangle]
pub unsafe extern "C" fn rect_area(rect: *const Rect) -> f32 {
    (*rect).size.width * (*rect).size.height
}

#[no_mangle]
pub unsafe extern "C" fn rect_type(rect: *const Rect) -> *mut c_char {
    let rect = &*rect;

    // This actually is a memory leak.
    match rect.rect_type {
        RectType::Normal => CString::new("Normal rect").unwrap().into_raw(),
        RectType::Rounded(radius) => CString::new(format!("Rounded with radius {radius}"))
            .unwrap()
            .into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn new_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    description: *const c_char,
    rect_type: RectType,
) -> *mut Rect {
    Box::into_raw(Box::new(Rect {
        origin: Point { x, y },
        size: Size { width, height },
        description,
        rect_type,
    }))
}

#[no_mangle]
pub extern "C" fn free_rect(p: *mut Rect) {
    if p.is_null() {
        return;
    }

    // Dropping will free the memory.
    unsafe {
        Box::from_raw(p);
    }
}
