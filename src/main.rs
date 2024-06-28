use std::io::{self, Write};
use std::env;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

#[derive(Debug)]
enum Op {
    Push(i32),
    Add,
    Sub,
    Eq, // pop stack twice - push 1 or 0
    Gt, // greater than
    Out, // pop stack - print to console
    Dup, // Duplicate value on the top of the stack
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
} // overall I feel like this function heavily abuses rust pattern matching.

fn tokenize(source: &Vec<(&str, usize, usize)>) -> Vec<Op> {
    let is_int = |w: &str| w.parse::<i32>().is_ok();
    let to_int = |w: &str| w.parse::<i32>().unwrap();

    let mut jump_locations: Vec<usize> = vec![];
    let mut tokens: Vec<Op> = vec![];
    for i in 0..source.len() {
        let (word, line, col) = source[i];
        match word {
            "+" => tokens.push(Op::Add),
            "-" => tokens.push(Op::Sub),
            "=" => tokens.push(Op::Eq),
            ">" => tokens.push(Op::Gt),
            "out" => tokens.push(Op::Out),
            "dup" => tokens.push(Op::Dup),
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
            w if is_int(w) => tokens.push(Op::Push(to_int(w))),
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

fn run(program: &Vec<Op>, stack: &mut Vec<i32>) {
    // `ip` stands for `instruction pointer`
    let mut ip = 0;
    while ip < program.len() {
        match program[ip] {
            Op::Push(n) => stack.push(n),
            Op::Add => {
                let x = stack.pop().expect("stack underflow"); 
                let y = stack.pop().expect("stack underflow"); 
                stack.push(x + y); // this will cause rust panic for stack underflow
            }
            Op::Sub => {
                let x = stack.pop().expect("stack underflow"); 
                let y = stack.pop().expect("stack underflow"); 
                stack.push(y - x); // may cause rust panic
            }
            Op::Eq => {
                let x = stack.pop().expect("stack underflow"); 
                let y = stack.pop().expect("stack underflow"); 
                stack.push((x == y) as i32); // may cause rust panic
            }
            Op::Gt => {
                let x = stack.pop().expect("stack underflow"); 
                let y = stack.pop().expect("stack underflow"); 
                stack.push((y > x) as i32); // may cause rust panic
            }
            Op::Out => {
                let x = stack.pop().expect("stack underflow"); 
                println!("{}", x); // may cause rust panic
            }
            Op:: Dup => {
                let x = stack.pop().expect("stack underflow");
                stack.push(x);
                stack.push(x);
            }
            Op::If(label) => {
                let x = stack.pop().expect("stack underflow");
                if x == 0 { // condition is false
                    // jump to label
                    ip = label;
                }
            }
            Op::Else(label) => ip = label,
            Op::End(label) => ip = label,
            Op::While => (), // doesnt do anything, just a label to jump to
            Op::Do(label) => {
                let x = stack.pop().expect("stack underflow");
                if x == 0 { // loop condition is false
                    ip = label // jump to end
                }
            }
        }
        ip+=1;
    }
}

fn main() {
    // get args
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("\nmust supply argument\nload to load program\nrepl to run repl\n"); 
    }

    // will be able to read from files soon
    if args[1] == "load" {
        let mut stack: Vec<i32> = vec![];
        let hardcoded = String::from("\n\n\n 8   8 8 = if\n  out  end  \n");
        println!("What you typed {:?}", parse_to_words(&hardcoded));
        //let tokens: Vec<Op> = tokenize(&parse_to_words(&hardcoded));
        //run(&tokens, &mut stack);
    } else if args[1] == "repl" {
        let mut input = String::new(); // to store input
        let mut stack: Vec<i32> = vec![];

        loop {
            print!("\n>> "); // prompt
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input)
                       .expect("failed"); // read input to buffer
            
            let tokens: Vec<Op> = tokenize(&parse_to_words(&input));
            println!("tokens - {:?}", tokens);
            run(&tokens, &mut stack);
            println!("STACK TRACE - {:?}", stack);
            input.clear();
        }
    } else {
        println!("Unimplemented?");
    }
}
