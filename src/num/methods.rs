#![macro_use]
macro_rules! impl_methods {
    ($name:ident, $type:ty) => {
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
    };
}
