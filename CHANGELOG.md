# Changelog

This project follows semantic versioning.

### v0.19 (2023-10-23)

- [Added] Depend on latest bgfx-sys with updated BGFX code. 
- [fixed] Window scaling in web example (Thanks tritao!) 

### v0.18 (2023-03-18)

- [Added] Emscripten support. See `web_example` for an example how this works.

### v0.9 (2022-04-08)

- [fixed] Fixed a crash when using dynamic vertex buffers

### v0.3 (2021-09-11)

- [changed] Made ViewId public
- [changed] create_uniform(..) now takes &str for name
- [changed] Depend on `cfixed-string` to construct strings for the FFI layer without allocations.

### v0.2 (2021-08-19)

- [changed] Updated bgfx/bimg/bx to latest git version
- [changed] Build Vulkan for Linux and Windows (OpenGL still default on Linux)
