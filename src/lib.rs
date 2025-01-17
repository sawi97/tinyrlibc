//! A tiny C library, written in Rust.
//!
//! See README.md for more details.
//!
//! This file is Copyright (c) Jonathan 'theJPster' Pallant 2019
//! Licensed under the Blue Oak Model Licence 1.0.0
//!
//! See each module for its respective license.

#![cfg_attr(not(test), no_std)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused_imports)]

#[cfg(feature = "alloc")]
mod malloc;
#[cfg(feature = "alloc")]
pub use self::malloc::{calloc, free, malloc, realloc};

// A new global allocator is required for the tests, but not for the library itself.
// This is because the default alloc crate uses the system allocator, collides with
// the one in this crate, and causes a link error.
#[cfg(all(feature = "alloc", test))]
use static_alloc::Bump;
#[cfg(all(feature = "alloc", test))]
#[global_allocator]
static ALLOCATOR: Bump<[u8; 1024 * 1024]> = Bump::uninit();

mod itoa;
#[cfg(feature = "itoa")]
pub use self::itoa::itoa;
#[cfg(feature = "utoa")]
pub use self::itoa::utoa;

mod abs;
#[cfg(feature = "abs")]
pub use self::abs::abs;

mod strcmp;
#[cfg(feature = "strcmp")]
pub use self::strcmp::strcmp;

mod strncmp;
#[cfg(feature = "strncmp")]
pub use self::strncmp::strncmp;

mod strcpy;
#[cfg(feature = "strcpy")]
pub use self::strcpy::strcpy;

mod strncpy;
#[cfg(feature = "strncpy")]
pub use self::strncpy::strncpy;

mod strlen;
#[cfg(feature = "strlen")]
pub use self::strlen::strlen;

mod strtol;
#[cfg(feature = "atoi")]
pub use self::strtol::atoi;
#[cfg(feature = "isalpha")]
pub use self::strtol::isalpha;
#[cfg(feature = "isdigit")]
pub use self::strtol::isdigit;
#[cfg(feature = "isspace")]
pub use self::strtol::isspace;
#[cfg(feature = "isupper")]
pub use self::strtol::isupper;
#[cfg(feature = "strtoimax")]
pub use self::strtol::strtoimax;
#[cfg(feature = "strtol")]
pub use self::strtol::strtol;
#[cfg(feature = "strtoll")]
pub use self::strtol::strtoll;
#[cfg(feature = "strtoul")]
pub use self::strtol::strtoul;
#[cfg(feature = "strtoull")]
pub use self::strtol::strtoull;
#[cfg(feature = "strtoumax")]
pub use self::strtol::strtoumax;

mod strstr;
#[cfg(feature = "strstr")]
pub use self::strstr::strstr;

mod strchr;
#[cfg(feature = "strchr")]
pub use self::strchr::strchr;

mod snprintf;

mod ctype;
pub use self::ctype::*;
