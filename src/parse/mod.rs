use std::{fmt::Display, iter::Peekable, vec};
mod display;
use crate::lexer::{self, Check, Lexer, Token};
/// Needed because of lifetime complications.
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>
}

enum Operator {
    Minus,
    Plus,
    Star,
    Bang
}

enum Atomic<'a> {
    Integer(i32),
    String(&'a str),
    Float(f32),
    Nil
}
pub enum TokenTree<'a> {
    Atomic(Atomic<'a>),
    // [TokenTree; 2]????
    Expression(Operator, Vec<TokenTree<'a>>),
    If {
        // We have to store them on the Heap, 
        // because otherwise it has infinite size.
        // Something that we cannot store on the Stack.
        condition: Box<TokenTree<'a>>,
        positive: Box<TokenTree<'a>>,
        negative: Option<Box<TokenTree<'a>>>
    }
}
impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        Parser {
            lexer: Lexer::new(input).peekable()
        }
    }
    pub fn parse(mut self) -> TokenTree<'a> {
        self.expression( 0)
    }
    fn expected_token_parse(&mut self, token: Token) -> TokenTree<'a> {
        if self.lexer.expect(token) {
            TokenTree::Atomic(Atomic::Nil)
        }
        else {
            let val = self.expression(0);
            self.lexer.panic_expect(token);
            val
        }

    }
    fn expression(&mut self, min_bp: u8) -> TokenTree<'a> {
        let token = match self.lexer.next() {
            Some(token) => token,
            None => return TokenTree::Atomic(Atomic::Nil)
        };
        println!("{:?}", token);
        // Short for Left Hand Side.
        let mut lhs: TokenTree = match token {
            Token::Float(num) => TokenTree::Atomic(Atomic::Float(num)),
            Token::Integer(num) => TokenTree::Atomic(Atomic::Integer(num)),
            Token::String(string) => TokenTree::Atomic(Atomic::String(string)),
            Token::Plus | Token::Minus => {
                let operator = match token {
                    Token::Plus => Operator::Plus,
                    Token::Minus => Operator::Minus,
                    _ => unreachable!("This is impossible to reach.")
                };

                let ((), right_bp) = prefix_binding_power(&operator);
                let rhs = self.expression(right_bp);
                TokenTree::Expression(operator, vec![rhs])
            },
            Token::ParenLeft => {
                let lhs = self.expected_token_parse(Token::ParenRight);
                lhs
            },
            Token::If => {
                // We want to see a condition, and after that a block. Maybe an else, and another block.
                let condition = self.expression(0);
                self.lexer.panic_expect(Token::BraceLeft);
                let positive = self.expected_token_parse(Token::BraceRight);
                

                let negative = if self.lexer.expect(Token::Else) {
                    self.lexer.panic_expect(Token::BraceLeft);
                    let val =self.expected_token_parse(Token::ParenRight);
                    Some(Box::new(val))
                }
                else {
                    None
                };
                TokenTree::If { condition: Box::new(condition), positive: Box::new(positive), negative: negative }
            }
            t => panic!("Bad token on left hand side. {:?}", t)
        };
    
        loop {
            // We peek, because it is recursive.
            // If this fails, we go back to the parent, but that parent is still in a loop.
            // 4D chess.
            let Some(token) = self.lexer.peek() else {
                break;
            };
            
            let operator = match token {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                Token::Star => Operator::Star,
                Token::Bang => Operator::Bang,
                
                t => {
                    println!("Unindetified operator: {:?}", t);
                    break;
                }
            };
            // Left hand binding power, and Right hand binding power.
            // If we have a special character and parsing folds, because of the break at end,
            // we get the Expression for further processing.For example parsing an if statement:
            // if variable >= 10 {}
            // ^  ^            ^ ^
            // id - expression - expected token
            if let Some((left_bp, ())) = postfix_binding_power(&operator) {
                if left_bp < min_bp {
                    break;
                }
                // We can now iterate, because we know that this specific Token checks out,
                // so we can move down another layer and check if we can collapse.
                self.lexer.next();
                
                
                lhs = TokenTree::Expression(operator, vec![lhs]);
                continue;
            }
            if let Some((left_bp, right_bp)) = infix_binding_power(&operator) {
                if left_bp < min_bp {
                    break;
                }
                // We can now iterate, because we know that this specific Token checks out,
                // so we can move down another layer and check if we can collapse.
                self.lexer.next();
    
                // Short of Right Hand Side
                let rhs = self.expression(right_bp);
                
                lhs = TokenTree::Expression(operator, vec![lhs, rhs]);
                continue;
            }
           break;
        };
    
        lhs
    }
    
    
}
/// Operator has two atomic neighbours.
fn infix_binding_power(operator: &Operator) -> Option<(u8, u8)> {
    match operator {
        Operator::Minus | Operator::Plus => Some((1, 2)),
        Operator::Star => Some((3, 4)),
        _ => None
    }
}
/// Operator only has one atomic neighbour to its right.
fn prefix_binding_power(operator: &Operator) -> ((), u8) {
    match operator {
        Operator::Plus | Operator::Minus => ((), 5),
        _ => panic!("Bad prefix operator!")
    }
}
fn postfix_binding_power(operator: &Operator) -> Option<(u8, ())>{
    match operator {
        Operator::Bang => Some((7, ())),
        _ => None
    }
}
#[cfg(test)]
mod tests{
    use crate::parse::Parser;

    

    #[test]
    fn expression_print() {
        let parser = Parser::new("1 + 2 * 3");
        let string = parser.parse();

        assert_eq!(string.to_string(), "(+ 1 (* 2 3))")
    }
    #[test]
    fn braces() {
        assert_eq!(Parser::new("((((1 + 2))))").parse().to_string(), "(+ 1 2)")
    }
}