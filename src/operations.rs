#![allow(non_snake_case)]
// The stack can hold these number types
// Floats, Integers, and usize (so memory addresses work easy)

// any positive number is automatically a usize
// operations that result in floats or negative numbers are handled accordingingly
use std::fmt;
use std::ops::{Add, Sub};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub enum V { // `V` stands for `Value`
    I(i32),
    F(f32),
    U(usize),
}

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            V::I(n) => write!(f, "{}", n),
            V::F(n) => write!(f, "{}", n),
            V::U(n) => write!(f, "{}", n),
        }
    }
}

// This will make it so that the basic `+` and `-` will work on the any of the V enum variants
// it handles the destructuring and type conversions automatically
impl Add for V {
    type Output = V;
    fn add(self, other: V) -> V {
        match (self, other) {
            (V::I(int1), V::I(int2)) => {
                let result = int1 + int2;
                if result < 0 {
                    V::I(result)
                } else {
                    V::U(result as usize)
                }
            }
            (V::I(int), V::F(float)) => V::F((int as f32) + float),
            (V::I(int), V::U(u)) => {
                let result = int + (u as i32);
                if result < 0 {
                    V::I(result)
                } else {
                    V::U(result as usize)
                }
            }
            (V::F(float), V::I(int)) => V::F(float + (int as f32)),
            (V::F(float1), V::F(float2)) => V::F(float1 + float2),
            (V::F(float), V::U(u)) => V::F(float + (u as f32)),

            (V::U(u), V::I(int)) => {
                let result = (u as i32) + int;
                if result < 0 {
                    V::I(result)
                } else {
                    V::U(result as usize)
                }
            }
            (V::U(u), V::F(float)) => V::F((u as f32) + float),
            (V::U(u1), V::U(u2)) => V::U(u1 + u2),
        }
    }
}

impl Sub for V {
    type Output = V;
    fn sub(self, other: V) -> V {
        match (self, other) {
            (V::I(int1), V::I(int2)) => V::I(int1 - int2),
            (V::I(int), V::F(float)) => V::F((int as f32) - float),
            (V::I(int), V::U(u)) => {
                let result = int - (u as i32);
                if result < 0 {
                    V::I(result)
                } else {
                    V::U(result as usize)
                }
            }
            (V::F(float), V::I(int)) => V::F(float - (int as f32)),
            (V::F(float1), V::F(float2)) => V::F(float1 - float2),
            (V::F(float), V::U(u)) => V::F(float - (u as f32)),

            (V::U(u), V::I(int)) => {
                let result = (u as i32) - int;
                if result < 0 {
                    V::I(result)
                } else {
                    V::U(result as usize)
                }
            }
            (V::U(u), V::F(float)) => V::F((u as f32) - float),
            (V::U(u1), V::U(u2)) => {
                let result = (u1 as i32) - (u2 as i32);
                if result < 0 {
                    V::I(result)
                } else {
                    V::U(result as usize)
                }
            }
        }
    }
}

impl PartialEq for V {
    // NOTE: this takes referencces to the values in a comparison, 
    // that way I don't have to think about whether `==` takes ownership of the values
    // sounds like it'd be janky if it did
    fn eq(&self, other: &V) -> bool {
        // self is the left value type, other is the right value type in an equality check
        match (self, other) {
            (V::I(left), V::I(right)) => left == right,
            // you have to dereference in order to do the type conversion
            // otherwise I believe you are converting the literal reference and not the value
            (V::I(left), V::F(right)) => (*left as f32) == *right,
            (V::I(left), V::U(right)) => *left == (*right as i32),

            (V::F(left), V::I(right)) => *left == (*right as f32),
            (V::F(left), V::F(right)) => left == right,
            (V::F(left), V::U(right)) => *left == (*right as f32),

            (V::U(left), V::I(right)) => (*left as i32) == *right,
            (V::U(left), V::F(right)) => (*left as f32) == *right,
            (V::U(left), V::U(right)) => left == right,
        }
    }
}

impl PartialOrd for V {
    // same thing, takes references to the values in the comparison
    fn partial_cmp(&self, other: &V) -> Option<Ordering> {
        // partial_cmp returns an Option<Ordering>
        // Ordering is an enum that contains if the left value was greater than/less than... whatever
        match (self, other) {
            (V::I(left), V::I(right)) => left.partial_cmp(right),
            (V::I(left), V::F(right)) => (*left as f32).partial_cmp(right),
            // dereferenes to the the type conversion, then the partial_cmp will take reference to
            // the converted value
            (V::I(left), V::U(right)) => left.partial_cmp(&(*right as i32)),

            (V::F(left), V::I(right)) => left.partial_cmp(&(*right as f32)),
            (V::F(left), V::F(right)) => left.partial_cmp(right),
            (V::F(left), V::U(right)) => left.partial_cmp(&(*right as f32)),

            (V::U(left), V::I(right)) => (*left as i32).partial_cmp(right),
            (V::U(left), V::F(right)) => (*left as f32).partial_cmp(right),
            (V::U(left), V::U(right)) => left.partial_cmp(right),
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

    stack.push(V::U((y == x) as usize))
}
pub fn OP_GT(stack: &mut Vec<V>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(V::U((y > x) as usize))
}
pub fn OP_LT(stack: &mut Vec<V>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(V::U((y < x) as usize))
}
pub fn OP_GTEQ(stack: &mut Vec<V>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(V::U((y >= x) as usize))
}
pub fn OP_LTEQ(stack: &mut Vec<V>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(V::U((y <= x) as usize))
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

