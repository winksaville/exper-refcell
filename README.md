# Experiment with RefCell

Here using and `Rc` and `RefCell` and is inspired by
the [Rc doc example](https://doc.rust-lang.org/std/rc/#examples).
I've also `impl Display for S1` which displays the address
of S1, &S1::this, S1::this, &S1::v1, S1::v1, sc (strong_counter)
and wc (weak_counter).

This additionally shows creating multiple S1's with mutable (this, v1)
and immutable fields (name).

# Run

Run dev
```
$ cargo run
   Compiling exper-refcell v0.1.0 (/home/wink/prgs/rust/myrepos/exper-refcell)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/exper-refcell`
main:+
offset_of S1.name 0
offset_of S1.other 24
offset_of S1.v1 40
create_two:+
new:- first: 0x7ffc2f105128 S1 { &other=0x7ffc2f105140 other=None &v1=0x7ffc2f105150 v1=123 }
new:- second: 0x7ffc2f105168 S1 { &other=0x7ffc2f105180 other=None &v1=0x7ffc2f105190 v1=123 }
create_two:  first: 0x55d6a9067bd0 S1 { &other=0x55d6a9067be8 other=0x55d6a9067c40 &v1=0x55d6a9067bf8 v1=123 sc=1 wc=1 }
create_two:  second: 0x55d6a9067c40 S1 { &other=0x55d6a9067c58 other=0x55d6a9067bd0 &v1=0x55d6a9067c68 v1=123 sc=1 wc=1 }
create_two:-
main:  &first =0x7ffc2f105628  first=first: 0x55d6a9067bd0 S1 { &other=0x55d6a9067be8 other=0x55d6a9067c40 &v1=0x55d6a9067bf8 v1=123 sc=1 wc=1 }
main:  &first=0x7ffc2f105630 second=second: 0x55d6a9067c40 S1 { &other=0x55d6a9067c58 other=0x55d6a9067bd0 &v1=0x55d6a9067c68 v1=123 sc=1 wc=1 }
 first.v1=124
second.v1=224
 first.v1_via_other().v1()=224 before add_via_other
 first.v1_via_other().v1()=225 after add_via_other
 second.v1_via_other().v1()=124 before add_via_other
 second.v1_via_other().v1()=125 after add_via_other
main:-
Drop S1 second: 0x55d6a9067c40 S1 { &other=0x55d6a9067c58 other=0x55d6a9067bd0 &v1=0x55d6a9067c68 v1=225 sc=1 wc=1 }
Drop S1 first: 0x55d6a9067bd0 S1 { &other=0x55d6a9067be8 other=0x55d6a9067c40 &v1=0x55d6a9067bf8 v1=125 sc=0 wc=0 }
```

Here is the release run
```
$ cargo run --release
   Compiling exper-refcell v0.1.0 (/home/wink/prgs/rust/myrepos/exper-refcell)
    Finished release [optimized] target(s) in 0.20s
     Running `target/release/exper-refcell`
main:+
offset_of S1.name 0
offset_of S1.other 24
offset_of S1.v1 40
create_two:+
new:- first: 0x7ffd6641d540 S1 { &other=0x7ffd6641d558 other=None &v1=0x7ffd6641d568 v1=123 }
new:- second: 0x7ffd6641d540 S1 { &other=0x7ffd6641d558 other=None &v1=0x7ffd6641d568 v1=123 }
create_two:  first: 0x560d89cbcbd0 S1 { &other=0x560d89cbcbe8 other=0x560d89cbcc40 &v1=0x560d89cbcbf8 v1=123 sc=1 wc=1 }
create_two:  second: 0x560d89cbcc40 S1 { &other=0x560d89cbcc58 other=0x560d89cbcbd0 &v1=0x560d89cbcc68 v1=123 sc=1 wc=1 }
create_two:-
main:  &first =0x7ffd6641d5e8  first=first: 0x560d89cbcbd0 S1 { &other=0x560d89cbcbe8 other=0x560d89cbcc40 &v1=0x560d89cbcbf8 v1=123 sc=1 wc=1 }
main:  &first=0x7ffd6641d5f0 second=second: 0x560d89cbcc40 S1 { &other=0x560d89cbcc58 other=0x560d89cbcbd0 &v1=0x560d89cbcc68 v1=123 sc=1 wc=1 }
 first.v1=124
second.v1=224
 first.v1_via_other().v1()=224 before add_via_other
 first.v1_via_other().v1()=225 after add_via_other
 second.v1_via_other().v1()=124 before add_via_other
 second.v1_via_other().v1()=125 after add_via_other
main:-
Drop S1 second: 0x560d89cbcc40 S1 { &other=0x560d89cbcc58 other=0x560d89cbcbd0 &v1=0x560d89cbcc68 v1=225 sc=1 wc=1 }
Drop S1 first: 0x560d89cbcbd0 S1 { &other=0x560d89cbcbe8 other=0x560d89cbcc40 &v1=0x560d89cbcbf8 v1=125 sc=0 wc=0 }
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
