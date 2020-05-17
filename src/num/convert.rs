#![macro_use]
macro_rules! impl_from {
    ($name:ident, $from:ty) => {
        impl From<$from> for $name {
            fn from(source: $from) -> $name {
                $name(source).mask()
            }
        }
    };
}

macro_rules! impl_into {
    ($name:ident, $into:ty) => {
        impl Into<$into> for $name {
            fn into(self) -> $into {
                self.0
            }
        }
    };
}

macro_rules! impl_deref {
    ($name:ident, $target:ty) => {
        impl Deref for $name {
            type Target = $target;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

macro_rules! impl_convert {
    ($name:ident, $target:ty) => {
        impl_deref!($name, $target);
        impl_from!($name, $target);
        impl_into!($name, $target);
    };
}
