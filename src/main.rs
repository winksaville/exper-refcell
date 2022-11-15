#![feature(new_uninit)]
use core::{cell::RefCell, ptr::addr_of_mut};

const V1_DEFAULT: i32 = 123;

struct S1<'a> {
    this: RefCell<&'a Self>,
    v1: i32
}

impl<'a> Drop for S1<'a> {
    fn drop(&mut self) {
        println!("Drop S1 {self:p}");
    }
}

#[allow(unused)]
impl<'a> S1<'a> {
    #[inline(never)]
    fn new() -> Box<Self> {
        let mut s1_uninit = Box::<S1>::new_uninit();
        let s1_mut_ptr = s1_uninit.as_mut_ptr();

        let s1 = unsafe {
            let s1p = &*s1_mut_ptr;
            let this = RefCell::new(s1p);

            addr_of_mut!((*s1_mut_ptr).this).write(this);
            addr_of_mut!((*s1_mut_ptr).v1).write(V1_DEFAULT);

            s1_uninit.assume_init()
        };
        println!("s1: {:p}", &s1);

        let v1 = s1.v1_via_this();
        if v1 == V1_DEFAULT {
            // This has always succeeded!
            return s1;
        }
        panic!();
    }

    #[inline(never)]
    fn v1_via_this(&self) -> i32 {
        let v1 = self.this.borrow().v1;
        println!("self: {self:p} v1: {v1}");
        v1
    }
}

#[inline(never)]
fn main() {
    let s1 = S1::new();
    let v1 = s1.v1_via_this();
    if v1 == V1_DEFAULT {
        println!("Success");
        return;
    }
    println!("Failed");
}
