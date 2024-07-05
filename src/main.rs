#![allow(warnings)]
use std::io::{self, Write};
use std::env;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

mod operations;
pub use operations::*;
mod types;
pub use types::*;

#[derive(Debug)]
enum Op<'a> {
    PushPos(usize),
    PushNeg(i64),
    PushFloat(f64),
    PushStr(&'a str),
    Add,
    Sub,
    Mul,
    Div,
    To_signed,
    To_unsigned,
    To_float,
    Eq, // pop stack twice - push 1 or 0
    Gt, // greater than
    Lt, // less than
    Gteq, // >= 
    Lteq, // <=
//    Or, // pop twice, if at least 1 is not 0, push 1 to stack
//    And, // pop twice, if both are not 0, push 1 to stack
//    Not, // pop stack, if >0 push 0, else push 1
    Dup, // Duplicate value on the top of the stack
    Swap, // swap 2 values on top of stack
    Drop, // remove value on top of stack
    Over, // copy element on bottom of the stack to the top of the stack 
    Rotate, // rotate 3 values on top of the stack, a b c - b c a
    Out, // pop stack - print to console
    If(usize), // pop stack - if 0 jump to end, otherwise proceed
    Ifstar(usize), // used in else-if blocks
    Else(usize), // unconditional jump instruction
    End(usize), // unconditional jump instruction
    While, // just a labe
    Do(usize), // pop stack - if 0 jump to end, otherwise proceed, same as `if` but has different rules
    Defword(usize), // unconditional jump
    Return, // jump based on return stack
    Call(usize), // jump to function definiton
    EndOfProgram,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Num,
    Str,
    Word,
}

fn remove_comments(source: &String) -> String {
    let mut result = String::new();

    for line in source.lines() {
        // remove comment portion 
        if let Some((code, _)) = line.split_once("//") {
            result.push_str(code);
            result.push('\n');
        } else { // this runs when there is no comment
            result.push_str(line);
            result.push('\n');
        }
    }

    return result;
}

fn find_end_str(source: &String, mut idx: usize) -> usize {
    loop {
        let char = source.chars().nth(idx).expect("Unclosed String");
        let peek = source.chars().nth(idx+1).expect("Problem with find_end_str()");

        if char == '"' {
            if !peek.is_whitespace() {
                panic!("Found unexpected character `{}` after string literal", peek);
            }
            break;
        }
        idx+=1;
    }
    return idx;
}


fn tokenize(source: &String) -> Vec<(&str, usize, usize, Token)> {
    // helper function to identify token types
    let is_num = |w: &str| {
        match w {
           w if w.parse::<i64>().is_ok() => true,
           w if w.parse::<f64>().is_ok() => true,
           _ => false,
        }
    };

    let mut result: Vec<(&str, usize, usize, Token)> = vec![];
    let mut word_start = None;
    let mut line_no: usize = 1;
    let mut column_no: usize = 1;
   
    let mut i = 0;

    while i < source.len() {
        let char = source.chars().nth(i).unwrap();

        match (char.is_whitespace(), word_start, char == '"') {
            (false, None, false) => {
                word_start = Some((i, column_no));
            }
            (false, None, true) => {
                let end = find_end_str(source, i+1);
                result.push((&source[(i+1)..end], line_no, column_no, Token::Str));
                i = end;
                column_no = i+1;
                // column numbering starts at 1, `i` and `end` are full string indexes
                word_start = None;
            }
            (true, Some((start, col)), false) => {
                let token_literal = &source[start..i];
                if is_num(token_literal) {
                    result.push((token_literal, line_no, col, Token::Num));
                } else {
                    result.push((token_literal, line_no, col, Token::Word));
                }

                word_start = None;
                if char == '\n' {
                    column_no = 0;                     
                    line_no += 1;
                }
            }
            (_, Some((_, _)), true) => panic!("found unexpected `\"` in word"),
            (true, _, _) if char == '\n' => {
                column_no = 0;
                line_no += 1;
            }
            _ => (),
        }
        i+=1;
        column_no += 1;
    }
    return result;
}

fn parse_to_program<'a>(source: &'a mut Vec<(&'a str, usize, usize, Token)>) -> Vec<Op<'a>> {
    let error_reference = source.clone(); // I need this for error referencing, since I consume tokens using .remove(0)
    let mut jump_locations: Vec<usize> = vec![];
    let mut program: Vec<Op> = vec![];
    let mut i = 0;
    let mut dict: HashMap<&str, usize> = HashMap::new();
    while !source.is_empty() {
        let (literal, line, col, token) = source.remove(0);
        if token == Token::Word {
            match literal {
                "+" => program.push(Op::Add),
                "-" => program.push(Op::Sub),
                "*" => program.push(Op::Mul),
                "/" => program.push(Op::Div),
                "to_signed" => program.push(Op::To_signed),
                "to_unsigned" => program.push(Op::To_unsigned),
                "to_float" => program.push(Op::To_float),
                "=" => program.push(Op::Eq),
                ">" => program.push(Op::Gt),
                "<" => program.push(Op::Lt),
                ">=" => program.push(Op::Gteq),
                "<=" => program.push(Op::Lteq),
                "out" => program.push(Op::Out),
                "dup" => program.push(Op::Dup),
                "swap" => program.push(Op::Swap),
                "drop" => program.push(Op::Drop),
                "over" => program.push(Op::Over),
                "rotate" => program.push(Op::Rotate),
                "if" => {
                    program.push(Op::If(0));
                    jump_locations.push(i);
                }
                "if*" => {
                    program.push(Op::Ifstar(0));
                    jump_locations.push(i);
                }
                "else" => {
                    program.push(Op::Else(0));
                    let errmsg = format!("{}:{} dangling `else`", line, col);
                    match &mut program[jump_locations.pop().expect(&errmsg)] {
                        Op::If(ref mut n) => *n = i+1,
                        Op::Ifstar(ref mut n) => {
                            *n = i+1;
                            let errmsg = format!("{}:{} `if*` expected to preceed `else`", line, col);
                            match &mut program[jump_locations.pop().expect(&errmsg)] {
                                Op::Else(ref mut n) => *n = i,
                                _ => panic!("{}", errmsg)
                            }
                        }
                        _ => {
                            panic!("{}", errmsg);
                        }
                    }
                    jump_locations.push(i);
                }
                "end" => {
                    let errmsg = format!("{}:{} dangling `end`", line, col);
                    let mut label = 0;
                    let mut location = jump_locations.pop().expect(&errmsg);
                    match &mut program[location] {
                        Op::If(ref mut n) => {
                            *n = i;
                            label = i+1;
                        }
                        Op::Ifstar(ref mut n) => {
                            *n = i;
                            label = i+1;
                            let errmsg = format!("{}:{} `if*` expected to preceed `else`", line, col);
                            match &mut program[jump_locations.pop().expect(&errmsg)] {
                                Op::Else(ref mut n) => *n = i,
                                _ => panic!("{}", errmsg)
                            }
                        }
                        Op::Else(ref mut n) => {
                            *n = i;
                            label = i+1;
                        }
                        Op::Do(ref mut n) => {
                            *n = i+1;
                            let errmsg = format!("{}:{} `end` expected `while` before `do`", line, col);
                            let while_loc = jump_locations.pop().expect(&errmsg);
                            match program[while_loc] { // this match is for the sole purpose of destructuring
                                Op::While => label = while_loc,
                                _ => {
                                    panic!("{}", errmsg);
                                }
                            }
                        }
                        Op::While => {
                            let errmsg = format!("{}:{} `end` expected `do` after `while`", line, col);
                            panic!("{}", errmsg);
                        },
                        Op::Defword(_) => {
                            let errmsg = format!("{}:{} use `return` to end word declarations", line, col);
                            panic!("{}", errmsg);
                        }
                        _ => (),
                    }
                    // jumps to itself in case of if statements
                    program.push(Op::End(label)); 
                }
                "while" => {
                    program.push(Op::While);
                    jump_locations.push(i);
                }
                "do" => {
                    program.push(Op::Do(0));
                    jump_locations.push(i);
                }
                "defword" => {
                    program.push(Op::Defword(0));
                    jump_locations.push(i);

                    let (word_name, _, _, _) = source.remove(0);
                    dict.insert(word_name, i+1);
                }
                "return" => {
                    program.push(Op::Return);
                    let errmsg = format!("{}:{} dangling `end`", line, col);
                    let mut location = jump_locations.pop().expect(&errmsg);
                    match program[location] {
                        Op::Defword(ref mut n) => *n = i+1,
                        _ => panic!("{}:{} `return` expected to be used in word declaration", line, col),
                    }
                }
                word => {
                    if let Some(loc) = dict.get(word) {
                        program.push(Op::Call(*loc))
                    } else {
                        panic!("{}:{} Unknown Word `{}` Encountered", line, col, literal)
                    }
                }
            }
        } else if token == Token::Num {
            // helper functions to parse integers that are strings
            let is_positive = |n: &str| n.parse::<usize>().is_ok();
            let to_positive = |n: &str| n.parse::<usize>().unwrap();
            let is_negative = |n: &str| n.parse::<i64>().is_ok();
            let to_negative = |n: &str| n.parse::<i64>().unwrap();
            let is_float = |n: &str| n.parse::<f64>().is_ok();
            let to_float = |n: &str| n.parse::<f64>().unwrap();
            match literal {
                n if is_positive(n) => program.push(Op::PushPos(to_positive(n))),
                n if is_negative(n) => program.push(Op::PushNeg(to_negative(n))),
                n if is_float(n) => program.push(Op::PushFloat(to_float(n))),
                _ => panic!("something went wrong in parse_to_program() when matching the numbers"),
            }
        } else if token == Token::Str {
            program.push(Op::PushStr(literal));
        }
        i+=1;
    }

    if !jump_locations.is_empty() {
        let (word, line, col, _) = error_reference[jump_locations.pop().unwrap()];
        match word {
            "if" => panic!("{}:{} Unclosed `if`", line, col),
            "else" => panic!("{}:{} unclosed `else`", line, col),
            "do" => panic!("{}:{} `do` block after loop condition is unclosed", line, col),
            "while" => panic!("{}:{} `while` loop unclosed", line, col),
            _ => (),
        }
    }
    program.push(Op::EndOfProgram);
    return program;
}

fn run(program: &Vec<Op>, s: &mut Vec<Type>) {
    // `ip` stands for `instruction pointer`
    let mut return_stack: Vec<usize> = vec![];
    let mut ip = 0;
    while ip < program.len() {
        match program[ip] {
            Op::PushNeg(n) => {
                s.push(Type::Signed(n));
                ip+=1;
            }
            Op::PushFloat(f) => {
                s.push(Type::Float(f));
                ip+=1;
            }
            Op::PushPos(u) => {
                s.push(Type::Unsigned(u));
                ip+=1;
            }
            Op::PushStr(string) => {
                panic!("Not Implemented")
            }
            Op::Add => {
                OP_ADD(s);
                ip+=1;
            }
            Op::Sub => {
                OP_SUB(s);
                ip+=1;
            }
            Op::Mul => {
                OP_MUL(s);
                ip+=1;
            }
            Op::Div => {
                OP_DIV(s);
                ip+=1;
            }
            Op::To_unsigned => {
                OP_TOSIGNED(s);
                ip+=1;
            }
            Op::To_signed => {
                OP_TOUNSIGNED(s);
                ip+=1;
            }
            Op::To_float => {
                OP_TOFLOAT(s);
                ip+=1;
            }
            Op::Eq => {
                OP_EQ(s);
                ip+=1;
            }
            Op::Gt => {
                OP_GT(s);
                ip+=1;
            }
            Op::Lt => {
                OP_LT(s);
                ip+=1;
            }
            Op::Gteq => {
                OP_GTEQ(s);
                ip+=1;
            }
            Op::Lteq => {
                OP_LTEQ(s);
                ip+=1;
            }
            Op::Out => {
                OP_OUT(s);
                ip+=1;
            }
            Op::Dup => {
                OP_DUP(s);
                ip+=1;
            }
            Op::Swap => {
                OP_SWAP(s);
                ip+=1;
            }
            Op::Drop => {
                OP_DROP(s);
                ip+=1;
            }
            Op::Over => {
                OP_OVER(s);
                ip+=1;
            }
            Op::Rotate => {
                OP_ROTATE(s);
                ip+=1;
            }
            Op::If(label) => {
                let x = s.pop().expect("stack underflow");
                if x == Type::Unsigned(0) { // condition is false
                    // jump to label
                    ip = label;
                } else {
                    ip+=1;
                }
            }
            Op::Ifstar(label) => {
                let x = s.pop().expect("stack underflow");
                if x == Type::Unsigned(0) { // condition is false
                    // jump to label
                    ip = label;
                } else {
                    ip+=1;
                }
            }
            Op::Else(label) => ip = label,
            Op::End(label) => ip = label,
            Op::While => ip+=1, // doesnt do anything, just a label to jump to
            Op::Do(label) => {
                let x = s.pop().expect("stack underflow");
                if x == Type::Unsigned(0) { // loop condition is false
                    ip = label // jump to end
                } else {
                    ip+=1;
                }
            }
            Op::Defword(label) => ip = label,
            Op::Return => ip = return_stack.pop().expect("major problem, developer, in return instruction in parse_to_program"),
            Op::Call(label) => {
                return_stack.push(ip+1);
                ip = label;
            }
            Op::EndOfProgram => ip = program.len(),
        }
    }
}

fn main() {
    // get args
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let mut input = String::new();
        let mut stack: Vec<Type> = vec![];

        println!("\nWelcome to Bombo's Forth Interactive Environment Repl");
        println!("Note that accessing memory can be quite janky in the REPL");
        println!("Just have fun with it, the REPL is only for getting a feel for things");
        loop {
            print!("\n>> "); // prompt
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input)
                       .expect("failed"); // read input to buffer
            
            let mut tokens = tokenize(&input);
            let program = parse_to_program(&mut tokens);
            run(&program, &mut stack);
            println!("{:?}", program);
            show_stack_debug(&stack);
            input.clear();
        }
    } else if args.len() == 2 {
        let mut source = String::new();
        let mut file = File::open(args[1].clone()).expect("Cannot find the file");
        file.read_to_string(&mut source).expect("Failed to read file.");

        let mut stack: Vec<Type> = vec![];

        let src = remove_comments(&source);
        let mut tokens = tokenize(&src);
        let program: Vec<Op> = parse_to_program(&mut tokens);
        println!("{:?}", program);
        run(&program, &mut stack);
        show_stack(&stack);
    } else {
        println!("calm down there buddy, to many arguments");
    }
}
