use std::io::{self, Write};
use std::env;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::fs::File;
use std::io::Read;

mod operations;
pub use operations::*;

#[derive(Debug)]
enum Op {
    PushU(usize),
    PushI(i32),
    PushF(f32),
    Add,
    Sub,
    Eq, // pop stack twice - push 1 or 0
    Gt, // greater than
    Lt, // less than
    Gteq, // >=
    Lteq, // <=
    Dup, // Duplicate value on the top of the stack
    Out, // pop stack - print to console
    Mem, // push address of the beggining of memory
    Read, // pop stack, expecting a memory address, push value contained at that memory address
    Write, // pop stack twice, expecting a memory address and value, store value at memory address
    If(usize), // pop stack - if 0 jump to end, otherwise proceed
    Else(usize), // unconditional jump instruction
    End(usize), // unconditional jump instruction
    While, // just a label
    Do(usize), // pop stack - if 0 jump to end, otherwise proceed, same as `if` but has different rules
}

// This is one of the functions
fn parse_to_words(source: &String) -> Vec<(&str, usize, usize)> {
    // Need to split string by spaces so `8 8 +` becomes `["8", "8", "+"]
    // BUT I need to keep track of the line and column each word is located
    // Which is why this returns an array of tuples, (word, line_no, column_no)
    let mut result: Vec<(&str, usize, usize)> = vec![];
    let mut word_start = None;
    let mut line_no: usize = 1; // keep track of current line and column
    let mut column_no: usize = 1; 

    for (i, char) in source.char_indices() {
        // I cannot use `i` because it's an INDEX OF ENTIRE STRING not the column number
        // the column starts at at 1 and resets to 1 with every new line
        match (char.is_whitespace(), word_start) {
            // word_start keeps track of the beggining index of each word, including column_no
            (false, None) => word_start = Some((i, column_no)), 
            (true, Some((start, col))) => {
                result.push((&source[start..i], line_no, col));
                word_start = None;
                // in case next is a newline, I have to update
                if char == '\n' {
                    column_no = 0; // must be 0 since this gets incremented after match anyways
                    line_no += 1;
                }
            }
            (true, _) if char == '\n' => {
                column_no = 0;
                line_no += 1;
            }
            _ => (),
        }
        column_no += 1;
    }
   
    // string doesn't neccesarily have to end with whitespace, so I check if there was a word
    // and add it to the result accordingly one more time
    if let Some((start, col)) = word_start {
        result.push((&source[start..], line_no, col))
    }

    return result;
}

fn tokenize(source: &Vec<(&str, usize, usize)>) -> Vec<Op> {
    let is_int = |w: &str| w.parse::<i32>().is_ok();
    let to_int = |w: &str| w.parse::<i32>().unwrap();

    let is_float = |w: &str| w.parse::<f32>().is_ok();
    let to_float = |w: &str| w.parse::<f32>().unwrap();

    let is_usize = |w: &str| w.parse::<usize>().is_ok();
    let to_usize = |w: &str| w.parse::<usize>().unwrap();

    let mut jump_locations: Vec<usize> = vec![];
    let mut tokens: Vec<Op> = vec![];
    for i in 0..source.len() {
        let (word, line, col) = source[i];
        match word {
            "+" => tokens.push(Op::Add),
            "-" => tokens.push(Op::Sub),
            "=" => tokens.push(Op::Eq),
            ">" => tokens.push(Op::Gt),
            "<" => tokens.push(Op::Lt),
            ">=" => tokens.push(Op::Gteq),
            "<=" => tokens.push(Op::Lteq),
            "out" => tokens.push(Op::Out),
            "dup" => tokens.push(Op::Dup),
            "mem" => tokens.push(Op::Mem),
            "read" => tokens.push(Op::Read),
            "write" => tokens.push(Op::Write),
            "if" => {
                tokens.push(Op::If(0));
                jump_locations.push(i);
            }
            "else" => {
                tokens.push(Op::Else(0));
                let errmsg = format!("{}:{} dangling `else`", line, col);
                match tokens[jump_locations.pop().expect(&errmsg)] {
                    // although `if` is supposed to jump to the instruction AFTER `else`
                    // the instruction pointer gets incremented after the jump anyways
                    // effectively skipping the `else`
                    // I could probably think it out more, but it sounds like a pain and this works
                    Op::If(ref mut n) => *n = i,
                    _ => {
                        let errmsg = format!("{}:{} Expected to close If statement", line, col);
                        panic!("{}", errmsg);
                    }
                }
                jump_locations.push(i);
            }
            "end" => {
                let errmsg = format!("{}:{} dangling `end`", line, col);
                let mut label = 0;
                let mut location = jump_locations.pop().expect(&errmsg);
                match tokens[location] {
                    Op::If(ref mut n) => {
                        *n = i;
                        label = i;
                    }
                    Op::Else(ref mut n) => {
                        *n = i;
                        label = i;
                    }
                    Op::Do(ref mut n) => {
                        *n = i;
                        println!("{:?}", tokens);
                        let errmsg = format!("{}:{} `end` expected `while` before `do`", line, col);
                        let while_loc = jump_locations.pop().expect(&errmsg);
                        match tokens[while_loc] { // this match is for the sole purpose of destructuring
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
                    _ => (),
                }
                tokens.push(Op::End(label)); // jumps to itself in the case of if statements
            }
            "while" => {
                tokens.push(Op::While);
                jump_locations.push(i);
            }
            "do" => {
                tokens.push(Op::Do(0));
                jump_locations.push(i);
            }
            w if is_usize(w) => tokens.push(Op::PushU(to_usize(w))),
            w if is_int(w) => tokens.push(Op::PushI(to_int(w))),
            w if is_float(w) => tokens.push(Op::PushF(to_float(w))),
            _ => panic!("{}:{} Unknown Word `{}` Encountered", line, col, word),
        }
    }

    if !jump_locations.is_empty() {
        let (word, line, col) = source[jump_locations.pop().unwrap()];
        match word {
            "if" => panic!("{}:{} Unclosed `if`", line, col),
            "else" => panic!("{}:{} unclosed `else`", line, col),
            "do" => panic!("{}:{} `do` block after loop condition is unclosed", line, col),
            "while" => panic!("{}:{} `while` loop unclosed", line, col),
            _ => (),
        }
    }
    return tokens;
}

fn run(program: &Vec<Op>, s: &mut Vec<V>, mem: *mut u8) -> Vec<V> {
    // `ip` stands for `instruction pointer`
    let mut ip = 0;
    while ip < program.len() {
        match program[ip] {
            Op::PushI(n) => s.push(V::I(n)),
            Op::PushF(f) => s.push(V::F(f)),
            Op::PushU(u) => s.push(V::U(u)),
            Op::Add => {
                OP_ADD(s);
            }
            Op::Sub => {
                OP_SUB(s);
            }
            Op::Eq => {
                OP_EQ(s);
            }
            Op::Gt => {
                OP_GT(s);
            }
            Op::Lt => {
                OP_LT(s);
            }
            Op::Gteq => {
                OP_GTEQ(s);
            }
            Op::Lteq => {
                OP_LTEQ(s);
            }
            Op::Out => {
                OP_OUT(s);
            }
            Op:: Dup => {
                OP_DUP(s);
            }
            Op::Mem => {
                s.push(V::U(mem as usize));
            }
            Op::Read => unsafe {
                let V::U(addr) = s.pop().expect("stack underflow") else {
                    panic!("`Read` expected a possible memory location, got a float/negative value")
                };
                s.push(V::U(*(addr as *mut u8) as usize));
            } 
            Op::Write => unsafe {
                let V::U(addr) = s.pop().expect("stack underflow") else {
                    panic!("`write` expected a possible memory location, got a float/negative value")
                };
                let V::U(val) = s.pop().expect("stack underflow") else {
                    panic!("cannot write negative/float to memory - yet")
                };
                *(addr as *mut u8) = val as u8;
            }
            Op::If(label) => {
                let x = s.pop().expect("stack underflow");
                if x == V::U(0) { // condition is false
                    // jump to label
                    ip = label;
                }
            }
            Op::Else(label) => ip = label,
            Op::End(label) => ip = label,
            Op::While => (), // doesnt do anything, just a label to jump to
            Op::Do(label) => {
                let x = s.pop().expect("stack underflow");
                if x == V::U(0) { // loop condition is false
                    ip = label // jump to end
                }
            }
        }
        ip+=1;
    }
    return s.to_vec();
}

fn main() {
    // get args
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let mut input = String::new();
        let mut stack: Vec<V> = vec![];

        let layout = Layout::from_size_align(1000, 1).unwrap(); // should be enough for me
        let mem = unsafe { alloc(layout) }; // pointer to beggining of memory

        loop {
            print!("\n>> "); // prompt
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input)
                       .expect("failed"); // read input to buffer
            
            let tokens: Vec<Op> = tokenize(&parse_to_words(&input));
            run(&tokens, &mut stack, mem);
            show_stack_debug(&stack);
            input.clear();
        }
    } else if args.len() == 2 {
        let mut source = String::new();
        let mut file = File::open(args[1].clone()).expect("Cannot find the file");
        file.read_to_string(&mut source).expect("Failed to read file.");

        let mut stack: Vec<V> = vec![];
        let layout = Layout::from_size_align(1000, 1).unwrap(); // is enough for me
        let mem = unsafe { alloc(layout) }; // pointer to beggining of memory
        
        let tokens: Vec<Op> = tokenize(&parse_to_words(&source));
        run(&tokens, &mut stack, mem);
        show_stack(&stack);
    } else {
        println!("calm down there buddy, to many arguments");
    }
}
