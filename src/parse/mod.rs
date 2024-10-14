use std::{fmt::Display, iter::Peekable, vec};
mod display;
use crate::lexer::{self, Lexer, Token};
/// Needed because of lifetime complications.
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>
}

enum Operator {
    Minus,
    Plus,
    Star
}

enum Atomic<'a> {
    Integer(i32),
    String(&'a str),
    Float(f32),
    Nil
}
pub enum TokenTree<'a> {
    Atomic(Atomic<'a>),
    Expression(Operator, Vec<TokenTree<'a>>)
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
    fn expression(&mut self, min_bp: u8) -> TokenTree<'a> {
        
        let token = match self.lexer.next() {
            Some(token) => token,
            None => return TokenTree::Atomic(Atomic::Nil)
        };
        // Short for Left Hand Side.
        let mut lhs: TokenTree = match token {
            Token::Float(num) => TokenTree::Atomic(Atomic::Float(num)),
            Token::Integer(num) => TokenTree::Atomic(Atomic::Integer(num)),
            Token::String(string) => TokenTree::Atomic(Atomic::String(string)),
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
                t => panic!("Bad token operator side {:?}", t)
            };
            // Left hand binding power, and Right hand binding power.
            let (left_bp, right_bp) = infix_binding_power(&operator);
            if left_bp < min_bp {
                break;
            }
            // We can now iterate, because we know that this specific Token checks out,
            // so we can move down another layer and check if we can collapse.
            self.lexer.next();

            // Short of Right Hand Side
            let rhs = self.expression(right_bp);
            
            lhs = TokenTree::Expression(operator, vec![lhs, rhs]);
        };
    
        lhs
    }
    
    
}
fn infix_binding_power(operator: &Operator) -> (u8, u8) {
    match operator {
        Operator::Minus | Operator::Plus => (1, 2),
        Operator::Star => (3, 4)
    }
}

#[cfg(test)]
mod tests{
    use crate::{parse::Parser};

    

    #[test]
    fn expression_print() {
        let parser = Parser::new("1 + 2 * 3");
        let string = parser.parse();

        assert_eq!(string.to_string(), "(+ 1 (* 2 3))")
    }
}