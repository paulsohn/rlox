use std::ops::{
    Add, Sub, Mul, Div, Neg,
};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)] // numbers are f64s here, so no Eq
pub enum Value {
    Number(f64), // pub unnecessary here
    Bool(bool),
    Nil,
}

// impl Value {
//     pub fn number(num: f64) -> Self {
//         Value::Number(num)
//     }
//     pub fn bool(b: bool) -> Self {
//         Value::Bool(b)
//     }
// }

/// Trait for Instr::Add
impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Number(
            f64::from(self) + f64::from(other)
        )
    }
}

/// Trait for Instr::Subtract
impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Number(
            f64::from(self) - f64::from(other)
        )
    }
}

/// Trait for Instr::Multiply
impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Number(
            f64::from(self) * f64::from(other)
        )
    }
}

/// Trait for Instr::Divide
impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::Number(
            f64::from(self) / f64::from(other)
        )
    }
}

/// Trait for Instr::Negate
impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Number(
            -f64::from(self)
        )
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match self {
        //     Self::Number(num) => write!(f, "{}", num),
        //     Self::Bool(b) => write!(f, "{}", b),
        //     Self::Nil => write!(f, "nil"),
        // }
        write!(f, "{:.3?}", self)
    }
}

// Conversions

impl From<Value> for f64 {
    fn from(val: Value) -> f64 {
        match val {
            Value::Number(num) => num,
            Value::Bool(b) => if b { 1.0 } else { 0.0 },
            Value::Nil => 0.0,
        }
    }
}

impl From<Value> for bool {
    fn from(val: Value) -> bool {
        match val {
            Value::Number(num) => num != 0.0,
            Value::Bool(b) => b,
            Value::Nil => false,
        }
    }
}