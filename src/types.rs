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
/// allows collect to work with iterators on Type
impl FromIterator<Types> for Types {
    fn from_iter<T: IntoIterator<Item = Types>>(iter: T) -> Types {
        let mut result = Vec::new();
        for i in iter{
            result.push(i)
        }
       Types::Array(result)
}
}
impl Iterator for Types{
    type Item = Types;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.clone())
    }
}
/// allows for printing of Type varaiants
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
            (Types::String(l), Types::Number(r)) => {return Types::String(r.to_string() + &l)},
            (Types::String(l), Types::String(r)) => {return Types::String(l.add(&r));},
            _ => {return Types::None} // all other cases
        }
    }
}

impl Sub for Types {
    type Output = Types;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs){
            (Types::Number(l), Types::Number(r)) => {return Types::Number(l-r)},
            (Types::Number(l), Types::String(r)) => {return Types::String((l.to_be_bytes()[0] - r.as_bytes()[0]).to_string())},
            (Types::String(l), Types::Number(r)) => {return Types::String((l.as_bytes()[0] - r.to_be_bytes()[0]).to_string())},
            (Types::String(l), Types::String(r)) => {return Types::String((l.as_bytes()[0] - r.as_bytes()[0]).to_string());},
            _ =>{return Types::None;} // all other cases
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
            _ =>{return Types::None;} // all other cases
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
            _ =>{return Types::None;} // all other cases
        }
    }
}
