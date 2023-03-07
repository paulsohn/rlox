use std::fmt;

pub struct Value(f64);

impl Value {
    pub fn numeric(num: f64) -> Self{
        Value(num)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
