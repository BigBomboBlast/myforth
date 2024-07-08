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

fn destructure_number(n: Type) -> Num {
    match n {
        Type::Number(n) => return n,
        _ => panic!("Expected number"),
        
    }
}
macro_rules! pop {
    ($stack:expr) => {
        $stack.pop().expect("stack underflow").clone()
    };
}
macro_rules! pop_num {
    ($stack:expr) => {
        destructure_number($stack.pop().expect("stack underflow"))
    };
}

pub fn OP_ADD(stack: &mut Vec<Type>) {
    let x = pop_num!(stack);
    let y = pop_num!(stack);

    stack.push(Type::Number(y + x))
}
pub fn OP_SUB(stack: &mut Vec<Type>) {
    let x = pop_num!(stack);
    let y = pop_num!(stack);

    stack.push(Type::Number(y - x))
}
pub fn OP_MUL(stack: &mut Vec<Type>) {
    let x = pop_num!(stack);
    let y = pop_num!(stack);

    stack.push(Type::Number(y * x))
}
pub fn OP_DIV(stack: &mut Vec<Type>) {
    let x = pop_num!(stack);
    let y = pop_num!(stack);

    stack.push(Type::Number(y / x))
}
pub fn OP_FLOOR(stack: &mut Vec<Type>) {
    let num = pop_num!(stack);
    match num {
        Num::Float(x) => stack.push(Type::Number(Num::Integer(x as i64))),
        Num::Integer(x) => stack.push(Type::Number(num)),
    }
}
pub fn OP_EQ(stack: &mut Vec<Type>) {
    let x = pop_num!(stack);
    let y = pop_num!(stack);

    stack.push(Type::Boolean(y == x))
}
pub fn OP_GT(stack: &mut Vec<Type>) {
    let x = pop_num!(stack);
    let y = pop_num!(stack);

    stack.push(Type::Boolean(y > x))
}
pub fn OP_LT(stack: &mut Vec<Type>) {
    let x = pop_num!(stack);
    let y = pop_num!(stack);

    stack.push(Type::Boolean(y < x))
}
pub fn OP_GTEQ(stack: &mut Vec<Type>) {
    let x = pop_num!(stack);
    let y = pop_num!(stack);

    stack.push(Type::Boolean(y <= x));
}
pub fn OP_LTEQ(stack: &mut Vec<Type>) {
    let x = pop_num!(stack);
    let y = pop_num!(stack);

    stack.push(Type::Boolean(y >= x));
}
pub fn OP_OUT(stack: &mut Vec<Type>) {
    let x = pop!(stack);

    println!("{}", x);
}
pub fn OP_DUP(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    stack.push(x.clone());
    stack.push(x);
}
pub fn OP_SWAP(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);
    stack.push(x);
    stack.push(y);
}
pub fn OP_DROP(stack: &mut Vec<Type>) {
    pop_num!(stack);
}
pub fn OP_OVER(stack: &mut Vec<Type>) {
    let x = pop!(stack);
    let y = pop!(stack);
    stack.push(y.clone());
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
pub fn OP_INDEX(stack: &mut Vec<Type>) {
    if let Num::Integer(n) = pop_num!(stack) {
        if let Type::List(y) = pop!(stack) {
            stack.push(y[n as usize].clone())
        } else {
            panic!("Expected List")
        }
    } else {
        panic!("Expected integer to index list")
    }
}
