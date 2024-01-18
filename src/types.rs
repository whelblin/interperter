use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Display;
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
#[derive(Clone)]
pub enum Types{
    Number(f32),
    String(String),
    Bool(bool),
    Array(Vec<Types>),
    None
}
impl Display for Types{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self{
            Types::Number(i) => {write!(f, "{}", i)},
            Types::String(i) =>{write!(f, "{}", i)},
            Types::Bool(i) => {write!(f, "{}", i)},
            Types::None => return Err(std::fmt::Error),
            Types::Array(i) => {
                print!("[");
                for item in &mut i.clone()[..i.len()-1]{
                    print!("{},", item);
                }
                print!("{}]", i.last().expect("Out OF Bounds"));
            Ok(())
            },
    }
}
}

impl Add for Types {
    type Output = Types;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs){
            (Types::Number(l), Types::Number(r)) => {return Types::Number(l+r)},
            (Types::Number(l), Types::String(r)) => {return Types::String(l.to_string() + &r)},
            (Types::Number(_), Types::None) => Types::None,
            (Types::String(l), Types::Number(r)) => {return Types::String(r.to_string() + &l)},
            (Types::String(l), Types::String(r)) => {return Types::String(l.add(&r));},
            (Types::String(_), Types::None) => Types::None,
            (Types::None, Types::Number(_)) => Types::None,
            (Types::None, Types::String(_)) => Types::None,
            (Types::None, Types::None) =>Types::None,
            (Types::Number(_), Types::Bool(_)) => Types::None,
            (Types::String(_), Types::Bool(_)) => Types::None,
            (Types::Bool(_), Types::Number(_)) => Types::None,
            (Types::Bool(_), Types::String(_)) => Types::None,
            (Types::Bool(_), Types::Bool(_)) => Types::None,
            (Types::Bool(_), Types::None) => Types::None,
            (Types::None, Types::Bool(_)) => Types::None,
            (Types::Number(_), Types::Array(_)) => Types::None,
            (Types::String(_), Types::Array(_)) => Types::None,
            (Types::Bool(_), Types::Array(_)) => Types::None,
            (Types::Array(_), Types::Number(_)) => Types::None,
            (Types::Array(_), Types::String(_)) => Types::None,
            (Types::Array(_), Types::Bool(_)) => Types::None,
            (Types::Array(_), Types::Array(_)) => Types::None,
            (Types::Array(_), Types::None) => Types::None,
            (Types::None, Types::Array(_)) => Types::None,
        }
    }
}

impl Sub for Types {
    type Output = Types;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs){
            (Types::Number(l), Types::Number(r)) => {return Types::Number(l-r)},
            (Types::Number(l), Types::String(r)) => {return Types::String((l.to_be_bytes()[0] - r.as_bytes()[0]).to_string())},
            (Types::Number(_), Types::None) => Types::None,
            (Types::String(l), Types::Number(r)) => {return Types::String((l.as_bytes()[0] - r.to_be_bytes()[0]).to_string())},
            (Types::String(l), Types::String(r)) => {return Types::String((l.as_bytes()[0] - r.as_bytes()[0]).to_string());},
            (Types::String(_), Types::None) => Types::None,
            (Types::None, Types::Number(_)) => Types::None,
            (Types::None, Types::String(_)) => Types::None,
            (Types::None, Types::None) =>Types::None,
            (Types::Number(_), Types::Bool(_)) => Types::None,
            (Types::String(_), Types::Bool(_)) => Types::None,
            (Types::Bool(_), Types::Number(_)) => Types::None,
            (Types::Bool(_), Types::String(_)) => Types::None,
            (Types::Bool(_), Types::Bool(_)) => Types::None,
            (Types::Bool(_), Types::None) => Types::None,
            (Types::None, Types::Bool(_)) => Types::None,
            (Types::Number(_), Types::Array(_)) => Types::None,
            (Types::String(_), Types::Array(_)) => Types::None,
            (Types::Bool(_), Types::Array(_)) => Types::None,
            (Types::Array(_), Types::Number(_)) => Types::None,
            (Types::Array(_), Types::String(_)) => Types::None,
            (Types::Array(_), Types::Bool(_)) => Types::None,
            (Types::Array(_), Types::Array(_)) => Types::None,
            (Types::Array(_), Types::None) => Types::None,
            (Types::None, Types::Array(_)) => Types::None,
        }
    }
}

impl Mul for Types {
    type Output = Types;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs){
            (Types::Number(l), Types::Number(r)) => {return Types::Number(l*r)},
            (Types::Number(l), Types::String(r)) => {return Types::String((l.to_be_bytes()[0] * r.as_bytes()[0]).to_string())},
            (Types::String(l), Types::Number(r)) => {return Types::String((l.as_bytes()[0] * r.to_be_bytes()[0]).to_string())},
            (Types::String(l), Types::String(r)) => {return Types::String((l.as_bytes()[0] * r.as_bytes()[0]).to_string());},
            (Types::Number(_), Types::None) => Types::None,
            (Types::String(_), Types::None) => Types::None,
            (Types::None, Types::Number(_)) => Types::None,
            (Types::None, Types::String(_)) => Types::None,
            (Types::None, Types::None) =>Types::None,
            (Types::Number(_), Types::Bool(_)) => Types::None,
            (Types::String(_), Types::Bool(_)) => Types::None,
            (Types::Bool(_), Types::Number(_)) => Types::None,
            (Types::Bool(_), Types::String(_)) => Types::None,
            (Types::Bool(_), Types::Bool(_)) => Types::None,
            (Types::Bool(_), Types::None) => Types::None,
            (Types::None, Types::Bool(_)) => Types::None,
            (Types::Number(_), Types::Array(_)) => Types::None,
            (Types::String(_), Types::Array(_)) => Types::None,
            (Types::Bool(_), Types::Array(_)) => Types::None,
            (Types::Array(_), Types::Number(_)) => Types::None,
            (Types::Array(_), Types::String(_)) => Types::None,
            (Types::Array(_), Types::Bool(_)) => Types::None,
            (Types::Array(_), Types::Array(_)) => Types::None,
            (Types::Array(_), Types::None) => Types::None,
            (Types::None, Types::Array(_)) => Types::None,
        }
    }
}

impl Div for Types {
    type Output = Types;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs){
            (Types::Number(l), Types::Number(r)) => {return Types::Number(l/r)},
            (Types::Number(l), Types::String(r)) => {return Types::String((l.to_be_bytes()[0] / r.as_bytes()[0]).to_string())},
            (Types::String(l), Types::Number(r)) => {return Types::String((l.as_bytes()[0] / r.to_be_bytes()[0]).to_string())},
            (Types::String(l), Types::String(r)) => {return Types::String((l.as_bytes()[0] / r.as_bytes()[0]).to_string());},
            (Types::Number(_), Types::None) => Types::None,
            (Types::String(_), Types::None) => Types::None,
            (Types::None, Types::Number(_)) => Types::None,
            (Types::None, Types::String(_)) => Types::None,
            (Types::None, Types::None) =>Types::None,
            (Types::Number(_), Types::Bool(_)) => Types::None,
            (Types::String(_), Types::Bool(_)) => Types::None,
            (Types::Bool(_), Types::Number(_)) => Types::None,
            (Types::Bool(_), Types::String(_)) => Types::None,
            (Types::Bool(_), Types::Bool(_)) => Types::None,
            (Types::Bool(_), Types::None) => Types::None,
            (Types::None, Types::Bool(_)) => Types::None,
            (Types::Number(_), Types::Array(_)) => Types::None,
            (Types::String(_), Types::Array(_)) => Types::None,
            (Types::Bool(_), Types::Array(_)) => Types::None,
            (Types::Array(_), Types::Number(_)) => Types::None,
            (Types::Array(_), Types::String(_)) => Types::None,
            (Types::Array(_), Types::Bool(_)) => Types::None,
            (Types::Array(_), Types::Array(_)) => Types::None,
            (Types::Array(_), Types::None) => Types::None,
            (Types::None, Types::Array(_)) => Types::None,
        }
    }
}
