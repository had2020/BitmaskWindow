# BitmaskWindow

A Low-overhead, data-oriented bitmask generator that isolates arbitrary data windows using compile-time bitwise AND masks.

A ultra-minimal, `no_std` Rust macro designed to save development time and eliminate the mental friction of manual bit-masking, all while guaranteeing low runtime overhead, on all integers. By evaluating mask boundaries strictly at compile time, `bitslice!` forces `rustc` to collapse the entire operation into a minimal set of hardware instructions. This prevents data dependencies that stall Out-of-Order (OoO) execution pipelines, keeps the L1 Instruction Cache footprint virtually at zero, and completely bypasses the data cache by operating entirely within CPU registers.

```rust
use bitmaskwindow::bitslice;

fn main() {
    // isolate bits 4 through 7 (inclusive) from a u8
    // arguments: (left_most_end, right_most_start, type_zero, input_value)
    let raw_data: u8 = 0b1011_0100;
    let sliced = bitslice!(7, 4, 0u8, raw_data); // collapses to a single shift/mask or `bextr` instruction
    
    assert_eq!(sliced, 11); // 0b1011 (11 decimal)
}
```

## Installation

``` bash
cargo add bitmaskwindow
```

Add this directly to your `Cargo.toml` dependencies, pointing to the GitHub repository:

```toml
[dependencies]
bitmaskwindow = { git = "[https://github.com/had2020/bitmaskwindow](https://github.com/had2020/bitmaskwindow)", branch = "main" }
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
