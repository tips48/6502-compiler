use std::collections::VecDeque;

#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Number(u8),
    Word(String),
}

#[derive(Debug)]
enum TokenzierStateMachine {
    NewToken,
    Number,
    Word,
    EOF
}

type TSM = TokenzierStateMachine;

pub fn tokenize(chars: Vec<char>) -> VecDeque<Token> {
    let mut state = TSM::NewToken;

    let mut tokens: VecDeque<Token> = VecDeque::new();
    let mut word = String::new();

    let size = chars.len();
    let mut index = 0;
    let mut grab_char = true;
    let mut c = ' ';

    while index < size {
        if grab_char {
            c = chars[index];
            index += 1;
        }

        grab_char = true; // by default get a character each loop

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
                } else if c == '-' {
                    tokens.push_back(Token::Keyword("-".to_string()));
                } else if c == '~' {
                    tokens.push_back(Token::Keyword("~".to_string()));
                } else if c == '!' {
                    tokens.push_back(Token::Keyword("!".to_string()));
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
                    tokens.push_back(Token::Number(word.parse::<u8>().unwrap()));
                    state = TSM::NewToken;
                    word.clear();
                } else if !c.is_numeric() {
                    tokens.push_back(Token::Number(word.parse::<u8>().unwrap()));
                    state = TSM::NewToken;
                    word.clear();

                    grab_char = false; // Process into keyword or word
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

                    grab_char = false; // Process into keyword
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
