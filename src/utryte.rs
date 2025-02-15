use crate::Trit;
use std::convert::From;
use std::convert::Into;
use std::fmt;
use std::ops::{self, Add, BitAnd, BitOr, BitXor, Div, Mul, Not};
macro_rules! impl_from_tryte {
    ($name:ident, $($t:ty),*) => {
        $(
            impl From<$name> for $t {
                fn from(val: $name) -> Self {
                    usize::from(val) as $t
                }
            }
        )*
    };
}

macro_rules! define_ternary_type {
    ($name:ident, $size:expr) => {
        #[derive(Clone, Copy)]
        pub struct $name {
            pub trits: [Trit; $size],
        }

        impl $name {
            pub fn new(trits: [Trit; $size]) -> Self {
                Self { trits }
            }
        }

        impl fmt::Binary for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                for i in 0..$size {
                    write!(f, "{}", self.trits[i])?;
                }
                Ok(())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let aggregate: usize = (*self).into();
                write!(f, "{}", aggregate)?;
                Ok(())
            }
        }
        impl From<$name> for usize {
            fn from(val: $name) -> Self {
                let mut aggregate: usize = 0;
                let mut power: usize = 1;

                for i in (0..$size).rev() {
                    match val.trits[i] {
                        Trit::Zero => (),
                        Trit::One => aggregate += power,
                        Trit::Two => aggregate += power * 2,
                    }
                    power *= 3;
                }
                aggregate
            }
        }

        impl<T> From<T> for $name
        where
            T: Copy
                + Default
                + std::ops::Rem<Output = T>
                + std::ops::DivAssign
                + PartialOrd
                + From<u8>,
        {
            fn from(mut item: T) -> Self {
                let three = T::from(3u8);
                let mut final_tryte = $name {
                    trits: [Trit::Zero; $size],
                };

                for i in (0..$size).rev() {
                    let digit = item % three;
                    final_tryte.trits[i] = match digit {
                        d if d == T::from(0u8) => Trit::Zero,
                        d if d == T::from(1u8) => Trit::One,
                        d if d == T::from(2u8) => Trit::Two,
                        _ => unreachable!(),
                    };
                    item /= three;
                }
                final_tryte
            }
        }

        impl ops::Shl<usize> for $name {
            type Output = Self;
            fn shl(self, rhs: usize) -> Self::Output {
                let mut trit_copy = self.trits;
                for _ in 0..rhs {
                    for i in 0..($size - 1) {
                        trit_copy[i] = trit_copy[i + 1];
                    }
                    trit_copy[$size - 1] = Trit::Zero;
                }
                Self { trits: trit_copy }
            }
        }

        impl ops::Shr<usize> for $name {
            type Output = Self;
            #[allow(clippy::suspicious_arithmetic_impl)]
            fn shr(self, rhs: usize) -> Self::Output {
                let mut trit_copy = self.trits;
                for _ in 0..rhs {
                    for i in (1..$size).rev() {
                        trit_copy[i] = trit_copy[i - 1];
                    }
                    trit_copy[0] = Trit::Zero;
                }
                Self { trits: trit_copy }
            }
        }
        impl BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                let mut trit_copy = self.trits;
                let rhs_trit_copy = rhs.trits;
                for i in (0..$size) {
                    trit_copy[i] = trit_copy[i] | rhs_trit_copy[i];
                }
                Self { trits: trit_copy }
            }
        }
        impl BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                let mut trit_copy = self.trits;
                let rhs_trit_copy = rhs.trits;
                for i in (0..$size) {
                    trit_copy[i] = trit_copy[i] & rhs_trit_copy[i];
                }
                Self { trits: trit_copy }
            }
        }
        impl BitXor for $name {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                let mut trit_copy = self.trits;
                let rhs_trit_copy = rhs.trits;
                for i in (0..$size) {
                    trit_copy[i] = trit_copy[i] ^ rhs_trit_copy[i];
                }
                Self { trits: trit_copy }
            }
        }
        impl Not for $name {
            type Output = Self;

            fn not(self) -> Self::Output {
                let mut trit_copy = self.trits;
                for i in (0..$size) {
                    trit_copy[i] = !trit_copy[i];
                }
                Self { trits: trit_copy }
            }
        }

        impl_from_tryte!($name, u8, u16, u32, u64, u128);
    };
}

define_ternary_type!(T3, 3);
define_ternary_type!(T6, 6);
define_ternary_type!(T9, 9);
define_ternary_type!(T12, 12);
