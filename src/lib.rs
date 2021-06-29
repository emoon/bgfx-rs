#[macro_use]
extern crate bitflags;

#[cfg(feature = "shared-api")]
pub mod shared_api;
#[cfg(feature = "shared-api")]
pub use shared_lib as bgfx;
#[cfg(not(feature = "shared-api"))]
pub mod static_lib;
#[cfg(not(feature = "shared-api"))]
pub use static_lib as bgfx;

