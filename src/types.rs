use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub enum Num {
    Integer(i64),
    Float(f64),
}

#[derive(Clone, Debug)]
pub enum Type {
    Null,
    Boolean(bool),
    Number(Num),
    Str(String),
    List(Vec<Type>),
}

pub fn is_falsy(t: Type) -> bool {
    match t {
        Type::Number(n) => {
            match n {
                n if n == Num::Integer(0) => return true,
                n if n == Num::Float(0 as f64) => return true,
                _ => return false,
            }
        }
        Type::Str(s) => return s == "",
        Type::Boolean(b) => return b == false,
        Type::Null => return true,
        _ => false,
    }
}

pub fn show_list(stack: &Vec<Type>) {
    let mut output = String::from("[ ");
    for v in stack {
        let val = format!("{} ", v);
        output.push_str(&val);
    }
    output.push(']');
    println!("STACK TRACE: {}", output);
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Number(n) => {
                match n {
                    Num::Integer(n) => write!(f, "{}", n),
                    Num::Float(n) => write!(f, "{}", n),
                }
            }
            Type::Str(n) => write!(f, "{}", n),
            Type::Boolean(b) => write!(f, "{}", b),
            Type::Null => write!(f, "∅"),
            Type::List(l) => {
                write!(f, "{:?}", l)
            }
        }
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Num::Integer(n) => write!(f, "{}", n),
            Num::Float(n) => write!(f, "{}", n),
        }
    }
}

impl Add for Num {
    type Output = Num;
    fn add(self, other: Num) -> Num {
        match (self, other) {
            (Num::Integer(n1), Num::Integer(n2)) => Num::Integer(n1 + n2),
            (Num::Integer(n), Num::Float(f)) => Num::Float(n as f64 + f), 

            (Num::Float(f1), Num::Float(f2)) => Num::Float(f1 + f2),
            (Num::Float(f), Num::Integer(n)) => Num::Float(f + n as f64),
        }
    }
}
impl Sub for Num {
    type Output = Num;
    fn sub(self, other: Num) -> Num {
        match (self, other) {
            (Num::Integer(n1), Num::Integer(n2)) => Num::Integer(n1 - n2),
            (Num::Integer(n), Num::Float(f)) => Num::Float(n as f64 - f), 

            (Num::Float(f1), Num::Float(f2)) => Num::Float(f1 - f2),
            (Num::Float(f), Num::Integer(n)) => Num::Float(f - n as f64),
        }
    }
}
impl Mul for Num {
    type Output = Num;
    fn mul(self, other: Num) -> Num {
        match (self, other) {
            (Num::Integer(n1), Num::Integer(n2)) => Num::Integer(n1 * n2),
            (Num::Integer(n), Num::Float(f)) => Num::Float(n as f64 * f), 

            (Num::Float(f1), Num::Float(f2)) => Num::Float(f1 * f2),
            (Num::Float(f), Num::Integer(n)) => Num::Float(f * n as f64),
        }
    }
}

impl Div for Num {
    type Output = Num;
    fn div(self, other: Num) -> Num {
        match (self, other) {
            (Num::Integer(n1), Num::Integer(n2)) => Num::Float((n1 / n2) as f64),
            (Num::Integer(n), Num::Float(f)) => Num::Float(n as f64 / f), 

            (Num::Float(f1), Num::Float(f2)) => Num::Float(f1 / f2),
            (Num::Float(f), Num::Integer(n)) => Num::Float(f / n as f64),
        }
    }
}

impl PartialEq for Num {
    fn eq(&self, other: &Num) -> bool {
        match (self, other) {
            (Num::Integer(left), Num::Integer(right)) => left == right,
            (Num::Integer(left), Num::Float(right)) => (*left as f64) == *right,

            (Num::Float(left), Num::Float(right)) => left == right,
            (Num::Float(left), Num::Integer(right)) => *left == (*right as f64),
        }
    }
}

impl PartialOrd for Num {
    // same thing, takes references to the values in the comparison
    fn partial_cmp(&self, other: &Num) -> Option<Ordering> {
        // partial_cmp returns an Option<Ordering>
        // Ordering is an enum that contains if the left value was greater than/less than... whatever
        match (self, other) {
            (Num::Integer(left), Num::Integer(right)) => left.partial_cmp(right),
            (Num::Integer(left), Num::Float(right)) => (*left as f64).partial_cmp(right),

            (Num::Float(left), Num::Float(right)) => left.partial_cmp(right),
            (Num::Float(left), Num::Integer(right)) => left.partial_cmp(&(*right as f64)),
        }
    }
}
