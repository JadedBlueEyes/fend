use crate::num::Number;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone)]
pub enum Value {
    Num(Number),
    Func(String),
}

impl Value {
    pub fn expect_num(&self) -> Result<Number, String> {
        match self {
            Value::Num(bigrat) => Ok(bigrat.clone()),
            _ => Err("Expected a number".to_string()),
        }
    }

    pub fn apply(&self, other: Value) -> Result<Value, String> {
        Ok(Value::Num(match self {
            Value::Num(n) => n.clone() * other.expect_num()?,
            Value::Func(name) => {
                if name == "sqrt" {
                    other.expect_num()?.root_n(&2.into())?
                } else if name == "cbrt" {
                    other.expect_num()?.root_n(&3.into())?
                } else if name == "abs" {
                    let arg = other.expect_num()?;
                    if arg < 0.into() {
                        -arg
                    } else {
                        arg
                    }
                } else {
                    return Err(format!("Unknown function '{}'", name));
                }
            }
        }))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::Num(n) => write!(f, "{}", n)?,
            Value::Func(name) => write!(f, "{}", name)?,
        }
        Ok(())
    }
}