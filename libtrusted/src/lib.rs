extern "C" {
    fn do_nothing_sleep(ptr: *mut i32, ms: usize);
    fn read_i32_sleep(ptr: *mut i32, ms: usize);
    fn access_buf_sleep(ptr: *mut i32, len: usize, ms: usize);
}

pub fn safe_do_nothing(ptr: Box<i32>, ms: usize) -> Box<i32> {
    let ptr = Box::into_raw(ptr);
    unsafe {
        do_nothing_sleep(ptr, ms);
        Box::from_raw(ptr)
    }
}

pub fn safe_read_i32(ptr: &mut i32, ms: usize) {
    unsafe { read_i32_sleep(ptr as *mut i32, ms) }
}

fn safe_access_slice(slice: &mut [i32], ms: usize) {
    let ptr = slice.as_mut_ptr();
    let len = slice.len();
    unsafe { access_buf_sleep(ptr, len, ms) }
}
