# Experiment with RefCell

By using `RefCell` a function can decide if it needs to mutate
the `this` parameter. This is not idiomatic Rust, but it does work.

# Run

```
$ cargo run
   Compiling exper-refcell v0.1.0 (/home/wink/prgs/rust/myrepos/exper-refcell)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/exper-refcell`
s1.v1=1
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
