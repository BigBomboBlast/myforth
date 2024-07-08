#![allow(warnings)]
use std::io::{self, Write};
use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

mod operations;
pub use operations::*;
mod types;
pub use types::*;

#[derive(Debug)]
enum Op<'a> {
    PushInteger(i64),
    PushFloat(f64),
    PushStr(&'a str),
    PushBool(bool),
    PushList(Vec<Type>),
    Add,
    Sub,
    Mul,
    Div,
    Eq, // pop stack twice - push 1 or 0
    Gt, // greater than
    Lt, // less than
    Gteq, // >= 
    Lteq, // <=
    Dup, // Duplicate value on the top of the stack
    Swap, // swap 2 values on top of stack
    Drop, // remove value on top of stack
    Over, // copy element on bottom of the stack to the top of the stack 
    Rotate, // rotate 3 values on top of the stack, a b c - b c a
    Out, // pop stack - print to console
    Index,
    Defvar(&'a str),
    Readvar(&'a str),
    Writevar(&'a str),
    If(usize), // pop stack - if 0 jump to end, otherwise proceed
    Ifstar(usize), // used in else-if blocks
    Else(usize), // unconditional jump instruction
    End(usize), // unconditional jump instruction
    While, // just a label
    Do(usize), // pop stack - if 0 jump to end, otherwise proceed, same as `if` but has different rules
    Defword(usize), // unconditional jump
    Return, // jump based on return stack
    Call(usize), // jump to function definiton
    EndOfProgram,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Num,
    Bool,
    Str,
    Word,
    VarOp, // an operation that acts on a variable
    List,
    Struct,
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
        let peek = source.chars().nth(idx+1).expect("Unclosed String");

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

fn find_end_list(source: &String, mut idx: usize) -> usize {
    let mut stack: Vec<bool> = vec![true];
    loop {
        let char = source.chars().nth(idx).expect("Unclosed List");
        let peek = source.chars().nth(idx+1).expect("Unclosed List");

        if char == '[' {
            stack.push(true);
        } else if char == ']' {
            stack.pop();
        }

        if stack.is_empty() {
            if !peek.is_whitespace() {
                panic!("Found unexpected character `{}` after list", peek);
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
    let is_bool = |w: &str| w.parse::<bool>().is_ok() == true;

    let mut result: Vec<(&str, usize, usize, Token)> = vec![];
    let mut word_start = None;
    let mut line_no: usize = 1;
    let mut column_no: usize = 1;
   
    let mut i = 0;

    while i < source.len() {
        let char = source.chars().nth(i).unwrap();

        match (char.is_whitespace(), word_start, char == '"' || char == '[') {
            (false, None, false) => {
                word_start = Some((i, column_no));
            }
            (false, None, true) => {
                let mut end = 0;
                if char == '"' {
                    end = find_end_str(source, i+1);
                    result.push((&source[(i+1)..end], line_no, column_no, Token::Str));
                } else if char == '[' {
                    end = find_end_list(source, i+1);
                    result.push((&source[i..end+1], line_no, column_no, Token::List));
                }
                i = end;
                column_no = i+1;
                // column numbering starts at 1, `i` and `end` are full string indexes
                word_start = None;
            }
            (true, Some((start, col)), false) => {
                let token_literal = &source[start..i];
                if is_num(token_literal) {
                    result.push((token_literal, line_no, col, Token::Num));
                } else if is_bool(token_literal) {
                    result.push((token_literal, line_no, col, Token::Bool));
                } else if token_literal.chars().nth(0).unwrap() == '@' ||
                          token_literal.chars().nth(0).unwrap() == '!' {
                    result.push((token_literal, line_no, col, Token::VarOp));
                } else {
                    result.push((token_literal, line_no, col, Token::Word));
                }

                word_start = None;
                if char == '\n' {
                    column_no = 0;                     
                    line_no += 1;
                }
            }
            (_, Some((_, _)), true) => panic!("found unexpected {} in word", char),
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

fn split_str_to_list(list_as_str: &String) -> Vec<&str> {
    // "[1 2 [3 4]]" becomes "[ 1 2 [ 3 4 ] ]"
    let mut start = None;
    let mut result: Vec<&str> = vec![];
    let mut i = 0; 
    while i < list_as_str.len() {
        let char = list_as_str.chars().nth(i).unwrap();
        match (char.is_whitespace(), start, char == '"') {
            (false, None, false) => {
                start = Some(i);
            }
            (false, None, true) => {
                let end = find_end_str(list_as_str, i+1);
                result.push(&list_as_str[i..end+1]);
                i = end;
                start = None;
            }
            (true, Some(n), false) => {
                result.push(&list_as_str[n..i]);
                start = None;
            }
            (_, Some(_), true) => panic!("found unexpected {} in list", char),
            _ => (),
        }
        i+=1;
    }

    return result;
}

fn parse_to_num(n: &str) -> Num { 
    match n {
        n if n.parse::<i64>().is_ok() => Num::Integer(n.parse::<i64>().unwrap()),
        n if n.parse::<f64>().is_ok() => Num::Float(n.parse::<f64>().unwrap()),
        _ => panic!("uh oh"),
    }
}

fn is_str(s: &str) -> bool {
    if s.chars().nth(0).expect("1") == '"' && 
       s.chars().nth(s.len()-1).expect("2") == '"' {
        return true;
    } else {
        return false;
    }
}

fn get_nested_list(split: &mut Vec<&str>) -> Vec<Type> {
    let mut nested_list: Vec<Type> = vec![];

    while !split.is_empty() {
        let current = split.remove(0);
        match current {
            "[" => nested_list.push(Type::List(get_nested_list(split))),
            "]" => break,
            n if n.parse::<f64>().is_ok() => nested_list.push(Type::Number(parse_to_num(n))),
            s if is_str(s) =>{
                let quotes_removed = &s[1..s.len()-1];
                nested_list.push(Type::Str(String::from(quotes_removed)));
            }
            b if b.parse::<bool>().is_ok() => nested_list.push(Type::Boolean(b.parse::<bool>().unwrap())),
            _ => panic!("expected valid type in list, got {}", current),
        }
    }
    return nested_list;
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
                "idx" => program.push(Op::Index),
                "defvar" => {
                    let (var_name, _, _, _) = source.remove(0);
                    program.push(Op::Defvar(var_name));
                }
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
            let is_int = |n: &str| n.parse::<i64>().is_ok();
            let to_int = |n: &str| n.parse::<i64>().unwrap();
            let is_float = |n: &str| n.parse::<f64>().is_ok();
            let to_float = |n: &str| n.parse::<f64>().unwrap();
            match literal {
                n if is_int(n) => program.push(Op::PushInteger(to_int(n))),
                n if is_float(n) => program.push(Op::PushFloat(to_float(n))),
                _ => panic!("something went wrong in parse_to_program() when matching the numbers"),
            }
        } else if token == Token::Bool {

            let to_bool = |b: &str| b.parse::<bool>().unwrap();
            program.push(Op::PushBool(to_bool(literal)))

        } else if token == Token::Str {

            program.push(Op::PushStr(literal));

        } else if token == Token::List {
   
            let mut str_ref = String::from(literal.replace("[", "[ ",).replace("]", " ]"));
            str_ref.push(' ');
            let mut thing = split_str_to_list(&str_ref);
            program.push(Op::PushList(get_nested_list(&mut thing)));

        } else if token == Token::VarOp {

            let var_name = &literal[1..literal.len()];

            if literal.chars().nth(0).unwrap() == '@' {
                program.push(Op::Writevar(var_name))
            } else if literal.chars().nth(0).unwrap() == '!' {
                program.push(Op::Readvar(var_name))
            }
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
    let mut mem: HashMap<&str, Type> = HashMap::new(); // this is where the variables are stored
    let mut ip = 0;
    while ip < program.len() {
        match program[ip] {
            Op::PushInteger(n) => {
                s.push(Type::Number(Num::Integer(n)));
                ip+=1;
            }
            Op::PushFloat(f) => {
                s.push(Type::Number(Num::Float(f)));
                ip+=1;
            }
            Op::PushBool(b) => {
                s.push(Type::Boolean(b));
                ip+=1;
            }
            Op::PushStr(string) => {
                s.push(Type::Str(String::from(string)));
                ip+=1;
            }
            Op::PushList(ref list) => {
                s.push(list[0].clone());
                ip+=1;
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
            Op::Index => {
                OP_INDEX(s);
                ip+=1;
            }
            Op::Defvar(var_name) => {
                mem.insert(var_name, Type::Null);
                ip+=1;
            }
            Op::Writevar(var_name) => {
                match mem.entry(var_name) {
                    Entry::Occupied(mut var) => var.insert(s.pop().expect("stack underflow")),
                    _ => panic!("Variable {} has not beed initialized", var_name)
                };
                ip+=1;
            }
            Op::Readvar(var_name) => {
                if let Some(val) = mem.get(var_name) {
                    s.push(val.clone());
                } else {
                    panic!("Variable {} has not been initialized", var_name);
                }
                ip+=1;
            }
            Op::If(label) => {
                let x = s.pop().expect("stack underflowww");
                if is_falsy(x) {
                    ip = label;
                } else {
                    ip+=1;
                }
            }
            Op::Ifstar(label) => {
                let x = s.pop().expect("stack underflow");
                if is_falsy(x) {
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
                if is_falsy(x) {
                    ip = label;
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
        //let mut memory: [u64; 100] = [0; 100];

        println!("\nWelcome to Bombo's Forth Interactive Environment Repl");
        loop {
            print!("\n>> "); // prompt
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input)
                       .expect("failed"); // read input to buffer
            
            let mut tokens = tokenize(&input);
            println!("{:?}", tokens);
            let program = parse_to_program(&mut tokens);
            println!("{:?}", program);
            run(&program, &mut stack);
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
