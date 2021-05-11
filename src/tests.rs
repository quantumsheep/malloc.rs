#[allow(unused_imports)]
use crate::malloc::{malloc, malloc_t};
#[allow(unused_imports)]
use std::mem::size_of;

#[test]
fn malloc_should_not_segfault() {
    let ptr = malloc(4usize * size_of::<i8>()) as *mut i8;

    unsafe {
        *ptr.add(0) = 'a' as i8;
        *ptr.add(1) = 'b' as i8;
        *ptr.add(2) = 'c' as i8;
        *ptr.add(3) = 0 as i8;

        libc::puts(ptr as *const i8);
    }
}

#[test]
fn malloc_t_should_not_segfault() {
    let ptr = malloc_t::<i8>(4);

    unsafe {
        *ptr.add(0) = 'a' as i8;
        *ptr.add(1) = 'b' as i8;
        *ptr.add(2) = 'c' as i8;
        *ptr.add(3) = 0 as i8;

        libc::puts(ptr as *const i8);
    }
}
