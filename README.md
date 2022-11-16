# Experiment with RefCell

Here using and `Rc` and `RefCell` and is inspired by
the [Rc doc example](https://doc.rust-lang.org/std/rc/#examples).
I've also `impl Display for S1` which displays the address
of S1, &S1::this, S1::this, &S1::v1, S1::v1, sc (strong_counter)
and wc (weak_counter).

# Run

Run dev
```
$ cargo run
   Compiling exper-refcell v0.1.0 (/home/wink/prgs/rust/myrepos/exper-refcell)
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/exper-refcell`
offset_of S1.this 0
offset_of S1.v1 16
new:- 0x7ffe56111518 S1 { &this=0x7ffe56111518 this=None &v1=0x7ffe56111528 v1=123 }
main:+ &owner_s1=0x7ffe56111510 owner_s1=0x564a2722faf0 S1 { &this=0x564a2722faf0 this=None &v1=0x564a2722fb00 v1=123 }
main:  initialize owner_s1: 0x564a2722faf0 S1 { &this=0x564a2722faf0 this=0x564a2722faf0 &v1=0x564a2722fb00 v1=123 sc=1 wc=1 }
main:  create    owner2_s1: 0x564a2722faf0 S1 { &this=0x564a2722faf0 this=0x564a2722faf0 &v1=0x564a2722fb00 v1=123 sc=2 wc=1 }
main:  owner_s1.v1_via_this=123
main:- owner_s1: 0x564a2722faf0 S1 { &this=0x564a2722faf0 this=0x564a2722faf0 &v1=0x564a2722fb00 v1=123 sc=2 wc=1 }
Drop S1 0x564a2722faf0 S1 { &this=0x564a2722faf0 this=0x564a2722faf0 &v1=0x564a2722fb00 v1=123 sc=0 wc=0 }
```

With `struct S1` defined as:
```
struct S1 {
    this: RefCell<Option<Weak<Self>>>,
    v1: i32,
}
```

Then from the above output the following is true:
  * "new:- .." shows that in `new()` `S1` created on the stack at `0x7ffe56111518`
  * "main:+ &ow.." `&owner_s1=0x7ffe56111510` shows that `owner_s1` is on the stack
  * "main:+ &ow.." `owner_s1=0x564a2722faf0 S1` shows that the owner_s1 data is on the heap
  * "main:  init.." `&this=0x564a2722faf0` this is at offset 0, i.e. equals `0x564a2722faf0 + 0x00`
  * "main:  init.." `this=0x564a2722faf0` points at itself, i.e. `owner_s1=0x564a2722faf0 S1`
  * "main:  init.." `&v1=0x564a2722fb00` v1 is at offset 16, i.e. equals `0x564a2722faf0 + 0x10`
  * "main:  init.." `v1=123` is `V1_DEFAULT`
  * "main:  init.." `sc=1` is strong_count
  * "main:  init.." `wc=1` is weak_count
  * "main:  crea.." `sc=2` strong_count was incremented all other values same as "main:  init.."


Here is the release run
```
$ cargo run --release
   Compiling exper-refcell v0.1.0 (/home/wink/prgs/rust/myrepos/exper-refcell)
    Finished release [optimized] target(s) in 0.16s
     Running `target/release/exper-refcell`
offset_of S1.this 0
offset_of S1.v1 16
new:- 0x7ffdea466740 S1 { &this=0x7ffdea466740 this=None &v1=0x7ffdea466750 v1=123 }
main:+ &owner_s1=0x7ffdea4666f8 owner_s1=0x55a1fbe22af0 S1 { &this=0x55a1fbe22af0 this=None &v1=0x55a1fbe22b00 v1=123 }
main:  initialize owner_s1: 0x55a1fbe22af0 S1 { &this=0x55a1fbe22af0 this=0x55a1fbe22af0 &v1=0x55a1fbe22b00 v1=123 sc=1 wc=1 }
main:  create    owner2_s1: 0x55a1fbe22af0 S1 { &this=0x55a1fbe22af0 this=0x55a1fbe22af0 &v1=0x55a1fbe22b00 v1=123 sc=2 wc=1 }
main:  owner_s1.v1_via_this=123
main:- owner_s1: 0x55a1fbe22af0 S1 { &this=0x55a1fbe22af0 this=0x55a1fbe22af0 &v1=0x55a1fbe22b00 v1=123 sc=2 wc=1 }
Drop S1 0x55a1fbe22af0 S1 { &this=0x55a1fbe22af0 this=0x55a1fbe22af0 &v1=0x55a1fbe22b00 v1=123 sc=0 wc=0 }
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
