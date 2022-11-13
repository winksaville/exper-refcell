#![no_main]

//extern crate libc;

use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;

//use core::panic;
use core::{cell::RefCell, mem::MaybeUninit, ptr::addr_of_mut};

const V1_DEFAULT: i32 = 123;

struct S1<'a> {
    this: RefCell<&'a Self>,
    v1: i32
}

#[allow(unused)]
impl<'a> S1<'a> {
    #[inline(never)]
    fn new() -> Self {
        let mut s1_uninit = MaybeUninit::<S1>::uninit();
        let s1_mut_ptr = s1_uninit.as_mut_ptr();
        //println!("s1_mut_ptr={:p}", s1_mut_ptr);

        let s1 = unsafe {
            let s1p = &*s1_mut_ptr;
            //println!("s1p={:p}", s1p);
            let x = RefCell::new(s1p);

            addr_of_mut!((*s1_mut_ptr).this).write(x);
            addr_of_mut!((*s1_mut_ptr).v1).write(V1_DEFAULT);
            s1_uninit.assume_init()
        };

        let v1 = s1.v1_via_this();
        if v1 == V1_DEFAULT {
            return s1;
        }
        panic!();
    }

    #[inline(never)]
    fn v1_via_this(&self) -> i32 {
        let v1 = self.this.borrow().v1;
        //println!("{self:p} {v1}");
        //println!("{v1}");
        v1
    }
}

#[inline(never)]
fn stdout() -> File {
    unsafe { File::from_raw_fd(1) }
}

#[inline(never)]
fn print_buf(buf: &[u8]) {
    let mut stdout = stdout();
    stdout.write(buf).unwrap();
}

#[inline(never)]
#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) {
    let s1 = S1::new();
    let v1 = s1.v1_via_this();
    if v1 == V1_DEFAULT {
        print_buf(b"Success\n\0");
        return
    }
    print_buf(b"Failed\n\0");
}
