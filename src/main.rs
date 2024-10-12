/// Input is the string that we need to Tokenize, basically the code.
/// Index helps us keep track of each character, it is helpful at:
///     - Start of words.
///     - Error handling.
struct Lexer<'a> {
    input: &'a str,
    index: usize,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a>
    {
        Lexer {
            input: input,
            index: 0
        }
    }
    fn get(&self, current: usize) -> &'a str {
        &self.input[current..self.index]
    }
}
#[derive(PartialEq, Debug)]
enum Token<'a> {
    ParenLeft,
    ParenRight,
    String(&'a str),
    Integer(i32),
    Float(f32),
    Eof
}

#[derive(PartialEq)]
enum State {
    Unknown,
    Integer,
    Float,
    String,
    Identifier
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Creates an iterator from the characters.
        let mut characters = self.input[self.index..].chars();
        let mut current = self.index;
        let mut state = State::Unknown;
        loop {
            let Some(character) = characters.next() else {
                return match state {
                    State::Unknown => Some(Token::Eof),
                    State::Integer => Some(Token::Integer(self.get(current).parse().unwrap())),
                    State::Float => Some(Token::Float(self.get(current).parse().unwrap())),
                    State::String => panic!("Unfinished string!"),
                    State::Identifier => todo!()
                };
            };
            

            
            
            if character.is_whitespace()  {
                if state == State::Unknown {
                    current = self.index;
                    continue;
                }
                else {
                    return match state {
                        State::Unknown => Some(Token::Eof),
                        State::Integer => Some(Token::Integer(self.get(current).parse().unwrap())),
                        State::Float => Some(Token::Float(self.get(current).parse().unwrap())),
                        State::String => panic!("Unfinished string!"),
                        State::Identifier => todo!()
                    };
                }
            }
            self.index += character.len_utf8();
            match state {
                State::Unknown => {
                    match character {
                        ')' => return Some(Token::ParenRight),
                        '(' => return Some(Token::ParenLeft),

                        '0'..'9' => state = State::Integer,
                        'A'..'Z' | 'a'..'z' => state = State::Identifier,
                        '"' => state = State::String,
                        _ => panic!("Unknown character!")
                    }
                },
                State::Integer => {
                    match character {
                        '0'..'9' => continue,
                        '.' => state = State::Float,
                        _ => panic!("Wrong character at {}", self.index)
                    } 
                }
                State::String => {
                    // This means we have reached the second "" which means the string is final.
                    if character == '"'{
                        return Some(Token::String(&self.input[current + '\"'.len_utf8()..self.index - '\"'.len_utf8()]))
                    }
                },
                State::Float => {
                    match character {
                        '0'..'9' => continue,
                        // Two or more dots!
                        '.' => panic!("Thank you! One dot is enough."),
                        _ => panic!("Wrong character at {}", self.index)
                    } 
                }
                _ => todo!()
            }
        }
    }
} 

fn main() {
    let mut lexer = Lexer::new("123.0");
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
        if token == Token::Eof {
            break;  
        }
    }
    
    
}

#[cfg(test)]
mod tests {
    use crate::{Lexer, Token};

    #[test]
    fn simple_characters(){
        let mut lexer = Lexer::new("()");
        let vec = vec![lexer.next().unwrap(), lexer.next().unwrap()];
        assert_eq!(vec, [Token::ParenLeft, Token::ParenRight])
    }
    #[test]
    fn string() {
        let mut lexer = Lexer::new("\"test\"");
        assert_eq!(lexer.next(), Some(Token::String("test")))
    }
}