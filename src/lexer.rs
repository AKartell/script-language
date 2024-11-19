use core::panic;
use std::{iter::Peekable, ops::Deref};

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

/// Input is the string that we need to Tokenize, basically the code.
/// Index helps us keep track of each character, it is helpful at:
///     - Start of words.
///     - Error handling.
pub struct Lexer<'a> {
    input: &'a str,
    index: usize,
    next_index: usize,
}
impl<'a> Lexer<'a> {
    fn expect(&mut self, character: char) -> bool {
        if let Some(next) = self.input[self.next_index..].chars().next() {
            if next == character {
                self.next_index += character.len_utf8();
                return true;
            }
        };

        return false;
    }

    fn char_token(&mut self, token: TokenType<'a>) -> Option<Result<Token<'a>, LexerError>> {
        let start_index = self.index;
        self.index = self.next_index;
        return Some(Ok(Token {
            start: start_index,
            end: self.next_index,
            token_type: token,
        }));
    }

    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input,
            index: 0,
            next_index: 0,
        }
    }
}
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub start: usize,
    pub end: usize,
}
impl Token<'_> {
    pub fn get_type(&self) -> TokenType<'_> {
        return self.token_type.clone();
    }
}
#[derive(Error, Debug, Diagnostic)]
#[error("Error while lexing.")]
#[diagnostic(
    code(oops::my::bad),
    url(docsrs),
    help("Contact the customer support... oh we don't have one!")
)]
pub struct LexerErrorStruct {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: String,
    // Snippets and highlights can be included in the diagnostic!
    #[label("This bit here")]
    bad_bit: SourceSpan,
}
#[derive(Error, Debug, Diagnostic)]
pub enum LexerError {
    #[error("Error while lexing.")]
    Failed(LexerErrorStruct),
    #[error("Thanks one dot is enough!")]
    DotErr(LexerErrorStruct),
    #[error("Unfinished string!")]
    UnfStr(LexerErrorStruct),
    #[error("Error while parsing!")]
    ParsErr(LexerErrorStruct),
    #[error("Unkown character!")]
    UnkChar(LexerErrorStruct),
}
/// This stores all the Tokens, that later will be parsed.
/// To add a new Token, add it here, and then in State Machine of the next() function.
/// If an identifier, it is enough to specify it in the Identifier state in the return.
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum TokenType<'a> {
    /// (
    ParenLeft,
    /// )
    ParenRight,

    BraceLeft,
    BraceRight,

    GreatEqual,
    Great,
    LessEqual,
    Less,
    Equal,
    Bang,
    BangEqual,
    SemiColon, // Colon :DD
    EqualEqual,

    String(&'a str),
    Integer(i32),
    Float(f32),
    Identifier(&'a str),

    Plus,
    PlusEqual,
    Increment,
    Minus,
    MinusEqual,
    Decrement,
    Star,
    Slash,

    True,
    False,

    And,
    Or,

    If,
    Else,
    While,
    Let,
    Return,
}

#[derive(PartialEq, Debug)]
enum State {
    Unknown,
    Integer,
    Float,
    StartString,
    EndString,
    Identifier,
}
pub trait Check {
    fn expect(&mut self, token: TokenType) -> bool;
    fn maybe_expect(&mut self, token: TokenType) -> bool;
    fn panic_expect(&mut self, token: TokenType) -> bool;
}

impl<'a> Check for Peekable<Lexer<'a>> {
    fn expect(&mut self, token: TokenType) -> bool {
        match self.peek() {
            Some(Ok(t)) if t.get_type() == token => {
                self.next();
                true
            }
            _ => false,
        }
    }

    fn maybe_expect(&mut self, token: TokenType) -> bool {
        match self.peek() {
            Some(Ok(t)) if t.get_type() == token => true,
            _ => false,
        }
    }

    fn panic_expect(&mut self, token: TokenType) -> bool {
        match self.peek() {
            Some(Ok(t)) if t.get_type() == token => {
                self.next();
                true
            }
            _ => panic!("No {:?} was found!", token),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Creates an iterator from the characters.
        let mut characters = self.input[self.index..].chars();
        let mut from_to = self.index;
        let mut state = State::Unknown;

        // Search token bounds.
        loop {
            let Some(character) = characters.next() else {
                break;
            };

            self.next_index = self.index + character.len_utf8();

            //println!("State: {:?} Current: \"{}\" Character: \'{}\'", state, &self.input[from_to..self.next_index], character);
            if character.is_whitespace() {
                match state {
                    State::Unknown => {
                        self.index = self.next_index;
                        continue;
                    }
                    State::StartString => (),
                    _ => break,
                }
            }

            match state {
                State::Unknown => {
                    from_to = self.index;
                    match character {
                        '>' if self.expect('=') => return self.char_token(TokenType::GreatEqual),
                        '>' => return self.char_token(TokenType::Great),
                        '<' if self.expect('=') => return self.char_token(TokenType::LessEqual),
                        '<' => return self.char_token(TokenType::Less),
                        '+' if self.expect('+') => return self.char_token(TokenType::Increment),
                        '+' if self.expect('=') => return self.char_token(TokenType::PlusEqual),
                        '+' => return self.char_token(TokenType::Plus),
                        '-' if self.expect('-') => return self.char_token(TokenType::Decrement),
                        '-' if self.expect('=') => return self.char_token(TokenType::MinusEqual),
                        '-' => return self.char_token(TokenType::Minus),
                        ')' => return self.char_token(TokenType::ParenRight),
                        '(' => return self.char_token(TokenType::ParenLeft),
                        '!' if self.expect('=') => return self.char_token(TokenType::BangEqual),
                        '!' => return self.char_token(TokenType::Bang),
                        '=' if self.expect('=') => return self.char_token(TokenType::EqualEqual),
                        '&' if self.expect('&') => return self.char_token(TokenType::And),
                        '|' if self.expect('|') => return self.char_token(TokenType::Or),
                        '=' => return self.char_token(TokenType::Equal),
                        ';' => return self.char_token(TokenType::SemiColon),
                        '{' => return self.char_token(TokenType::BraceLeft),
                        '}' => return self.char_token(TokenType::BraceRight),
                        '*' => return self.char_token(TokenType::Star),
                        '/' => return self.char_token(TokenType::Slash),
                        '0'..='9' => state = State::Integer,
                        '_' | 'A'..='Z' | 'a'..='z' => state = State::Identifier,
                        '"' => state = State::StartString,
                        _ => {
                            return Some(Err(LexerError::UnkChar(LexerErrorStruct {
                                src: self.input.into(),
                                bad_bit: (from_to, self.index - from_to).into(),
                            })))
                        }
                    }
                }
                State::Integer => match character {
                    '0'..='9' => (),
                    '.' => state = State::Float,
                    _ => break,
                },
                State::Float => {
                    match character {
                        '0'..='9' => (),
                        // Two or more dots!
                        '.' => {
                            return Some(Err(LexerError::DotErr(LexerErrorStruct {
                                src: self.input.into(),
                                bad_bit: (from_to, self.index - from_to).into(),
                            })))
                        }
                        _ => break,
                    }
                }
                State::StartString => {
                    // This means we have reached the second "" which means the string is final.
                    if character == '"' {
                        state = State::EndString;
                        //Some(Token::String(&self.input[from_to + '\"'.len_utf8()..self.index - '\"'.len_utf8()]))
                    }
                }
                State::EndString => {
                    break;
                }
                State::Identifier => match character {
                    '_' | 'A'..='Z' | 'a'..='z' | '0'..='9' => (),
                    _ => break,
                },
            };

            self.index = self.next_index;
        }

        // Create token.

        //println!("Final:\nState: {:?} Current: \"{}\"\n", state, &self.input[from_to..self.index]);

        let token_type = match state {
            State::Unknown => None,
            State::Integer => match &self.input[from_to..self.index].parse::<i32>() {
                Ok(num) => Some(TokenType::Integer(*num)),
                Err(e) => {
                    return Some(Err(LexerError::ParsErr(LexerErrorStruct {
                        src: self.input.into(),
                        bad_bit: (from_to, self.index - from_to).into(),
                    })))
                }
            },
            State::Float => match &self.input[from_to..self.index].parse::<f32>() {
                Ok(num) => Some(TokenType::Float(*num)),
                Err(e) => {
                    return Some(Err(LexerError::ParsErr(LexerErrorStruct {
                        src: self.input.into(),
                        bad_bit: (from_to, self.index - from_to).into(),
                    })))
                }
            },
            State::StartString => {
                return Some(Err(LexerError::UnfStr(LexerErrorStruct {
                    src: self.input.into(),
                    bad_bit: (from_to, self.index - from_to).into(),
                })))
            }
            State::EndString => Some(TokenType::String(
                &self.input[from_to + '\"'.len_utf8()..self.index - '\"'.len_utf8()],
            )),
            State::Identifier => match &self.input[from_to..self.index] {
                "if" => Some(TokenType::If),
                "else" => Some(TokenType::Else),
                "while" => Some(TokenType::While),
                "let" => Some(TokenType::Let),
                "return" => Some(TokenType::Return),
                "true" => Some(TokenType::True),
                "false" => Some(TokenType::False),
                id => Some(TokenType::Identifier(id)),
            },
        };
        match token_type {
            Some(val) => {
                return Some(Ok(Token {
                    start: from_to,
                    end: self.index,
                    token_type: val,
                }))
            }
            None => return None,
        }
    }
}
