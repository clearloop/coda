//! Numeric traits and functions for the built-in numeric types.
mod cmp;
mod convert;
mod fmt;
mod init;
mod methods;
mod ops;

// deps
use core::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt::{Binary, Display, Error, Formatter, LowerHex, Octal, UpperHex},
    hash::{Hash, Hasher},
    ops::{
        Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Div,
        Mul, Not, Shl, ShlAssign, Shr, ShrAssign, Sub,
    },
};

impl_int! {
    [i2, 2, i8],
    [i4, 4, i8],
}

impl_uint! {
    [u2, 2, i8],
    [u4, 4, i8],
}
