//! Numeric traits and functions for the built-in numeric types.
mod calc;
mod convert;
mod init;

// deps
use core::ops::{Deref, DerefMut};

impl_int! {
    [i2, 2, i8],
}

impl_uint! {
    [u2, 2, i8],
}
