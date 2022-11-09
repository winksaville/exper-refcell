use std::cell::RefCell;

struct S1 {
    v1: i32
}

impl<'a> S1 {
    fn new() -> RefCell<Self> {
        let this = Self {
            v1: 0,
        };

        RefCell::new(this)
    }

    fn add(this: &RefCell<Self>, val: i32) {
        this.borrow_mut().v1 += val;
    }

    fn v1(this: &RefCell<Self>) -> i32 {
        this.borrow().v1
    }
}

#[allow(unused)]
fn main() {
    let s1 = S1::new();

    S1::add(&s1, 1);
    println!("s1.v1={}", S1::v1(&s1));
}
