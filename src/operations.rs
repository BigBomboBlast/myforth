#![allow(non_snake_case)]
// this file defines the operations and the type system 
// The stack can hold these number types
// Floats, Integers, and usize (so memory addresses work easy)

// any positive number is automatically a usize
// operations that result in floats or negative numbers are handled accordingingly
use std::fmt;
use std::ops::{Add, Sub};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub enum Type { // we have three types, Positive, Negatives, and Floats
    Neg(i32),
    Float(f32),
    Pos(usize), // i geuss `0` is included as a positive number in this language
    // since by "positive" I really mean "unsigned number"
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Neg(n) => write!(f, "{}", n),
            Type::Float(n) => write!(f, "{}", n),
            Type::Pos(n) => write!(f, "{}", n),
        }
    }
}

// defining how adding and subtracting work in the type system
// it covers ever single possible pair of types
// if there is a better way to handle this please let me know
// really it's just so that type conversions and destructuring are handled automatically
// whenever I use `+` or `-` on my Type enum variants
impl Add for Type {
    type Output = Type;
    fn add(self, other: Type) -> Type {
        match (self, other) {
            (Type::Neg(neg1), Type::Neg(neg2)) => {
                let result = neg1 + neg2;
                if result < 0 {
                    Type::Neg(result)
                } else {
                    Type::Pos(result as usize)
                }
            }
            (Type::Neg(neg), Type::Float(float)) => Type::Float((neg as f32) + float),
            (Type::Neg(neg), Type::Pos(pos)) => {
                let result = neg + (pos as i32);
                if result < 0 {
                    Type::Neg(result)
                } else {
                    Type::Pos(result as usize)
                }
            }
            (Type::Float(float), Type::Neg(neg)) => Type::Float(float + (neg as f32)),
            (Type::Float(float1), Type::Float(float2)) => Type::Float(float1 + float2),
            (Type::Float(float), Type::Pos(pos)) => Type::Float(float + (pos as f32)),

            (Type::Pos(pos), Type::Neg(neg)) => {
                let result = (pos as i32) + neg;
                if result < 0 {
                    Type::Neg(result)
                } else {
                    Type::Pos(result as usize)
                }
            }
            (Type::Pos(pos), Type::Float(float)) => Type::Float((pos as f32) + float),
            (Type::Pos(pos1), Type::Pos(pos2)) => Type::Pos(pos1 + pos2),
        }
    }
}

impl Sub for Type {
    type Output = Type;
    fn sub(self, other: Type) -> Type {
        match (self, other) {
            (Type::Neg(neg1), Type::Neg(neg2)) => {
                let result = neg1 - neg2;
                if result < 0 {
                    Type::Neg(result)
                } else {
                    Type::Pos(result as usize)
                }
            }
            (Type::Neg(neg), Type::Float(float)) => Type::Float((neg as f32) - float),
            (Type::Neg(neg), Type::Pos(pos)) => {
                let result = neg - (pos as i32);
                if result < 0 {
                    Type::Neg(result)
                } else {
                    Type::Pos(result as usize)
                }
            }
            (Type::Float(float), Type::Neg(neg)) => Type::Float(float - (neg as f32)),
            (Type::Float(float1), Type::Float(float2)) => Type::Float(float1 - float2),
            (Type::Float(float), Type::Pos(pos)) => Type::Float(float - (pos as f32)),

            (Type::Pos(pos), Type::Neg(neg)) => {
                let result = (pos as i32) - neg;
                if result < 0 {
                    Type::Neg(result)
                } else {
                    Type::Pos(result as usize)
                }
            }
            (Type::Pos(pos), Type::Float(float)) => Type::Float((pos as f32) - float),
            (Type::Pos(pos1), Type::Pos(pos2)) => {
                let result = (pos1 as i32) - (pos2 as i32);
                if result < 0 {
                    Type::Neg(result)
                } else {
                    Type::Pos(result as usize)
                }
            }
        }
    }
} // im gonna have some real fun with multiplication/division/exponentiation/modulo?? :O

impl PartialEq for Type {
    // NOTE: this takes references to the values in a comparison, 
    // that way I don't have to think about whether `==` takes ownership of the values
    // sounds like it'd be janky if it did
    fn eq(&self, other: &Type) -> bool {
        // self is the left value type, other is the right value type in an equality check
        match (self, other) {
            (Type::Neg(left), Type::Neg(right)) => left == right,
            // you have to dereference in order to do the type conversion
            // otherwise I believe you are converting the literal reference and not the value
            (Type::Neg(left), Type::Float(right)) => (*left as f32) == *right,
            (Type::Neg(left), Type::Pos(right)) => *left == (*right as i32),

            (Type::Float(left), Type::Neg(right)) => *left == (*right as f32),
            (Type::Float(left), Type::Float(right)) => left == right,
            (Type::Float(left), Type::Pos(right)) => *left == (*right as f32),

            (Type::Pos(left), Type::Neg(right)) => (*left as i32) == *right,
            (Type::Pos(left), Type::Float(right)) => (*left as f32) == *right,
            (Type::Pos(left), Type::Pos(right)) => left == right,
        }
    }
}

impl PartialOrd for Type {
    // same thing, takes references to the values in the comparison
    fn partial_cmp(&self, other: &Type) -> Option<Ordering> {
        // partial_cmp returns an Option<Ordering>
        // Ordering is an enum that contains if the left value was greater than/less than... whatever
        match (self, other) {
            (Type::Neg(left), Type::Neg(right)) => left.partial_cmp(right),
            (Type::Neg(left), Type::Float(right)) => (*left as f32).partial_cmp(right),
            // dereferenes to the the type conversion, then the partial_cmp will take reference to
            // the converted value
            (Type::Neg(left), Type::Pos(right)) => left.partial_cmp(&(*right as i32)),

            (Type::Float(left), Type::Neg(right)) => left.partial_cmp(&(*right as f32)),
            (Type::Float(left), Type::Float(right)) => left.partial_cmp(right),
            (Type::Float(left), Type::Pos(right)) => left.partial_cmp(&(*right as f32)),

            (Type::Pos(left), Type::Neg(right)) => (*left as i32).partial_cmp(right),
            (Type::Pos(left), Type::Float(right)) => (*left as f32).partial_cmp(right),
            (Type::Pos(left), Type::Pos(right)) => left.partial_cmp(right),
        }
    }
}

pub fn show_stack(stack: &Vec<Type>) {
    let mut output = String::from("[ ");
    for v in stack {
        let val = format!("{} ", v);
        output.push_str(&val);
    }
    output.push(']');
    println!("STACK TRACE: {}", output);
}

pub fn show_stack_debug(stack: &Vec<Type>) {
    let mut output = String::from("[ ");
    for v in stack {
        let val = format!("{:?} ", v);
        output.push_str(&val);
    }
    output.push(']');
    println!("STACK TRACE: {}", output);
}

macro_rules! pop {
    ($stack:expr) => {
        $stack.pop().expect("stack underflow")
    };
}

pub fn OP_ADD(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(y + x)
}
pub fn OP_SUB(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(y - x)
}
pub fn OP_EQ(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(Type::Pos((y == x) as usize))
}
pub fn OP_GT(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(Type::Pos((y > x) as usize))
}
pub fn OP_LT(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(Type::Pos((y < x) as usize))
}
pub fn OP_GTEQ(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(Type::Pos((y >= x) as usize))
}
pub fn OP_LTEQ(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(Type::Pos((y <= x) as usize))
}
pub fn OP_OUT(stack: &mut Vec<Type>) {
    let x = pop!(stack);

    println!("{}", x);
}
pub fn OP_DUP(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    stack.push(x);
    stack.push(x);
}

