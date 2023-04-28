use std::ops::{
    // Add, Sub, Mul, Div, Neg, // unused
    Not,
};
use std::cmp::{
    PartialOrd, Ordering
};
use std::fmt;

#[derive(/* Debug, */ Copy, Clone, PartialEq)] // numbers are f64s here, so no Eq
pub enum Value {
    Number(f64), // pub unnecessary here
    Bool(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{:.3}", num),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
        }
        // write!(f, "{:.3?}", self)
    }
}

impl fmt::Debug for Value { // for stack display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{:.3}", num),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
        }
    }
}

pub type ValueOpnError = ();
pub type ValueOpnResult = Result<Value, ValueOpnError>;

// impl Value {
//     pub fn number(num: f64) -> Self {
//         Value::Number(num)
//     }
//     pub fn bool(b: bool) -> Self {
//         Value::Bool(b)
//     }
// }


impl Value {
    /// Method for Instr::Add
    pub fn checked_add(self, other: Self) -> ValueOpnResult {
        // want a try block like this:
        // try {
        //     Self::Number(
        //         f64::try_from(self)? + f64::try_from(other)?
        //     )
        // }
        Ok(
            Self::Number(
                f64::try_from(self)? + f64::try_from(other)?
            )
        )
    }

    /// Method for Instr::Subtract
    pub fn checked_sub(self, other: Self) -> ValueOpnResult {
        Ok(
            Self::Number(
                f64::try_from(self)? - f64::try_from(other)?
            )
        )
    }

    /// Method for Instr::Multiply
    pub fn checked_mul(self, other: Self) -> ValueOpnResult {
        Ok(
            Self::Number(
                f64::try_from(self)? * f64::try_from(other)?
            )
        )
    }

    /// Method for Instr::Divide
    pub fn checked_div(self, other: Self) -> ValueOpnResult {
        Ok(
            Self::Number(
                f64::try_from(self)? / f64::try_from(other)?
            )
        )
    }

    /// Method for Instr::Negate
    pub fn checked_neg(self) -> ValueOpnResult {
        Ok(
            Self::Number(
                -f64::try_from(self)?
            )
        )
    }

    /// Method for Instr::And
    pub fn and(self, other: Self) -> Self {
        Value::Bool(
            bool::from(self)
            && bool::from(other)
        )
    }

    /// Method for Instr::Or
    pub fn or(self, other: Self) -> Self {
        Value::Bool(
            bool::from(self)
            || bool::from(other)
        )
    }
}

/// Trait for Instr::Not
impl Not for Value {
    type Output = Self;
    fn not(self) -> Self::Output {
        Value::Bool(
            bool::from(self).not()
        )
    }
}

/// Trait for Instr::{Equal, NotEqual} -- PartialEq (already derived)

/// Trait for Instr::{Greater, Less, /* GreaterEqual, LessEqual */}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let (Value::Number(a), Value::Number(b)) = (self, other) {
            a.partial_cmp(b)
        } else { None }
    }
}

// Conversions

// impl From<Value> for f64 {
//     fn from(value: Value) -> f64 {
//         match value {
//             Value::Number(num) => num,
//             Value::Bool(b) => if b { 1.0 } else { 0.0 },
//             Value::Nil => 0.0,
//         }
//     }
// }

impl From<Value> for bool {
    fn from(value: Value) -> bool {
        match value {
            Value::Number(num) => num != 0.0,
            Value::Bool(b) => b,
            Value::Nil => false,
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(num) => Ok(num),
            _ => Err(()),
        }
    }
}

// impl TryFrom<Value> for bool {
//     type Error = ();
//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         match value {
//             Value::Bool(b) => Ok(b),
//             _ => Err(()),
//         }
//     }
// }