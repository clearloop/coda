#![macro_use]
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

macro_rules! impl_arithmetic {
    ($name:ident, $type:ty) => {
        impl Add<$name> for $name {
            type Output = $name;
            fn add(self, other: $name) -> $name {
                let res = self.0 + other.0;
                assert_range!($name, res);
                $name(res)
            }
        }

        impl Sub<$name> for $name {
            type Output = $name;
            fn sub(self, other: $name) -> $name {
                let res = self.0 - other.0;
                assert_range!($name, res);
                $name(res)
            }
        }

        impl Mul for $name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                let res = self.0 * rhs.0;
                assert_range!($name, res);
                $name(res)
            }
        }

        impl Div for $name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self {
                let res = self.0 / rhs.0;
                assert_range!($name, res);
                $name(res)
            }
        }
    };
}

macro_rules! impl_bitwise {
    ($name:ident, $type:ty) => {
        impl<T> Shr<T> for $name
        where
            $type: Shr<T, Output = $type>,
        {
            type Output = $name;
            fn shr(self, rhs: T) -> $name {
                $name(self.mask().0.shr(rhs))
            }
        }

        impl<T> Shl<T> for $name
        where
            $type: Shl<T, Output = $type>,
        {
            type Output = $name;

            fn shl(self, rhs: T) -> $name {
                $name(self.mask().0.shl(rhs))
            }
        }

        impl<T> ShrAssign<T> for $name
        where
            $type: ShrAssign<T>,
        {
            fn shr_assign(&mut self, rhs: T) {
                *self = self.mask();
                self.0.shr_assign(rhs);
            }
        }

        impl<T> ShlAssign<T> for $name
        where
            $type: ShlAssign<T>,
        {
            fn shl_assign(&mut self, rhs: T) {
                *self = self.mask();
                self.0.shl_assign(rhs);
            }
        }

        impl BitOr<$name> for $name {
            type Output = $name;

            fn bitor(self, rhs: $name) -> Self::Output {
                $name(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a> BitOr<&'a $name> for $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitor(self, rhs: &'a $name) -> Self::Output {
                $name(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a> BitOr<$name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitor(self, rhs: $name) -> Self::Output {
                $name(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a> BitOr<&'a $name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitor(self, rhs: &'a $name) -> Self::Output {
                $name(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl BitOrAssign<$name> for $name {
            fn bitor_assign(&mut self, other: $name) {
                *self = self.mask();
                self.0.bitor_assign(other.mask().0)
            }
        }

        impl BitXor<$name> for $name {
            type Output = $name;

            fn bitxor(self, rhs: $name) -> Self::Output {
                $name(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl<'a> BitXor<&'a $name> for $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitxor(self, rhs: &'a $name) -> Self::Output {
                $name(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl<'a> BitXor<$name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitxor(self, rhs: $name) -> Self::Output {
                $name(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl<'a> BitXor<&'a $name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitxor(self, rhs: &'a $name) -> Self::Output {
                $name(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl BitXorAssign<$name> for $name {
            fn bitxor_assign(&mut self, other: $name) {
                *self = self.mask();
                self.0.bitxor_assign(other.mask().0)
            }
        }

        impl Not for $name {
            type Output = $name;

            fn not(self) -> $name {
                $name(self.mask().0.not())
            }
        }

        impl<'a> Not for &'a $name {
            type Output = <$name as Not>::Output;

            fn not(self) -> $name {
                $name(self.mask().0.not())
            }
        }

        impl BitAnd<$name> for $name {
            type Output = $name;

            fn bitand(self, rhs: $name) -> Self::Output {
                $name(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl<'a> BitAnd<&'a $name> for $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitand(self, rhs: &'a $name) -> Self::Output {
                $name(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl<'a> BitAnd<$name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitand(self, rhs: $name) -> Self::Output {
                $name(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl<'a> BitAnd<&'a $name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            fn bitand(self, rhs: &'a $name) -> Self::Output {
                $name(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl BitAndAssign<$name> for $name {
            fn bitand_assign(&mut self, other: $name) {
                *self = self.mask();
                self.0.bitand_assign(other.mask().0)
            }
        }
    };
}

macro_rules! impl_ops {
    ($name:ident, $type:ty) => {
        impl_arithmetic!($name, $type);
        impl_bitwise!($name, $type);
    };
}
