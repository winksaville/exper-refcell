use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

use memoffset::offset_of;

const V1_DEFAULT: i32 = 123;

struct S1 {
    this: RefCell<Option<Weak<Self>>>,
    v1: i32,
}

impl Display for S1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binding = self.this.borrow();
        if let Some(binding_as_ref) = binding.as_ref() {
            write!(
                f,
                "{self:p} S1 {{ &this={:p} this={:p} &v1={:p} v1={} sc={} wc={} }}",
                &self.this,
                (*binding_as_ref).as_ptr(),
                &self.v1,
                self.v1,
                Weak::strong_count(binding_as_ref),
                Weak::weak_count(binding_as_ref)
            )
        } else {
            write!(
                f,
                "{self:p} S1 {{ &this={:p} this=None &v1={:p} v1={} }}",
                &self.this, &self.v1, self.v1
            )
        }
    }
}

impl Drop for S1 {
    fn drop(&mut self) {
        println!("Drop S1 {self}");
    }
}

#[allow(unused)]
impl S1 {
    #[inline(never)]
    fn new() -> Self {
        let s1 = Self {
            this: RefCell::new(None),
            v1: V1_DEFAULT,
        };
        println!("new:- {s1}");

        s1
    }

    fn add(&mut self, val: i32) {
        self.v1 += val;
    }

    #[inline(never)]
    fn v1_via_this(&self) -> i32 {
        //println!("v1_via_this:+ self={self}");
        let binding = self.this.borrow();
        let v1 = match binding.as_ref() {
            Some(binding_as_ref) => {
                let s1_ptr = Weak::clone(binding_as_ref).as_ptr();
                unsafe { (*s1_ptr).v1 }
            }
            None => {
                println!("v1_via_this:  this is None");
                panic!();
            }
        };

        //println!("v1_via_this:- v1={v1}");
        v1
    }
}

#[inline(never)]
fn main() {
    //println!("size_of S1={}", size_of::<S1>());
    //println!(
    //    "size_of RefCell<Option<Weak<S1>>>={}",
    //    size_of::<RefCell<Option<Weak<S1>>>>()
    //);
    //println!("size_of i32={}", size_of::<i32>());
    //assert!(size_of::<S1>() >= size_of::<RefCell<Option<Weak<S1>>>>() + size_of::<i32>());

    println!("offset_of S1.this {}", offset_of!(S1, this));
    println!("offset_of S1.v1 {}", offset_of!(S1, v1));

    // Allocate owner_s1
    let owner_s1: Rc<S1> = Rc::new(S1::new());
    println!("main:+ &owner_s1={:p} owner_s1={owner_s1}", &owner_s1);

    // Initialize owner_s1.this
    *owner_s1.this.borrow_mut() = Some(Rc::downgrade(&owner_s1));
    println!("main:  initialize owner_s1: {owner_s1}");

    // Create a second strong reference
    let owner2_s1 = Rc::clone(&owner_s1);
    println!("main:  create    owner2_s1: {owner2_s1}");

    // Invoke method v1_via_this
    println!("main:  owner_s1.v1_via_this={}", owner_s1.v1_via_this());

    // Invoke method p
    println!("main:- owner_s1: {owner_s1}");
}
