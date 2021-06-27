# bgfx-rs

Rust bindings to [bgfx](https://github.com/bkaradzic/bgfx), a cross-platform, graphics API agnostic, "Bring Your Own Engine/Framework" style rendering library.

## Status

Currently being developed. While this [wrapper](https://github.com/rhoot/bgfx-rs) for Rust exists, the code here takes a different approach and generate high-level bindings from the BGFX API [def](https://github.com/bkaradzic/bgfx/blob/master/scripts/bgfx.idl) instead which will allow easier updating of the bindings once the API changes with much reduced work.

