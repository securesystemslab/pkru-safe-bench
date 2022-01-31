#![feature(plugin, custom_attribute)]
#![feature(macros_in_extern)]
#![plugin(mpk_protector)]
#![mpk_protector]

extern "C" {
    fn do_nothing_sleep(ptr: *mut i32, ns: usize);
    fn read_i32_sleep(ptr: *mut i32, ns: usize);
    fn access_buf_sleep(ptr: *mut i32, len: usize, ns: usize);
    fn callback(ptr: *mut i32, ns: usize, func: unsafe extern "C" fn(&mut i32, usize));
    fn access_vec(ptr: *mut i32, len: usize, ns: usize);
    fn access_box_vec(ptr: *mut *mut i32, len: usize, ns: usize);
}

pub fn safe_do_nothing(ptr: Box<i32>, ns: usize) -> Box<i32> {
    let ptr = Box::into_raw(ptr);
    unsafe {
        do_nothing_sleep(ptr, ns);
        Box::from_raw(ptr)
    }
}

pub fn safe_read_i32(ptr: &mut i32, ns: usize) {
    unsafe { read_i32_sleep(ptr as *mut i32, ns) }
}

fn safe_access_slice(slice: &mut [i32], ns: usize) {
    let ptr = slice.as_mut_ptr();
    let len = slice.len();
    unsafe { access_buf_sleep(ptr, len, ns) }
}

extern "C" fn callable(ptr: &mut i32, ns: usize) {
    if !(*ptr > 1) {
        panic!("Ptr !> 1");
    }
}

pub fn safe_callback(ptr: &mut i32, ns: usize) {
    unsafe { callback(ptr, ns, callable) }
}

pub fn safe_access_vec(ptr: *mut i32, len: usize, ns: usize) {
    unsafe { access_vec(ptr, len, ns); }
}

pub fn safe_access_box_vec(ptr: *mut *mut i32, len: usize, ns: usize) {
    unsafe { access_box_vec(ptr, len, ns); }
}
