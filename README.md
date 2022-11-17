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
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/exper-refcell`
main:+
offset_of S1.name 0
offset_of S1.other 24
offset_of S1.v1 40
create_two:+
new:- first: 0x7ffc04103908 S1 { &other=0x7ffc04103920 other=None &v1=0x7ffc04103930 v1=123 }
new:- second: 0x7ffc04103948 S1 { &other=0x7ffc04103960 other=None &v1=0x7ffc04103970 v1=123 }
create_two:  first: 0x5572b096ebd0 S1 { &other=0x5572b096ebe8 other=0x5572b096ec40 &v1=0x5572b096ebf8 v1=123 sc=1 wc=1 }
create_two:  second: 0x5572b096ec40 S1 { &other=0x5572b096ec58 other=0x5572b096ebd0 &v1=0x5572b096ec68 v1=123 sc=1 wc=1 }
create_two:-
main:  &first =0x7ffc04103dc8  first=first: 0x5572b096ebd0 S1 { &other=0x5572b096ebe8 other=0x5572b096ec40 &v1=0x5572b096ebf8 v1=123 sc=1 wc=1 }
main:  &second=0x7ffc04103dd0 second=second: 0x5572b096ec40 S1 { &other=0x5572b096ec58 other=0x5572b096ebd0 &v1=0x5572b096ec68 v1=123 sc=1 wc=1 }
 first.v1=124
second.v1=125
 first.v1_via_other().v1()=125
       second.other().v1()=124
main:-
Drop S1 second: 0x5572b096ec40 S1 { &other=0x5572b096ec58 other=0x5572b096ebd0 &v1=0x5572b096ec68 v1=125 sc=1 wc=1 }
Drop S1 first: 0x5572b096ebd0 S1 { &other=0x5572b096ebe8 other=0x5572b096ec40 &v1=0x5572b096ebf8 v1=124 sc=0 wc=0 }
```

Here is the release run
```
$ cargo run --release
   Compiling exper-refcell v0.1.0 (/home/wink/prgs/rust/myrepos/exper-refcell)
    Finished release [optimized] target(s) in 0.18s
     Running `target/release/exper-refcell`
main:+
offset_of S1.name 0
offset_of S1.other 24
offset_of S1.v1 40
create_two:+
new:- first: 0x7ffc6a551b50 S1 { &other=0x7ffc6a551b68 other=None &v1=0x7ffc6a551b78 v1=123 }
new:- second: 0x7ffc6a551b50 S1 { &other=0x7ffc6a551b68 other=None &v1=0x7ffc6a551b78 v1=123 }
create_two:  first: 0x559c6fa53bd0 S1 { &other=0x559c6fa53be8 other=0x559c6fa53c40 &v1=0x559c6fa53bf8 v1=123 sc=1 wc=1 }
create_two:  second: 0x559c6fa53c40 S1 { &other=0x559c6fa53c58 other=0x559c6fa53bd0 &v1=0x559c6fa53c68 v1=123 sc=1 wc=1 }
create_two:-
main:  &first =0x7ffc6a551bf8  first=first: 0x559c6fa53bd0 S1 { &other=0x559c6fa53be8 other=0x559c6fa53c40 &v1=0x559c6fa53bf8 v1=123 sc=1 wc=1 }
main:  &second=0x7ffc6a551c00 second=second: 0x559c6fa53c40 S1 { &other=0x559c6fa53c58 other=0x559c6fa53bd0 &v1=0x559c6fa53c68 v1=123 sc=1 wc=1 }
 first.v1=124
second.v1=125
 first.v1_via_other().v1()=125
       second.other().v1()=124
main:-
Drop S1 second: 0x559c6fa53c40 S1 { &other=0x559c6fa53c58 other=0x559c6fa53bd0 &v1=0x559c6fa53c68 v1=125 sc=1 wc=1 }
Drop S1 first: 0x559c6fa53bd0 S1 { &other=0x559c6fa53be8 other=0x559c6fa53c40 &v1=0x559c6fa53bf8 v1=124 sc=0 wc=0 }
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
