# bgfx-rs

Rust bindings to [bgfx](https://github.com/bkaradzic/bgfx), a cross-platform, graphics API agnostic, "Bring Your Own Engine/Framework" style rendering library.

## Why another wrapper?

While this [wrapper](https://github.com/rhoot/bgfx-rs) for Rust exists, the code here takes a different approach and generate high-level bindings from the BGFX API [def](https://github.com/bkaradzic/bgfx/blob/master/scripts/bgfx.idl) instead which will allow easier updating of the bindings once the API changes with much reduced work.
In some cases there will be manual implementation where it makes sense to provide more ergonomic Rust code where auto-generation is difficult.

## Status

Currently being developed and the API is changing. The goal of this library is to cover the majority of the BGFX functionality while providing some Rust style convenience on top of the C API.

Usage
-----

```toml
# Cargo.toml
[dependencies]
bgfx-rs = "0.19"
```

The library doesn't include any window handling and that has to be provided by the user. See [examples](https://github.com/emoon/bgfx-rs/tree/main/examples) an how to use [GLFW](https://crates.io/crates/glfw)

## License

Licensed under [BSD 2-Clause](https://bkaradzic.github.io/bgfx/license.html) to keep the license the same the BGFX code.

