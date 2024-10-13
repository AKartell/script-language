/// Input is the string that we need to Tokenize, basically the code.
/// Index helps us keep track of each character, it is helpful at:
///     - Start of words.
///     - Error handling.
struct Lexer<'a> {
    input: &'a str,
    index: usize,
}
impl<'a> Lexer<'a> {
    fn character_token(&mut self, character: char, token_type: Token<'a>) -> Option<Token<'a>>{
        self.index += character.len_utf8();
        return Some(token_type);
    }
    
    pub fn new(input: &'a str) -> Lexer<'a>
    {
        Lexer {
            input: input,
            index: 0
        }
    }
    
}
#[derive(PartialEq, Debug)]
enum Token<'a> {
    ParenLeft,
    ParenRight,
    String(&'a str),
    Integer(i32),
    Float(f32),
    Plus,
    Minus,
    Increment,
    Decrement,
    Eof
}

#[derive(PartialEq, Debug)]
enum State {
    Unknown,
    Integer,
    Float,
    StartString,
    EndString,
    Identifier,
    Operator
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

            let next_index: usize = self.index + character.len_utf8();
            
            //println!("State: {:?} Current: \"{}\" Character: \'{}\'", state, &self.input[from_to..next_index], character);
            if character.is_whitespace()  {
                if state == State::Unknown {
                    self.index = next_index;
                    continue;
                }
                else {
                    break;
                }
            }
            
            match state {
                State::Unknown => {
                    from_to = self.index;
                    // Extremely hacky solution for single character tokens.
                    let temp_val = self.index;
                    self.index = next_index;
                    match character {
                        ')' | '(' | '+' | '-' => state = State::Operator,

                        '0'..'9' => state = State::Integer,
                        'A'..'Z' | 'a'..'z' => state = State::Identifier,
                        '"' => state = State::StartString,
                        _ => panic!("Unknown character!")
                    }
                    self.index = temp_val;
                },
                State::Integer => {
                    match character {
                        '0'..'9' => (),
                        '.' => state = State::Float,
                        _ => break
                    } 
                },
                State::Float => {
                    match character {
                        '0'..'9' => (),
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
                State::Operator => {
                    match &self.input[from_to..next_index] {
                        "++" => (),
                        "--" => (),
                        _ => break
                    }
                }
                _ => todo!()
            };
            //println!("Lefutott;");
            self.index = next_index;
        };

        // Create token.
        println!("Final:\nState: {:?} Current: \"{}\"\n", state, &self.input[from_to..self.index]);

        return match state {
            State::Unknown => Some(Token::Eof),
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
            State::Identifier => todo!(),
            State::Operator => match &self.input[from_to..self.index] {
                "+" => Some(Token::Plus),
                "++" => Some(Token::Increment),
                "-" => Some(Token::Minus),
                "--" => Some(Token::Decrement),
                ")" => Some(Token::ParenRight),
                "(" => Some(Token::ParenLeft),
                _ => panic!("This should not be possible!")
            }
        };
    }
} 

fn main() {
    //println!("{}", &"wasd"[0..1]);
    let mut lexer = Lexer::new(" (123.0) +- \"asd\"  -- ");
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
    fn operators() {
        let mut lexer = Lexer::new("+ - ++ --");
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Increment));
        assert_eq!(lexer.next(), Some(Token::Decrement));
        
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
        assert_eq!(lexer.next(), Some(Token::Eof));
    }
}