use std::env;
use std::io;
use std::io::*;
use std::process;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";

const MEMORY_SIZE: usize = 30_000;

#[derive(Debug)]
enum Language {
    Brainfuck,
    Ook,
    Spoon,
}

#[derive(Debug)]
enum Token {
    Right,
    Left,
    Incr,
    Decr,
    Out,
    In,
    Begin,
    End,
    Exit,
    Print,
}

fn brainfuck_parser(plain: &str) -> Vec<Token> {
    let plain: Vec<char> = plain.chars().collect();
    let mut tokenized = Vec::new();

    for c in plain {
        tokenized.push(match c {
            '>' => Token::Right,
            '<' => Token::Left,
            '+' => Token::Incr,
            '-' => Token::Decr,
            '.' => Token::Out,
            ',' => Token::In,
            '[' => Token::Begin,
            ']' => Token::End,
            _ => continue,
        });
    }

    tokenized
}

fn ook_parser(plain: &str) -> Vec<Token> {
    let plain: Vec<char> = plain.chars().collect();
    let mut tokenized = Vec::new();
    let mut first_word: Option<char> = None;
    let mut word_head: i32 = 0;

    for c in plain {
        let result = match c {
            'O' => {
                if word_head == 0 {
                    Ok(())
                } else {
                    Err("O")
                }
            }
            'o' => {
                if word_head == 1 {
                    Ok(())
                } else {
                    Err("o")
                }
            }
            'k' => {
                if word_head == 2 {
                    Ok(())
                } else {
                    Err("k")
                }
            }
            '.' | '?' | '!' => {
                if word_head == 3 {
                    match first_word {
                        Some(c1) => {
                            first_word = None;
                            tokenized.push(match (c1, c) {
                                ('.', '?') => Token::Right,
                                ('?', '.') => Token::Left,
                                ('.', '.') => Token::Incr,
                                ('!', '!') => Token::Decr,
                                ('!', '.') => Token::Out,
                                ('.', '!') => Token::In,
                                ('!', '?') => Token::Begin,
                                ('?', '!') => Token::End,
                                ('?', '?') => Token::Exit,
                                _ => {
                                    println!("{}{}Logic error.{}", BOLD, RED, RESET);
                                    process::exit(1)
                                }
                            })
                        }
                        None => first_word = Some(c),
                    }
                    word_head = 0;
                    continue;
                } else {
                    Err("k")
                }
            }
            _ => continue,
        };

        if let Err(expected) = result {
            println!(
                "{}{}Encountered {} but expected {}.{}",
                BOLD, RED, c, expected, RESET
            );
            process::exit(1);
        }

        word_head += 1
    }

    if word_head != 0 {
        println!(
            "{}{}Last Ook has been partially eaten. Please restitute the end.{}",
            BOLD, RED, RESET
        );
        process::exit(1);
    }

    if let Some(_) = first_word {
        println!(
            "{}{}Last Ook is alone. Please give a friend to it.{}",
            BOLD, RED, RESET
        );
        process::exit(1);
    }

    tokenized
}

fn spoon_parser(plain: &str) -> Vec<Token> {
    let plain: Vec<char> = plain.chars().collect();
    let mut tokenized = Vec::new();
    let mut word = ['0'; 8];
    let mut word_head: usize = 0;

    for c in plain {
        match c {
            '0' | '1' => word[word_head] = c,
            _ => continue,
        }

        let token = match word_head {
            0 => {
                if word[0] == '1' {
                    Some(Token::Incr)
                } else {
                    None
                }
            }
            2 => match word[1..=2] {
                ['1', '0'] => Some(Token::Right),
                ['1', '1'] => Some(Token::Left),
                ['0', '0'] => Some(Token::Decr),
                _ => None,
            },
            3 => {
                if word[3] == '1' {
                    Some(Token::End)
                } else {
                    None
                }
            }
            4 => {
                if word[4] == '0' {
                    Some(Token::Begin)
                } else {
                    None
                }
            }
            5 => {
                if word[5] == '0' {
                    Some(Token::Out)
                } else {
                    None
                }
            }
            6 => {
                if word[6] == '0' {
                    Some(Token::In)
                } else {
                    None
                }
            }
            7 => {
                if word[7] == '0' {
                    Some(Token::Print)
                } else if word[7] == '1' {
                    Some(Token::Exit)
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(token) = token {
            tokenized.push(token);
            word_head = 0;
        } else {
            word_head += 1
        }
    }

    tokenized
}

fn main() {
    println!("{}{}Reading code...{}", BOLD, BLUE, RESET);

    let args: Vec<_> = env::args().collect();

    let program = if 2 <= args.len() {
        &args[1]
    } else {
        println!(
            "{}{}Please provide the program as first argument.{}",
            BOLD, RED, RESET
        );
        process::exit(1);
    };

    let lang = if 3 <= args.len() {
        if args[2] == "--ook" {
            Language::Ook
        } else if args[2] == "--spoon" {
            Language::Spoon
        } else if args[2] == "--brainfuck" {
            Language::Brainfuck
        } else {
            println!(
                "{}{}Second argument should be --brainfuck, --ook or --spoon.{}",
                BOLD, RED, RESET
            );
            process::exit(1);
        }
    } else {
        Language::Brainfuck
    };

    println!("{}", program);

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();

    println!("{}{}Ready to brainfuck?{}", BOLD, GREEN, RESET);
    println!("{}{}Please provide the input:{}", BOLD, BLUE, RESET);
    handle.read_line(&mut input).expect("Error reading stdin.");
    println!("{}{}Please input provided!{}", BOLD, BLUE, RESET);
    println!("{}{}Tokenizing...{}", BOLD, BLUE, RESET);

    let program = match lang {
        Language::Brainfuck => brainfuck_parser(&program),
        Language::Ook => ook_parser(&program),
        Language::Spoon => spoon_parser(&program),
    };

    println!("{:?}", program);

    println!("{}{}Executing...{}", BOLD, BLUE, RESET);

    let program_len = program.len();
    let mut input = input.bytes();
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    let mut program_head: usize = 0;
    let mut memory_head: usize = 0;
    let mut output = Vec::new();

    'exec: while program_head < program_len {
        match program[program_head] {
            Token::Right => memory_head += 1,
            Token::Left => memory_head -= 1,
            Token::Incr => memory[memory_head] = memory[memory_head].wrapping_add(1),
            Token::Decr => memory[memory_head] = memory[memory_head].wrapping_sub(1),
            Token::Out => output.push(memory[memory_head]),
            Token::In => memory[memory_head] = input.nth(0).expect("Input is empty!"),
            Token::Begin => {
                if memory[memory_head] == 0 {
                    loop {
                        match program[program_head] {
                            Token::End => break,
                            _ => {
                                if program_len <= program_head {
                                    break 'exec;
                                }
                                program_head += 1
                            }
                        }
                    }
                }
            }
            Token::End => {
                if memory[memory_head] != 0 {
                    loop {
                        match program[program_head] {
                            Token::Begin => break,
                            _ => {
                                if program_len == 0 {
                                    break 'exec;
                                }
                                program_head -= 1
                            }
                        }
                    }
                }
            }
            Token::Exit => break 'exec,
            Token::Print => output.extend(memory.iter()),
        }

        program_head += 1;
    }

    println!("{}{}Executed! Here is the output:{}", BOLD, GREEN, RESET);
    println!("{}", String::from_utf8_lossy(&output));
}
