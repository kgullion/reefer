**Note**: Reefer is currently in **early** development and is **not** yet feature-complete.

![banner](img/banner.png)
<div align="center">
A rusty shipmate for sailing the seas of Geometric Algebra.
</div>

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Background
Reefer began as a project to learn both Rust and Geometric Algebra (GA). I've spent over a decade in the software industry but never had the chance to dive into Rust until now. I’ve also been interested in GA for some time now but hadn’t found the time to explore it in depth.

Short of macros for C or C++ (or maybe comptime in Zig), I don’t know another way to build something quite like this. Whether it’s the best way, I can’t say, but I went ahead with it regardless. There’s no doubt some aspects could be refined, both in the math and in the code.

At this stage, Reefer has most of what you’d expect from a GA library, though it’s still missing Inverse, Normalize, Exponentials, and Logarithms. I’m still working through how to implement these, but they’re on the roadmap.

## Install
Reefer is currently not published on crates.io. You can install from Github or clone the repo and build it locally if you want to give it a try. I do plan to publish it eventually, but I want to get a few more features in first.

Add the following to your `Cargo.toml` dependencies to try it out:
```toml
reefer = { git = "https://github.com/kgullion/reefer", branch = "main" }
```

## Usage
See `pga2d.rs`, `vga3d.rs`, and `vga6d.rs` for current examples of building a library. I'm still working on the semantics of building a specific algebra, but for now those 3 are used for testing and development. Currently, they are hardcoded to use f32 as the field, but that will be configurable in the future. If you have a specific algebra you'd like to see, let me know and I'll try to get it in there.

### Implemented Operations

| Operation             | Syntax                 |
|-----------------------|------------------------|
| Equality              | `a == b`               | 
| Addition              | `a + b`                | 
| Subtraction           | `a - b`                | 
| Geometric Product     | `a * b`                | 
| Outer Product         | `a ^ b`                | 
| Inner Product         | `a \| b`               |
| Regressive Product    | `a & b`                | 
| Left Contraction      | `a << b`               | 
| Right Contraction     | `a >> b`               | 
| Commutator Product    | `a.commutator(b)`      | 
| Scalar Product        | `a.scalar_prod(b)`     | 
| "Fat" Dot Product     | `a.fat_dot(b)`         | 
| Dual                  | `a.dual()`             | 
| Undual                | `a.undual()`           | 
| Reverse               | `a.reverse()`          | 
| Involute              | `a.involute()`         | 
| Conjugate             | `a.conjugate()`        | 
| Grade Selection       | `a.graded<typenum::U2>()`|
| Basis Selection       | `a[e02]`               | 
| Basis Mutation        | `a[e02] = 3.0`         | 


## Quickstart
```rust
// hopefully the traits won't need explicitly imported in the future but for now they do
use reefer::pga2d::*;
use reefer::traits::*;

fn main() {
    // I only provide the "canonical" basis elements, but most ops (those excluding floats and adds) are available for the basis elements
    let e20 = e02.reverse();
    let a = 3.0 * e20 + 5.0 * e01 + 1.0 * e12;
    let b = 7.0 * e20 + 11.0 * e01 + 1.0 * e12;
    let c = a & b;
    println!("{}", c);
    // Output: 2 * e0 + 6 * e1 - 4 * e2

    // Each of these are 12 bytes, 3 floats of 4 bytes each
    // No extra space is used and no heap allocations are done!
    println!("size(a): {}", core::mem::size_of_val(&a)); // = 12
    println!("size(b): {}", core::mem::size_of_val(&b)); // = 12
    println!("size(c): {}", core::mem::size_of_val(&c)); // = 12

    // The basis elements are zero-sized types, so they don't take up any space until floats are multiplied in
    println!("size(e02): {}", core::mem::size_of::<e02>()); // = 0
    println!("size(e20): {}", core::mem::size_of::<e20>()); // = 0
    assert!(e02 == -e20);
}
```

Check out the tests at the bottom of `src/mvect/mul.rs` (and other files) for some more examples. I plan to add more examples and better documentation in the future but that should be enough to poke around.

## Maintainers

[@kgullion](https://github.com/kgullion)

## Contributing

Feel free to dive in! Open an issue, submit a PR, or just ask a question. I'm happy to help out where I can. I frequently can be found in the https://bivector.net/ Discord server, which is a great place to go if you're also interested in Geometric Algebra.

## License

MIT © 2024 Kyle Gullion
