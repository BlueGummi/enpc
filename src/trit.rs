use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};
#[derive(Clone, Copy, PartialEq)]
pub enum Trit {
    Zero,
    One,
    Two,
}

impl fmt::Display for Trit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trit::Zero => write!(f, "0")?,
            Trit::One => write!(f, "1")?,
            Trit::Two => write!(f, "2")?,
        }
        Ok(())
    }
}

impl BitOr for Trit {
    type Output = Trit;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::Two, _) | (_, Trit::Two) => Trit::Two, // two dominates
            (Trit::One, _) | (_, Trit::One) => Trit::One, // one dominates if no Two
            (Trit::Zero, Trit::Zero) => Trit::Zero,       // both Zero
        }
    }
}
impl BitXor for Trit {
    type Output = Trit;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::Zero, x) | (x, Trit::Zero) => x,
            (Trit::One, Trit::One) => Trit::Zero, // 1 ⊕ 1 = 0
            (Trit::Two, Trit::Two) => Trit::Zero, // 2 ⊕ 2 = 0
            (Trit::One, Trit::Two) | (Trit::Two, Trit::One) => Trit::Two, // 1 ⊕ 2 = 2
        }
    }
}
impl BitAnd for Trit {
    type Output = Trit;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::Zero, _) | (_, Trit::Zero) => Trit::Zero, // zero dominates (absorbing element)
            (Trit::One, Trit::One) => Trit::One,
            (Trit::One, Trit::Two) | (Trit::Two, Trit::One) => Trit::One,
            (Trit::Two, Trit::Two) => Trit::Two,
        }
    }
}
impl Not for Trit {
    type Output = Trit;
    fn not(self) -> Self::Output {
        match self {
            Trit::Zero => Trit::Two,
            Trit::One => Trit::One,
            Trit::Two => Trit::Two,
        }
    }
}
