# Experiment with RefCell

By using `RefCell` with `MaybeUninit` I'm trying to make it a
little more idiomatic rust. In that you use s1.this.borrow().v1()
instead of a S1::v1(&s1). But it doesn't always work :)

# Run

Works for both dev and release builds:
```
$ cargo run
   Compiling exper-refcell v0.1.0 (/home/wink/prgs/rust/myrepos/exper-refcell)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `/home/wink/prgs/rust/myrepos/exper-refcell/target/debug/exper-refcell`
s1: 0x7ffe4b4b38f8
self: 0x5589ad5bbba0 v1: 123
self: 0x5589ad5bbba0 v1: 123
Success
Drop S1 0x5589ad5bbba0

$ cargo run --release
   Compiling exper-refcell v0.1.0 (/home/wink/prgs/rust/myrepos/exper-refcell)
    Finished release [optimized] target(s) in 0.14s
     Running `/home/wink/prgs/rust/myrepos/exper-refcell/target/release/exper-refcell`
s1: 0x7ffec14d9950
self: 0x55df45479ba0 v1: 123
self: 0x55df45479ba0 v1: 123
Success
Drop S1 0x55df45479ba0
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
