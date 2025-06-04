<div align="center">
A rusty crate for sailing the seas of Geometric Algebra.

![banner](img/banner.png)
</div>

**Note**: Reefer is currently in **early** development and is **not** yet feature-complete.

Reefer is a Rust procedural macro library that provides compile-time geometric algebra with shape reification. It allows you to define geometric algebras with custom metrics and automatically generates optimized code for specific multivector shapes.

## Install
Reefer is currently not published on crates.io. You can add the following to your `Cargo.toml` dependencies to try it out:
```toml
[dependencies]
reefer = { git = "https://github.com/kgullion/reefer", branch = "main" }
```

## Usage
See `tests/lorentz.rs` and `tests/pga2d.rs` for current examples.

Uncomment the `println!` at `src/lib.rs:L51-58` and run
`cargo test -- --nocapture` to see the pretty-printed generated code.
Note: you may need to also change the file of interest in `tests` to invalidate any build caching. Just saving or adding a space is enough.

## Core Concepts

### Algebraic Module

The `#[reefer::algebraic]` attribute transforms a regular rust module into a geometric algebra module with compile-time optimizations.

### Field Requirements

Algebras must define a `Field` type and that type must have a `pow` operation defined (along with standard math ops like `Add`, `Mul`, `Neg` etc):
```rust
type Field = f32;

trait Pow {
    fn pow(self, n: Self) -> Self;
}
impl Pow for Field {
    fn pow(self, n: Self) -> Self {
        self.powf(n)
    }
}
```

### Magic Macros

`square!` and `shape!` are both valid only inside the GA module. The information provided is used to build out structs atd traits. It also drives the `impl` reification magic further down.

#### `square!(basis, value)`

**Parameters:**
- `basis`: Identifier for the basis vector (e.g., `e1`, `e2`)
- `value`: Numeric value for the square (`1`, `-1`, or `0`)

**Description:**
Defines the metric signature of your geometric algebra. Common values:
- `1`: Positive (Euclidean) dimension
- `-1`: Negative (Minkowski/Lorentzian) dimension
- `0`: Null/degenerate dimension (used in projective algebras like PGA)

The basis is a lowercase prefix and an index (either digit or uppercase). This means there are 936 (!!) available basis vector names currently (26*36), though I do have ideas to support even more.

#### `shape!(name, [ Mv<blades...>, ... ])`

**Parameters:**
- `Name`: Identifier for the shape family
- `Mv<blades...>`: Multivector type with specified blades

**Attributes:**
Attributes are passed through and put directly onto the structs.

**Description:**
Shapes define families of multivectors that share common blade structures. Multiple `Mv<>` definitions create shape variants within the same family (more on that later).

#### Example

```rust
#[reefer::algebraic]
mod my_pga2d {
    type Field = f32;

    // Define basis vectors and their squares
    square!(e0, 0);
    square!(e1, 1);
    square!(e2, 1);

    // Define geometric shapes
    #[derive(Clone, Debug)]
    shape!(Scalar, Mv<scalar>);
    #[derive(Clone, Debug)]
    shape!(Psuedoscalar, Mv<e012>);
    #[derive(Clone, Debug)]
    shape!(Line, Mv<e1, e2, e0>);
    #[derive(Clone, Debug)]
    shape!(Ideal, Mv<e01, e20>);
    #[derive(Clone, Debug)]
    shape!(Point, Mv<e20, e01, e12>);
}
```
expands to
```rust
mod my_pga2d {
    trait Mv {}
    type Field = f32;
    trait Scalar {}
    impl Scalar for Mv_scalar {}
    impl Mv for Mv_scalar {}
    #[derive(Clone, Debug)]
    pub struct Mv_scalar {
        pub scalar: Field,
    }
    trait Psuedoscalar {}
    impl Psuedoscalar for Mv_e012 {}
    impl Mv for Mv_e012 {}
    #[derive(Clone, Debug)]
    pub struct Mv_e012 {
        pub e012: Field,
    }
    trait Line {}
    impl Line for Mv_e1_e2_e0 {}
    impl Mv for Mv_e1_e2_e0 {}
    #[derive(Clone, Debug)]
    pub struct Mv_e1_e2_e0 {
        pub e1: Field,
        pub e2: Field,
        pub e0: Field,
    }
    trait Ideal {}
    impl Ideal for Mv_e01_e20 {}
    impl Mv for Mv_e01_e20 {}
    #[derive(Clone, Debug)]
    pub struct Mv_e01_e20 {
        pub e01: Field,
        pub e20: Field,
    }
    trait Point {}
    impl Point for Mv_e20_e01_e12 {}
    impl Mv for Mv_e20_e01_e12 {}
    #[derive(Clone, Debug)]
    pub struct Mv_e20_e01_e12 {
        pub e20: Field,
        pub e01: Field,
        pub e12: Field,
    }
}
```

## Shape Reification

Now that we've generated all the structs, let's reify some geometric concepts!

### `#[reify(Shape as Type)]`

**Parameters:**
- `Shape`: The shape family name
- `T`: Type parameter identifier used in the implementation

**Description:**
The reifier generates concrete implementations for each shape variant in the Cartesian product of all reified parameters. This enables compile-time specialization while maintaining generic code.

**Example:**
```rust
#[reify(Point as P)]
#[reify(Line as L)]
impl BitXor<L> for P {
    type Output = impl Point;
    /// Meet operation: intersect line with point
    fn bitxor(self, line: L) -> Self::Output {
        self ^ line
    }
}
```
expands into
```rust
impl BitXor<Mv_e1_e2_e0> for Mv_e1_e2_e0 {
    type Output = Mv_e20_e01_e12;
    /// meet two lines into a point
    fn bitxor(self, line: Mv_e1_e2_e0) -> Self::Output {
        Mv_e20_e01_e12 {
            e20: -1 as Field * (self.e0 * line.e2 + self.e2 * line.e0 * -1 as Field),
            e01: self.e0 * line.e1 + self.e1 * line.e0 * -1 as Field,
            e12: self.e1 * line.e2 + self.e2 * line.e1 * -1 as Field,
        }
    }
}
```

A few things are happening here.

First, the `reify` macro will sub in and emit a new `impl` for each combination of shape struct (currently each shape only has one struct, see `tests` for more complex examples).

Next reefer generates a symbolic multivector for each shape argument.

Then reefer symbolically expands any built in operations.

Finally reefer will replace `type Output = impl Shape;` with the calculated shape, so long as it is a known shape.

## Built-in Operations

### Geometric Product Operations

| Operator | Method | Description |
|----------|---------|-------------|
| `x + y` | `x.add(y)` | Addition |
| `x - y` | `x.sub(y)` | Subtraction |
| `x * y` | `x.mul(y)` | Geometric product |
|         | `x.pow(y)`     | raise x to a positive integer power |
| `x ^ y` | `x.wedge(y)` | Exterior (wedge) product |
| `x | y` | `x.fatdot(y)`| "fat" dot product
|         | `x.dot(y)`   | scalar dot product (basically just `x.fatdot(y).grade(0)`) |
| `x << y`| `x.lcontract(y)`| Left Contraction |
| `x >> y`| `x.rcontract(y)`| Right Contraction |
| `x & y` | `x.regressive(y)` | Regressive product |
| `!x`    | `x.dual(ps)`   | Dual (ps defaults to the wedge of all basis vectors) |
|         | `x.undual(ps)` | Undual (see dual) |
| `-x`    | `x.neg()`      | Negation          |
|         | `x.aut()`      | Automorphism (negates when `grade%2`) |
|         | `x.rev()`      | Reverse blade axis order |
|         | `x.conj()`     | Conjugation |
|         | `x.grade(n)`   | Grade             |
|         | `x.commutate(y)`| Commutator Product (x*y-y*x)/2 |
|         | `x.anticomm(y)`| Anticommutator Product (x*y+y*x)/2 |
| `x % y` | `x.sandwich(y)`| Sandwich product (x * y * x.reverse()) |
|         | `x.norm()`     | Gets the norm of x. Note this is still a mv so do `x.norm().scalar` if you want the scalar part |
|         | `x.normed()`   | normalizes the x component |
|         | `x.simplify()` | simplifies the multivector, dropping any comptime known symbolic zeros |
| The following is planned but not yet implemented
|         | `x.exp()`      | taylor series expansion of `(euler's const)**x`. if `x` goes to zero within 4 squarings, the value is exact, otherwise runtime range reduction will need to be performed |
|         | `x.log()`      | taylor series expansion of `log(x)` |
|         | `x.sqrt()`     | square root of x |
| `1/x`   | `x.inverse()`  | Inversion of x |
| `x/y`   | `x.div(y)`     | Division       |

## Performance Notes

- **Compile-time optimization**: Reefer generates specialized code for each shape variant, eliminating runtime overhead
- **Zero-cost abstractions**: Operations compile down to direct field arithmetic with no hidden allocations (just field copies)
- **Shape inference**: The type system ensures only valid geometric operations are performed
- **Algebraic simplification**: The `simplify()` method performs compile-time symbolic reduction

## Maintainers

[@kgullion](https://github.com/kgullion)

## Contributing

Feel free to open an issue if you have any thoughts or improvements, even if it's just a syntax suggestion, a method you think should be included, or just information on the subject you think may be relevant.

If you'd like to contribute then adding more documentation, tests, and examples is a great place to start. Beyond that, `geometry/mvect.rs` is where the mathematics happens and `build/reifier.rs` does most of the heavy lifting for codegen.

The https://bivector.net/ Discord server is a great place to go if you're interested in Geometric Algebra.

## License

MIT © 2024 Kyle Gullion
