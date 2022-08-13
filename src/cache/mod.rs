#[cfg(feature = "fs")]
mod fs;
#[cfg(not(feature = "fs"))]
mod memory;
mod utils;

#[cfg(feature = "fs")]
pub use self::fs::*;
#[cfg(not(feature = "fs"))]
pub use self::memory::*;
