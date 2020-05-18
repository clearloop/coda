#![macro_use]
macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

macro_rules! impl_all {
    ($name:ident, $bits:expr, $type:ty) => {
        impl_convert!($name, $type);
        impl_methods!($name, $type);
        impl_cmp!($name, $type);
        impl_fmt!($name, $type);
        impl_ops!($name, $type);
    };
}

macro_rules! impl_int {
    {$([$name:ident, $bits:expr, $type:ty],)*} => {$(impl_int!($name, $bits, $type);)*};
    ($name:ident, $bits:expr, $type:ty) => {
        doc_comment! {
            concat!("The ", stringify!($bits), "-bit signed integer type."),
            #[allow(non_camel_case_types)]
            #[derive(Default, Clone, Copy, Debug)]
            pub struct $name(pub $type);
        }

        impl $name {
            pub const MAX: Self = $name(((1 as $type) << ($bits - 1)) - 1);
            pub const MIN: Self = $name(-((1 as $type) << ($bits - 1)));

            fn mask(self) -> Self {
                if (self.0 & (1<<($bits-1))) == 0 {
                    $name(self.0 & ((1 as $type) << $bits).overflowing_sub(1).0)
                } else {
                    $name(self.0 | !((1 as $type) << $bits).overflowing_sub(1).0)
                }
            }
        }

        impl_all!($name, $bits, $type);
    };
}

macro_rules! impl_uint {
    {$([$name:ident, $bits:expr, $type:ident],)*} => {$(impl_uint!($name, $bits, $type);)*};
    ($name:ident, $bits:expr, $type:ident) => {
        doc_comment! {
            concat!("The ", stringify!($bits), "-bit unsigned integer type."),
            #[allow(non_camel_case_types)]
            #[derive(Default, Clone, Copy, Debug)]
            pub struct $name(pub $type);
        }

        impl $name {
            pub const MAX: Self = $name(((1 as $type) << $bits) - 1);
            pub const MIN: Self = $name(0);

            fn mask(self) -> Self {
                $name(self.0 & (((1 as $type) << $bits).overflowing_sub(1).0))
            }
        }

        impl_all!($name, $bits, $type);
    };
}
