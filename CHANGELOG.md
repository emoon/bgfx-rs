# Changelog

This project follows semantic versioning.

### v0.9 (2022-04-08)

- [fixed] Fixed a crash when using dynamic vertex buffers

### v0.3 (2021-09-11)

- [changed] Made ViewId public
- [changed] create_uniform(..) now takes &str for name
- [changed] Depend on `cfixed-string` to construct strings for the FFI layer without allocations.

### v0.2 (2021-08-19)

- [changed] Updated bgfx/bimg/bx to latest git version
- [changed] Build Vulkan for Linux and Windows (OpenGL still default on Linux)
