# Experiment with RefCell

By using `RefCell` with `MaybeUninit` I'm trying to make it a
little more idiomatic rust. In that you use s1.this.borrow().v1()
instead of a S1::v1(&s1). But it doesn't always work :)

Just playing around!

# Run

Fails debug builds:
```
$ cargo run
   Compiling exper-refcell v0.1.0 (/home/wink/prgs/rust/myrepos/exper-refcell)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/exper-refcell`
thread 'main' panicked at 'explicit panic', src/main.rs:53:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Succeeds release builds:
```
$ cargo run --release
   Compiling exper-refcell v0.1.0 (/home/wink/prgs/rust/myrepos/exper-refcell)
    Finished release [optimized] target(s) in 0.13s
     Running `target/release/exper-refcell`
```

I'm guessing this has something to do with optimizations, but the
assert_eq! in `fn new` "always" works so something definitely weird.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
