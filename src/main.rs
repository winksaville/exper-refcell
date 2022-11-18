use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

use memoffset::offset_of;

const V1_DEFAULT: i32 = 123;

struct S1 {
    name: String,
    other: RefCell<Option<Weak<Self>>>,
    v1: RefCell<i32>,
}

impl Display for S1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binding = self.other.borrow();
        if let Some(binding_as_ref) = binding.as_ref() {
            write!(
                f,
                "{}: {self:p} S1 {{ &other={:p} other={:p} &v1={:p} v1={} sc={} wc={} }}",
                self.name,
                &self.other,
                (*binding_as_ref).as_ptr(),
                &self.v1,
                self.v1.borrow(),
                Weak::strong_count(binding_as_ref),
                Weak::weak_count(binding_as_ref)
            )
        } else {
            write!(
                f,
                "{}: {self:p} S1 {{ &other={:p} other=None &v1={:p} v1={} }}",
                self.name,
                &self.other,
                &self.v1,
                self.v1.borrow()
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
    fn new(name: &str) -> Self {
        let s1 = Self {
            name: name.to_owned(),
            other: RefCell::new(None),
            v1: RefCell::new(V1_DEFAULT),
        };
        println!("new:- {s1}");

        s1
    }

    #[inline(never)]
    fn add(&self, val: i32) {
        *self.v1.borrow_mut() += val;
    }

    #[inline(never)]
    fn v1(&self) -> i32 {
        *self.v1.borrow()
    }

    #[inline(never)]
    fn other(&self) -> &Self {
        let binding = self.other.borrow();
        let other: &Self = match binding.as_ref() {
            Some(binding_as_ref) => {
                let s1_ptr = Weak::clone(binding_as_ref).as_ptr();
                unsafe { &*s1_ptr }
            }
            None => {
                println!("other:  other is None");
                panic!();
            }
        };

        other
    }

    #[inline(never)]
    fn v1_via_other(&self) -> i32 {
        self.other().v1()
    }

    #[inline(never)]
    fn add_via_other(&self, val: i32) {
        self.other().add(val);
    }
}

#[inline(never)]
fn create_two_rcs1() -> (Rc<S1>, Rc<S1>) {
    println!("create_two:+");

    // Create first and second
    let first: Rc<S1> = Rc::new(S1::new("first"));
    let second: Rc<S1> = Rc::new(S1::new("second"));

    // Connect them
    *first.other.borrow_mut() = Some(Rc::downgrade(&second));
    println!("create_two:  {first}");
    *second.other.borrow_mut() = Some(Rc::downgrade(&first));
    println!("create_two:  {second}");

    println!("create_two:-");
    (first, second)
}

#[inline(never)]
fn main() {
    println!("main:+");

    println!("offset_of S1.name {}", offset_of!(S1, name));
    println!("offset_of S1.other {}", offset_of!(S1, other));
    println!("offset_of S1.v1 {}", offset_of!(S1, v1));

    let (first, second) = create_two_rcs1();
    println!("main:  &first ={:p}  first={first}", &first);
    println!("main:  &first={:p} second={second}", &second);

    first.add(1);
    println!(" first.v1={}", first.v1());
    second.add(101);
    println!("second.v1={}", second.v1());

    println!(" first.v1_via_other().v1()={} before add_via_other", first.v1_via_other());
    first.add_via_other(1);
    println!(" first.v1_via_other().v1()={} after add_via_other", first.v1_via_other());

    println!(" second.v1_via_other().v1()={} before add_via_other", second.v1_via_other());
    second.add_via_other(1);
    println!(" second.v1_via_other().v1()={} after add_via_other", second.v1_via_other());

    println!("main:-");
}
