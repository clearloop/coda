#![macro_use]
#[allow(unused_macros)]
macro_rules! assert_range {
    ($name:ty, $val:expr) => {
        debug_assert!(
            $val <= <$name>::MAX.0 && $val >= <$name>::MIN.0,
            "{} should between {} and {}",
            $val,
            <$name>::MAX.0,
            <$name>::MIN.0,
        );
    };
}

macro_rules! impl_calc {
    ($name:ident, $bits:expr, $type:ty) => {
        impl $name {
            pub fn min_value() -> $name {
                $name::MIN
            }

            pub fn max_value() -> $name {
                $name::MIN
            }

            pub fn wrapping_sub(self, rhs: Self) -> Self {
                $name(self.0.wrapping_sub(rhs.0)).mask()
            }

            pub fn wrapping_add(self, rhs: Self) -> Self {
                $name(self.0.wrapping_add(rhs.0)).mask()
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.mask().0 == other.mask().0
            }
        }
    };
}
