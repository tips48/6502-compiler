use std::collections::VecDeque;
use std::str::Chars;

#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Number(u32),
    Word(String),
}

#[derive(Debug, PartialEq)]
enum TokenzierStateMachine {
    NewToken,
    Number,
    Word,
    EOF
}

type TSM = TokenzierStateMachine;

pub fn tokenize(chars: Chars<'_>) -> VecDeque<Token> {
    let mut state = TSM::NewToken;

    let mut tokens: VecDeque<Token> = VecDeque::new();
    let mut word = String::new();

    for c in chars {
        match state {
            TSM::NewToken => {
                if c == '{' {
                    tokens.push_back(Token::Keyword("{".to_string()));
                } else if c == '}' {
                    tokens.push_back(Token::Keyword("}".to_string()));
                } else if c == '(' {
                    tokens.push_back(Token::Keyword("(".to_string()));
                } else if c == ')' {
                    tokens.push_back(Token::Keyword(")".to_string()));
                } else if c == ';' {
                    tokens.push_back(Token::Keyword(";".to_string()));
                } else if c.is_whitespace() {
                    // Do nothing
                } else if c.is_digit(10) {
                    state = TSM::Number;
                    word.push(c);
                } else if c.is_alphabetic() {
                    state = TSM::Word;
                    word.push(c);
                } else {
                    println!("Unknown char: {}", c);
                }
            },

            TSM::Number => {
                if c.is_whitespace() {
                    tokens.push_back(Token::Number(word.parse::<u32>().unwrap()));
                    state = TSM::NewToken;
                    word.clear();
                } else if !c.is_numeric() {
                    tokens.push_back(Token::Number(word.parse::<u32>().unwrap()));
                    state = TSM::NewToken;
                    word.clear();

                    if c == '{' {
                        tokens.push_back(Token::Keyword("{".to_string()));
                    } else if c == '}' {
                        tokens.push_back(Token::Keyword("}".to_string()));
                    } else if c == '(' {
                        tokens.push_back(Token::Keyword("(".to_string()));
                    } else if c == ')' {
                        tokens.push_back(Token::Keyword(")".to_string()));
                    } else if c == ';' {
                        tokens.push_back(Token::Keyword(";".to_string()));
                    }
                } else {
                    word.push(c);
                }
            },

            TSM::Word => {
                if c.is_whitespace() {
                    tokens.push_back(Token::Word(word.to_string()));
                    state = TSM::NewToken;
                    word.clear();
                } else if !c.is_alphanumeric() {
                    tokens.push_back(Token::Word(word.to_string()));
                    state = TSM::NewToken;
                    word.clear();

                    if c == '{' {
                        tokens.push_back(Token::Keyword("{".to_string()));
                    } else if c == '}' {
                        tokens.push_back(Token::Keyword("}".to_string()));
                    } else if c == '(' {
                        tokens.push_back(Token::Keyword("(".to_string()));
                    } else if c == ')' {
                        tokens.push_back(Token::Keyword(")".to_string()));
                    } else if c == ';' {
                        tokens.push_back(Token::Keyword(";".to_string()));
                    }
                } else {
                    word.push(c);
                }
            },

            TSM::EOF => {
                println!("Done tokenizing!");
            }
        }
    }

    tokens
}
