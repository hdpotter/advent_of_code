use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("src/bin/input/aoc2024_day3")?;

    let mut state = State::ExpectMOrD;

    let mut first_vec = Vec::new();
    let mut second_vec = Vec::new();

    let mut enabled = true;

    let mut total = 0;
    let mut total_enabled = 0;

    for char in input.chars() {
        match state {
            State::ExpectMOrD => match char {
                'm' => {
                    state = State::ExpectU;

                    first_vec.clear();
                    second_vec.clear();
                },
                'd' => state = State::ExpectO,
                _ => (),
            },
            State::ExpectU => match char {
                'u' => state = State::ExpectL,
                _ => state = State::ExpectMOrD,
            },
            State::ExpectL => match char {
                'l' => state = State::ExpectLParenMul,
                _ => state = State::ExpectMOrD,
            },
            State::ExpectLParenMul => match char {
                '(' => state = State::ExpectFirstDigitOrComma,
                _ => state = State::ExpectMOrD,
            }
            State::ExpectFirstDigitOrComma => match char {
                '0'..='9' => first_vec.push(char),
                ',' => state = State::ExpectSecondDigitOrRParen,
                _ => state = State::ExpectMOrD,
            },
            State::ExpectSecondDigitOrRParen => match char {
                '0'..='9' => second_vec.push(char),
                ')' => {
                    state = State::ExpectMOrD;

                    // might be able to skip these allocations if we needed to
                    let left = String::from_iter(&first_vec).parse::<u32>().unwrap();
                    let right = String::from_iter(&second_vec).parse::<u32>().unwrap();
                    
                    total += left * right;

                    if enabled {
                        total_enabled += left * right;
                    }
                },
                _ => state = State::ExpectMOrD,
            },
            State::ExpectO => match char {
                'o' => state = State::ExpectLParenOrN,
                _ => state = State::ExpectMOrD,
            },
            State::ExpectLParenOrN => match char {
                '(' => state = State::ExpectRParenDo,
                'n' => state = State::ExpectApostrophe,
                _ => state = State::ExpectMOrD,
            },
            State::ExpectRParenDo => match char {
                ')' => {
                    state = State::ExpectMOrD;
                    enabled = true;
                },
                _ => state = State::ExpectMOrD,
            },
            State::ExpectApostrophe => match char {
                '\'' => state = State::ExpectT,
                _ => state = State::ExpectMOrD,
            },
            State::ExpectT => match char {
                't' => state = State::ExpectLParenDont,
                _ => state = State::ExpectMOrD,
            },
            State::ExpectLParenDont => match char {
                '(' => state = State::ExpectRParenDont,
                _ => state = State::ExpectMOrD,
            },
            State::ExpectRParenDont => match char {
                ')' => {
                    state = State::ExpectMOrD;
                    enabled = false;
                },
                _ => state = State::ExpectMOrD,
            }
        }
    };

    println!("total: {total}");
    println!("total enabled : {total_enabled}");

    Ok(())
}

pub enum State {
    ExpectMOrD,
    ExpectU,
    ExpectL,
    ExpectLParenMul,
    ExpectFirstDigitOrComma,
    ExpectSecondDigitOrRParen,

    ExpectO,
    ExpectLParenOrN,
    ExpectRParenDo,
    ExpectApostrophe,
    ExpectT,
    ExpectLParenDont,
    ExpectRParenDont,
}