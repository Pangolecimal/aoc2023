#![allow(dead_code)]

use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("input-2.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(v) => v,
        Err(e) => panic!("Could not find file {}: {}", display, e),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => println!("Successfully read {}", display),
        Err(e) => panic!("Could not read file {}: {}", display, e),
    }

    println!("Result: {}", task_2(s));
}

fn task_1(s: String) -> u32 {
    let mut result: u32 = 0;
    for line in s.lines() {
        let mut first_digit_opt: Option<u32> = None;
        let mut last_digit_opt: Option<u32> = None;

        for c in line.chars() {
            if c.is_ascii_digit() {
                first_digit_opt = c.to_digit(10);
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                last_digit_opt = c.to_digit(10);
                break;
            }
        }

        let first_digit = match first_digit_opt {
            Some(v) => v,
            None => panic!("Digit not found: {}", line),
        };
        let last_digit = match last_digit_opt {
            Some(v) => v,
            None => panic!("Digit not found: {}", line),
        };

        println!("{}: {}{}", line, first_digit, last_digit);
        let value = first_digit * 10 + last_digit;
        result += value;
    }
    result
}

fn task_2(s: String) -> u32 {
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut result: u32 = 0;
    for line in s.lines() {
        let mut first_digit_opt: Option<u32> = None;
        let mut last_digit_opt: Option<u32> = None;

        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if first_digit_opt == None {
                    first_digit_opt = c.to_digit(10);
                }
                last_digit_opt = c.to_digit(10);
            }

            for (j, d) in digits.iter().enumerate() {
                if line.len() - i >= d.len() {
                    let mut matched = true;
                    for k in 0..d.len() {
                        if line.as_bytes()[i + k] != d.as_bytes()[k] {
                            matched = false;
                            break;
                        }
                    }
                    if matched {
                        if first_digit_opt == None {
                            first_digit_opt = Some(j as u32 + 1);
                        }
                        last_digit_opt = Some(j as u32 + 1);
                    }
                }
            }
        }

        let first_digit = match first_digit_opt {
            Some(v) => v,
            None => panic!("Digit 1 not found: {}", line),
        };
        let last_digit = match last_digit_opt {
            Some(v) => v,
            None => panic!("Digit 2 not found: {}", line),
        };

        // println!("{}: {}{}", line, first_digit, last_digit);
        let value = first_digit * 10 + last_digit;
        result += value;
    }
    result
}
