#![macro_use]
macro_rules! impl_uint {
    {$([$name:ident, $bits:expr, $type:ident],)*} => {$(impl_uint!($name, $bits, $type);)*};
    ($name:ident, $bits:expr, $type:ident) => {
        doc_comment! {
            concat!("The ", stringify!($bits), "-bit unsigned integer type."),
            #[allow(non_camel_case_types)]
            #[derive(Default, Clone, Copy, Debug)]
            pub struct $name($type);
        }

        impl $name {
            pub const MAX: Self = $name(((1 as $type) << $bits) - 1);
            pub const MIN: Self = $name(0);

            fn mask(self) -> Self {
                $name(self.0 & Self::MAX.0.overflowing_sub(1).0)
            }
        }
    };
}

impl_uint! {
    [u2, 2, i8],
}
