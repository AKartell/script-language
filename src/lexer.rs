use core::panic;
use std::iter::Peekable;

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
    fn expect(&mut self, character: char) -> bool{
        if let Some(next) = self.input[self.next_index..].chars().next() {
            if next == character {
                self.next_index += character.len_utf8();
                return true
            }
        };
        
        return false
    }
    
    fn char_token(&mut self, token: Token<'a>) -> Option<Token<'a>> {
        self.index = self.next_index;
        return Some(token);
    }
    
    pub fn new(input: &'a str) -> Lexer<'a>
    {
        Lexer {
            input: input,
            index: 0,
            next_index: 0,
        }
    }
    
}
/// This stores all the Tokens, that later will be parsed.
/// To add a new Token, add it here, and then in State Machine of the next() function.
/// If an identifier, it is enough to specify it in the Identifier state in the return.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token<'a> {
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
    fn expect(&mut self, token: Token) -> bool;
    fn maybe_expect(&mut self, token: Token) -> bool; 
    fn panic_expect(&mut self, token: Token) -> bool;
}
impl<'a> Check for Peekable<Lexer<'a>> {
    fn expect(&mut self, token: Token) -> bool {
        match self.peek() {
            Some(t) if *t == token => {
                self.next();
                true
            },
            _ => false
        }
    }
    
    fn maybe_expect(&mut self, token: Token) -> bool {
        match self.peek() {
            Some(t) if *t == token => {
                true
            },
            _ => false
        }
    }
    
    fn panic_expect(&mut self, token: Token) -> bool {
        match self.peek() {
            Some(t) if *t == token => {
                self.next();
                true
            },
            _ => panic!("No {:?} was found!", token)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;
    
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
            if character.is_whitespace()  {
                match state {
                    State::Unknown => {
                        self.index = self.next_index;
                        continue;
                    },
                    State::StartString => (),
                    _ => break
                }
                
            }
            
            match state {
                State::Unknown => {
                    from_to = self.index;
                    match character {
                        '>' if self.expect('=') => {
                            return self.char_token(Token::GreatEqual)
                        },
                        '>' => {
                            return self.char_token(Token::Great)
                        },
                        '<' if self.expect('=') => {
                            return self.char_token(Token::LessEqual)
                        },
                        '<' => {
                            return self.char_token(Token::Less)
                        },
                        '+' if self.expect('+') => {
                            return self.char_token(Token::Increment)
                        }
                        '+' if self.expect('=') => {
                            return self.char_token(Token::PlusEqual)
                        },
                        '+' => {
                            return self.char_token(Token::Plus)
                        },
                        '-' if self.expect('-') => {
                            return self.char_token(Token::Decrement)
                        },
                        '-' if self.expect('=') => {
                            return self.char_token(Token::MinusEqual)
                        },
                        '-' => {
                            return self.char_token(Token::Minus)
                        },
                        ')' => {
                            return self.char_token(Token::ParenRight)
                        },
                        '(' => {
                            return self.char_token(Token::ParenLeft)
                        },
                        '!' if self.expect('=') => {
                            return self.char_token(Token::BangEqual)
                        },
                        '!' => {
                            return self.char_token(Token::Bang)
                        },
                        '=' if self.expect('=') => {
                            return self.char_token(Token::EqualEqual)
                        }
                        '=' => {
                            return self.char_token(Token::Equal)
                        },
                        ';' => {
                            return self.char_token(Token::SemiColon)
                        },
                        '{' => {
                            return self.char_token(Token::BraceLeft)
                        },
                        '}' => {
                            return self.char_token(Token::BraceRight)
                        },
                        '*' => {
                            return self.char_token(Token::Star)
                        }

                        '0'..='9' => state = State::Integer,
                        '_' | 'A'..='Z' | 'a'..='z' => state = State::Identifier,
                        '"' => state = State::StartString,
                        _ => panic!("Unknown character!")
                    }
                },
                State::Integer => {
                    match character {
                        '0'..='9' => (),
                        '.' => state = State::Float,
                        _ => break
                    } 
                },
                State::Float => {
                    match character {
                        '0'..='9' => (),
                        // Two or more dots!
                        '.' => panic!("Thank you! One dot is enough."),
                        _ => break
                    } 
                },
                State::StartString => {
                    // This means we have reached the second "" which means the string is final.
                    if character == '"'{
                        state = State::EndString;
                        //Some(Token::String(&self.input[from_to + '\"'.len_utf8()..self.index - '\"'.len_utf8()]))
                    }
                },
                State::EndString => {
                    break;
                },
                State::Identifier => {
                    match character {
                        '_' | 'A'..='Z' | 'a'..='z' | '0'..='9' => (),
                        _ => break
                    }
                }
            };
            
            self.index = self.next_index;
        };

        // Create token.

        //println!("Final:\nState: {:?} Current: \"{}\"\n", state, &self.input[from_to..self.index]);

        return match state {
            State::Unknown => None,
            State::Integer => {
                match &self.input[from_to..self.index].parse::<i32>(){
                    Ok(num) => Some(Token::Integer(*num)),
                    Err(e) => panic!("{:?}", e)
                }
                
            },
            State::Float => {
                match &self.input[from_to..self.index].parse::<f32>(){
                    Ok(num) => Some(Token::Float(*num)),
                    Err(e) => panic!("{:?}", e)
                }
            },
            State::StartString => panic!("Unfinished string!"),
            State::EndString => Some(Token::String(&self.input[from_to + '\"'.len_utf8()..self.index - '\"'.len_utf8()])),
            State::Identifier => match &self.input[from_to..self.index]{
                "if" => Some(Token::If),
                "else" => Some(Token::Else),
                "while" => Some(Token::While),
                "let" => Some(Token::Let),
                "return" => Some(Token::Return),
                id => Some(Token::Identifier(id))
            },
        };
    }
} 

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};
    #[test]
    fn operators() {
        let mut lexer = Lexer::new("+ - ++ --");
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Increment));
        assert_eq!(lexer.next(), Some(Token::Decrement));
        
    }
    #[test]
    fn basic_operation() {
        let mut lexer = Lexer::new("1 + 2 * 3 - 4");
        assert_eq!(lexer.next().unwrap(), Token::Integer(1));
        assert_eq!(lexer.next().unwrap(), Token::Plus);

        assert_eq!(lexer.next().unwrap(), Token::Integer(2));
        assert_eq!(lexer.next().unwrap(), Token::Star);

        assert_eq!(lexer.next().unwrap(), Token::Integer(3));
        assert_eq!(lexer.next().unwrap(), Token::Minus);

        assert_eq!(lexer.next().unwrap(), Token::Integer(4));
    }
    #[test]
    fn string() {
        let mut lexer = Lexer::new("\"Test string\"");
        assert_eq!(lexer.next(), Some(Token::String("Test string")));
    }
    #[test]
    fn simple_characters() {
        let mut lexer = Lexer::new("()");
        let vec = vec![lexer.next().unwrap(), lexer.next().unwrap()];
        assert_eq!(vec, [Token::ParenLeft, Token::ParenRight])
    }
    #[test]
    fn basic_types() {
        let mut lexer = Lexer::new("\"test\" 123 123.0");
        assert_eq!(lexer.next(), Some(Token::String("test")));
        assert_eq!(lexer.next(), Some(Token::Integer(123)));
        assert_eq!(lexer.next(), Some(Token::Float(123.0)));
        assert_eq!(lexer.next(), None);
    }
}