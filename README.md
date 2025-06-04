**Note**: Reefer is currently in **early** development and is **not** yet feature-complete.

![banner](img/banner.png)
<div align="center">
A rusty crate for sailing the seas of Geometric Algebra.
</div>

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
Note: you may need to also change the file in test to invalidate any build caching. Just saving or adding a space is enough.

## Maintainers

[@kgullion](https://github.com/kgullion)

## Contributing

Feel free to dive in! Open an issue, submit a PR, or just ask a question. I'm happy to help out where I can. I can sometimes be found in the https://bivector.net/ Discord server, which is a great place to go if you're interested in Geometric Algebra.

## License

MIT © 2024 Kyle Gullion
