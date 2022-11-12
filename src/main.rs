use std::{cell::RefCell, mem::MaybeUninit, ptr::addr_of_mut};

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
            addr_of_mut!((*s1_mut_ptr).v1).write(123);

            s1_uninit.assume_init()
        };

        let v1 = s1.v1_via_this();
        s1
    }

    #[inline(never)]
    fn v1_via_this(&self) -> i32 {
        let v1 = self.this.borrow().v1;
        //println!("{self:p} {v1}");
        println!("{v1}");
        v1
    }
}

#[inline(never)]
fn main() {
    let s1 = S1::new();
    s1.v1_via_this();
}
