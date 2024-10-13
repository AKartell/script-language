/// Input is the string that we need to Tokenize, basically the code.
/// Index helps us keep track of each character, it is helpful at:
///     - Start of words.
///     - Error handling.
struct Lexer<'a> {
    input: &'a str,
    index: usize,
}
impl<'a> Lexer<'a> {
    fn token_from(&self, state: State, from_to: usize) -> Option<Token<'a>> {
        return match state {
            State::Unknown => Some(Token::Eof),
            State::Integer => {
                let parsed = &self.get(from_to).parse::<i32>();
                println!("{:?}", parsed);
                match parsed{
                    Ok(num) => Some(Token::Integer(*num)),
                    Err(e) => panic!("{:?}", e)
                }
                
            },
            State::Float => Some(Token::Float(self.get(from_to).parse().unwrap())),
            State::String => panic!("Unfinished string!"),
            State::Identifier => todo!()
        };
    }
    pub fn new(input: &'a str) -> Lexer<'a>
    {
        Lexer {
            input: input,
            index: 0
        }
    }
    fn get(&self, from_to: usize) -> &'a str {
        &self.input[from_to..self.index]
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

#[derive(PartialEq, Debug)]
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
        let mut from_to = self.index;
        let mut state = State::Unknown;
        // Search token bounds.
        loop {
            let start_index = self.index;
            println!("{:?}", state);
            let Some(character) = characters.next() else {
                return self.token_from(state, from_to);
            };
            
            self.index += character.len_utf8();
            
            println!("{}", &self.input[from_to..self.index]);
            if character.is_whitespace()  {
                if state == State::Unknown {
                    from_to = self.index;
                    continue;
                }
                else {
                    self.index = start_index;
                    return self.token_from(state, from_to);
                }
            }
            
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
                        _ => return {
                            Some(Token::Integer(self.get(from_to).parse().unwrap()))

                        }
                    } 
                },
                State::Float => {
                    match character {
                        '0'..'9' => continue,
                        // Two or more dots!
                        '.' => panic!("Thank you! One dot is enough."),
                        _ => return Some(Token::Float(self.get(from_to).parse().unwrap()))
                    } 
                },
                State::String => {
                    // This means we have reached the second "" which means the string is final.
                    if character == '"'{
                        return Some(Token::String(&self.input[from_to + '\"'.len_utf8()..self.index - '\"'.len_utf8()]))
                    }
                },
                
                _ => todo!()
            }
        }
        
    }
} 

fn main() {
    let mut lexer = Lexer::new(" 123.0 ");
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
    fn basic_types() {
        let mut lexer = Lexer::new("\"test\" 123 123.0");
        let string = lexer.next();
        let integer = lexer.next();
        let float = lexer.next();
        let eof = lexer.next();
        assert_eq!(string, Some(Token::String("test")), "string: {:?}", string);
        assert_eq!(integer, Some(Token::Integer(123)), "integer: {:?}", integer);
        assert_eq!(float, Some(Token::Float(123.0)), "float: {:?}", float);
        assert_eq!(eof, Some(Token::Eof), "eof: {:?}", eof);
    }
}