use libc::{perror, MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::{mem::size_of, ptr::null_mut};

pub unsafe fn get_page_size() -> usize {
    libc::sysconf(libc::_SC_PAGE_SIZE) as usize
}

#[allow(dead_code)]
pub struct SBlock {
    size: usize,
    is_free: usize,
    prev: *mut SBlock,
    next: *mut SBlock,
}

#[allow(non_upper_case_globals)]
static mut token: *mut SBlock = null_mut();

fn align_8(x: usize) -> usize {
    ((((x) - 1) >> 3) << 3) + 8
}

#[allow(dead_code)]
pub fn malloc_t<T>(count: usize) -> *mut T {
    malloc(size_of::<T>() * count) as *mut T
}

#[allow(dead_code)]
pub fn malloc(size: usize) -> *mut u8 {
    if size == 0 {
        return 0 as *mut u8;
    }

    unsafe {
        if token.is_null() {
            token = new_block(get_page_size());
        }

        let aligned_size = align_8(size);
        let mut b = find_block(aligned_size);

        if b.is_null() {
            b = new_block(aligned_size);
        }

        if b.is_null() {
            return null_mut();
        }

        split_block(b, size);

        (*b).is_free = 0;
        token = b;

        return b.add(1) as *mut u8;
    }
}

unsafe fn new_block(mut size: usize) -> *mut SBlock {
    size = ((size / get_page_size()) + 1) * get_page_size();

    let b = libc::mmap(
        null_mut(),
        size,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANON,
        -1,
        0,
    ) as *mut SBlock;

    if b == (MAP_FAILED as *mut SBlock) {
        perror("mmap".as_ptr() as *const i8);
        return null_mut();
    } else if token.is_null() {
        (*b).prev = b;
        (*b).next = b;
    } else {
        if !(*token).next.is_null() {
            (*(*token).next).prev = b;
        }

        (*b).next = (*token).next;
        (*b).prev = token;
        (*token).next = b;
    }

    (*b).is_free = 1;
    (*b).size = size;

    return b;
}

unsafe fn find_block(size: usize) -> *mut SBlock {
    if (*token).size >= size {
        return token;
    }

    let mut b = (*token).next;

    while b != token {
        if (*b).is_free > 0 && (*b).size >= size {
            return b;
        }

        b = (*b).next;
    }

    return null_mut();
}

unsafe fn split_block(b: *mut SBlock, s: usize) {
    if (*b).size > (s + size_of::<SBlock>()) {
        let new: *mut SBlock = b.add(s);

        if !(*b).next.is_null() {
            (*(*b).next).prev = new;
        }

        (*new).next = (*b).next;
        (*new).prev = b;
        (*b).next = new;
        (*new).size = (*b).size - s;
        (*b).size = s;
        (*new).is_free = 1;
    }
}
