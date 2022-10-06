#[macro_use]
extern crate bitflags;

#[cfg(not(feature = "shared-api"))]
pub mod static_lib;
#[cfg(not(feature = "shared-api"))]
pub use static_lib as bgfx;
