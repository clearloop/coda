macro_rules! impl_uint {
    (#[$doc:meta], $name:ident, $bits:expr, $type:ident) => {
        #[$doc]
        #[allow(non_camel_case_types)]
        #[derive(Default, Clone, Copy, Debug)]
        pub struct $name($type);

        impl $name {
            pub const MAX: Self = $name(((1 as $type) << $bits) - 1);
            pub const MIN: Self = $name(0);

            fn mask(self) -> Self {
                $name(self.0 & (((1 as $type) << $bits).overflowing_sub(1).0))
            }
        }
    };
}

macro_rules! impl_int {
    (#[$doc:meta], $name:ident, $bits:expr, $type:ident) => {
        #[$doc]
        #[allow(non_camel_case_types)]
        #[derive(Default, Clone, Copy, Debug)]
        pub struct $name($type);

        #[$doc]
        impl $name {
            pub const MAX: Self = $name(((1 as $type) << ($bits - 1)) - 1);
            pub const MIN: Self = $name(-((1 as $type) << ($bits - 1)));

            fn mask(self) -> Self {
                if (self.0 & (1 << ($bits - 1))) == 0 {
                    $name(self.0 & (((1 as $type) << $bits).overflowing_sub(1).0))
                } else {
                    $name(self.0 | !(((1 as $type) << $bits).overflowing_sub(1).0))
                }
            }
        }
    };
}

impl_int!(#[doc="The 2-bit signed integer type."], i2, 2, i8);
impl_uint!(#[doc="The 2-bit unsigned integer type."], u2, 2, u8);
