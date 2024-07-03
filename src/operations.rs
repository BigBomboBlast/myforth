#![allow(non_snake_case)]

use crate::types::*;

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
pub fn OP_MUL(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(y * x)
}
pub fn OP_DIV(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(y / x)
}
pub fn OP_TOSIGNED(stack: &mut Vec<Type>) {
    match pop!(stack) {
        Type::Unsigned(n) => stack.push(Type::Unsigned(n as usize)),
        Type::Signed(n) => stack.push(Type::Unsigned(n as usize)),
        Type::Float(n) => stack.push(Type::Unsigned(n as usize)),
    }
}
pub fn OP_TOUNSIGNED(stack: &mut Vec<Type>) {
    match pop!(stack) {
        Type::Unsigned(n) => stack.push(Type::Signed(n as i64)),
        Type::Signed(n) => stack.push(Type::Signed(n as i64)),
        Type::Float(n) => stack.push(Type::Signed(n as i64)),
    }
}
pub fn OP_TOFLOAT(stack: &mut Vec<Type>) {
    match pop!(stack) {
        Type::Unsigned(n) => stack.push(Type::Float(n as f64)),
        Type::Signed(n) => stack.push(Type::Float(n as f64)),
        Type::Float(n) => stack.push(Type::Float(n as f64)),
    }
}
pub fn OP_EQ(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(Type::Unsigned((y == x) as usize))
}
pub fn OP_GT(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(Type::Unsigned((y > x) as usize))
}
pub fn OP_LT(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(Type::Unsigned((y < x) as usize))
}
pub fn OP_GTEQ(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(Type::Unsigned((y >= x) as usize))
}
pub fn OP_LTEQ(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);

    stack.push(Type::Unsigned((y <= x) as usize))
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
pub fn OP_SWAP(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);
    stack.push(x);
    stack.push(y);
}
pub fn OP_DROP(stack: &mut Vec<Type>) {
    pop!(stack);
}
pub fn OP_OVER(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);
    stack.push(y);
    stack.push(x);
    stack.push(y);
}
pub fn OP_ROTATE(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);
    let z = pop!(stack);
    stack.push(y);
    stack.push(x);
    stack.push(z);
}
pub unsafe fn OP_PRINTLN(stack: &mut Vec<Type>) {
    let Type::Unsigned(size) = pop!(stack) else {
        panic!(">:(");
    };
    let Type::Unsigned(string_ptr) = pop!(stack) else {
        panic!(">:(");
    };
    let string_as_bytes = std::slice::from_raw_parts((string_ptr as *const u8), size);
    if let Ok(s) = std::str::from_utf8(string_as_bytes) {
        println!("{}", s);
    } else {
        panic!("blame the guy who wrote this interpreter, something went wrong with OP_PRINTLN");
    }
    stack.push(Type::Unsigned(string_ptr));
    stack.push(Type::Unsigned(size))

}
