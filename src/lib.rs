use std::ptr;

#[no_mangle]
pub unsafe extern fn test(src: *mut Vec<u8>, dst: *mut Vec<u8>) {
    ptr::copy(src, dst, 100);
}