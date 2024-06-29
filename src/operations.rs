#![allow(non_snake_case)]
// The stack can hold these number types
// Floats, Integers, and Bytes (u8)

// Any number between 0 and 255 is a byte automatically
// Any negative number or any number above 255 is an i32
// Any decimal number is a float

// as long as adding/subtracting values is in between 0 and 255 they should result in u8 types
// and any operation involving floats will always result in a float
use std::fmt;
use std::ops::{Add, Sub};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub enum V { // `V` stands for `Value`
    I(i32),
    F(f32),
    B(u8),
}

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            V::I(n) => write!(f, "{}", n),
            V::F(n) => write!(f, "{}", n),
            V::B(n) => write!(f, "{}", n),
        }
    }
}

// if there is a better way of implementing these traits pls let me know
// I think this will make it so that I don't have to wrory about destructuring when implementing
// the instructions
impl Add for V {
    type Output = V;
    fn add(self, other: V) -> V {
        match (self, other) {
            (V::I(int1), V::I(int2)) => {
                let result = int1 + int2;
                if result < 0 || result > 255 {
                    V::I(result)
                } else {
                    V::B(result as u8)
                }
            }
            (V::I(int), V::F(float)) => V::F((int as f32) + float),
            (V::I(int), V::B(byte)) => {
                let result = int + (byte as i32);
                if result < 0 || result > 255 {
                    V::I(result)
                } else {
                    V::B(result as u8)
                }
            }
            (V::F(float), V::I(int)) => V::F(float + (int as f32)),
            (V::F(float1), V::F(float2)) => V::F(float1 + float2),
            (V::F(float), V::B(byte)) => V::F(float + (byte as f32)),

            (V::B(byte), V::I(int)) => {
                let result = (byte as i32) + int;
                if result < 0 || result > 255 {
                    V::I(result)
                } else {
                    V::B(result as u8)
                }
            }
            (V::B(byte), V::F(float)) => V::F((byte as f32) + float),
            (V::B(byte1), V::B(byte2)) => {
                let result = (byte1 as i32) + (byte2 as i32);
                if result < 0 || result > 255 {
                    V::I(result)
                } else {
                    V::B(byte1 + byte2)
                }
            }
        }
    }
}

impl Sub for V {
    type Output = V;
    fn sub(self, other: V) -> V {
        match (self, other) {
            (V::I(int1), V::I(int2)) => V::I(int1 - int2),
            (V::I(int), V::F(float)) => V::F((int as f32) - float),
            (V::I(int), V::B(byte)) => {
                let result = int - (byte as i32);
                if result < 0 {
                    V::I(result)
                } else {
                    V::B(result as u8)
                }
            }
            (V::F(float), V::I(int)) => V::F(float - (int as f32)),
            (V::F(float1), V::F(float2)) => V::F(float1 - float2),
            (V::F(float), V::B(byte)) => V::F(float - (byte as f32)),

            (V::B(byte), V::I(int)) => {
                let result = (byte as i32) - int;
                if result < 0 {
                    V::I(result)
                } else {
                    V::B(result as u8)
                }
            }
            (V::B(byte), V::F(float)) => V::F((byte as f32) - float),
            (V::B(byte1), V::B(byte2)) => {
                let result = (byte1 as i32) - (byte2 as i32);
                if result < 0 {
                    V::I(result)
                } else {
                    V::B(byte1 - byte2)
                }
            }
        }
    }
}

impl PartialEq for V {
    fn eq(&self, other: &V) -> bool {
        match (self, other) {
            (V::I(lhs), V::I(rhs)) => lhs == rhs,
            (V::I(lhs), V::F(rhs)) => (*lhs as f32) == *rhs,
            (V::I(lhs), V::B(rhs)) => *lhs == (*rhs as i32),

            (V::F(lhs), V::I(rhs)) => *lhs == (*rhs as f32),
            (V::F(lhs), V::F(rhs)) => lhs == rhs,
            (V::F(lhs), V::B(rhs)) => *lhs == (*rhs as f32),

            (V::B(lhs), V::I(rhs)) => (*lhs as i32) == *rhs,
            (V::B(lhs), V::F(rhs)) => (*lhs as f32) == *rhs,
            (V::B(lhs), V::B(rhs)) => lhs == rhs,
        }
    }
}

impl PartialOrd for V {
    fn partial_cmp(&self, other: &V) -> Option<Ordering> {
        match (self, other) {
            (V::I(lhs), V::I(rhs)) => lhs.partial_cmp(rhs),
            (V::I(lhs), V::F(rhs)) => (*lhs as f32).partial_cmp(rhs),
            (V::I(lhs), V::B(rhs)) => lhs.partial_cmp(&(*rhs as i32)),

            (V::F(lhs), V::I(rhs)) => lhs.partial_cmp(&(*rhs as f32)),
            (V::F(lhs), V::F(rhs)) => lhs.partial_cmp(rhs),
            (V::F(lhs), V::B(rhs)) => lhs.partial_cmp(&(*rhs as f32)),

            (V::B(lhs), V::I(rhs)) => (*lhs as i32).partial_cmp(rhs),
            (V::B(lhs), V::F(rhs)) => (*lhs as f32).partial_cmp(rhs),
            (V::B(lhs), V::B(rhs)) => lhs.partial_cmp(rhs),
        }
    }
}

pub fn show_stack(stack: &Vec<V>) {
    let mut output = String::from("[ ");
    for v in stack {
        let val = format!("{} ", v);
        output.push_str(&val);
    }
    output.push(']');
    println!("STACK TRACE: {}", output);
}

pub fn show_stack_debug(stack: &Vec<V>) {
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

pub fn OP_ADD(stack: &mut Vec<V>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(y + x)
}
pub fn OP_SUB(stack: &mut Vec<V>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(y - x)
}
pub fn OP_EQ(stack: &mut Vec<V>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(V::B((y == x) as u8))
}
pub fn OP_GT(stack: &mut Vec<V>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(V::B((y > x) as u8))
}
pub fn OP_LT(stack: &mut Vec<V>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(V::B((y < x) as u8))
}
pub fn OP_GTEQ(stack: &mut Vec<V>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(V::B((y >= x) as u8))
}
pub fn OP_LTEQ(stack: &mut Vec<V>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(V::B((y <= x) as u8))
}
pub fn OP_OUT(stack: &mut Vec<V>) {
    let x = pop!(stack);

    println!("{}", x);
}
pub fn OP_DUP(stack: &mut Vec<V>) {
    let x = pop!(stack);
    stack.push(x);
    stack.push(x);
}

