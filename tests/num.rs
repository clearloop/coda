use coda::num::{i2, u2};

#[test]
fn min_max_values() {
    macro_rules! mmv {
        {$([$name: ident, $min:expr, $max:expr],)*} => {$(mmv!($name, $min, $max);)*};
        ($name:ident, $min:expr, $max:expr) => {
            assert_eq!($name::MIN, $name($min));
            assert_eq!($name::MAX, $name($max));
        };
    }

    mmv! {
        [i2, -2_i8, 1_i8],
        [u2, 0_i8, 3_i8],
    };
}

#[test]
fn test_convert() {
    macro_rules! mmc {
        {$([$name: ident, $min:expr, $max:expr],)*} => {$(mmc!($name, $min, $max);)*};
        ($name:ident, $min:expr, $max:expr) => {
            assert_eq!($name::MIN, $name::from($min));
            assert_eq!($name::MAX, $name::from($max));
        };
    }

    mmc! {
        [i2, i8::MIN + 2, i8::MAX - 2],
        [u2, i8::MIN, i8::MAX],
    }
}
