#![macro_use]
macro_rules! impl_int {
    {$([$name:ident, $bits:expr, $type:ty],)*} => {$(impl_int!($name, $bits, $type);)*};
    ($name:ident, $bits:expr, $type:ty) => {
        doc_comment! {
            concat!("The ", stringify!($bits), "-bit signed integer type."),
            #[allow(non_camel_case_types)]
            #[derive(Default, Clone, Copy, Debug)]
            pub struct $name($type);
        }

        impl $name {
            pub const MAX: Self = $name(((1 as $type) << ($bits - 1)) - 1);
            pub const MIN: Self = $name(-((1 as $type) << ($bits - 1)));

            fn mask(self) -> Self {
                if (self.0 & (1 << ($bits - 1))) == 0 {
                    $name(self.0 & Self::MAX.0.overflowing_sub(1).0)
                } else {
                    $name(self.0 | !Self::MAX.0.overflowing_sub(1).0)
                }
            }
        }

        impl_convert!($name, $type);
    };
}

/// Implementation
use core::ops::{Deref, DerefMut};

impl_int! {
    [i2, 2, i8],
}
