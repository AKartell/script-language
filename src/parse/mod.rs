use std::{fmt::Display, iter::Peekable, vec};
mod display;
use crate::lexer::{self, Check, Lexer, LexerError, Token, TokenType};
/// Needed because of lifetime complications.
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}
#[derive(Clone, Copy)]

pub enum Operator {
    Minus,
    Plus,
    Star,
    Slash,
    Bang,
    Assign,

    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    EqualEqual,
    NotEqual,

    And,
    Or,

    Let,
}
#[derive(Clone, Copy, Debug)]

pub enum Atomic<'a> {
    Integer(i32),
    String(&'a str),
    Float(f32),
    Identifier(&'a str),
    Boolean(bool),
    Nil,
}
#[derive(Clone)]
pub enum TokenTree<'a> {
    Atomic(Atomic<'a>),
    // [TokenTree; 2]????
    InfixExpression(Operator, Vec<TokenTree<'a>>),
    PostfixExpression(Operator, Vec<TokenTree<'a>>),
    PrefixExpression(Operator, Vec<TokenTree<'a>>),
    If {
        // We have to store them on the Heap,
        // because otherwise it has infinite size.
        // Something that we cannot store on the Stack.
        condition: Box<TokenTree<'a>>,
        positive: Box<TokenTree<'a>>,
        negative: Option<Box<TokenTree<'a>>>,
    },
    While {
        condition: Box<TokenTree<'a>>,
        body: Box<TokenTree<'a>>,
    },
}
impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        Parser {
            lexer: Lexer::new(input).peekable(),
        }
    }
    pub fn parse(mut self) -> TokenTree<'a> {
        self.expression(0)
    }
    fn expected_token_parse(&mut self, token: TokenType) -> TokenTree<'a> {
        if self.lexer.expect(token) {
            TokenTree::Atomic(Atomic::Nil)
        } else {
            let val = self.expression(0);
            self.lexer.panic_expect(token);
            val
        }
    }
    // 1 + (2 * 2 * 2) + 2
    // 1 + 2 * (-2)
    //    +
    // 1     *
    //     2    -
    //          2
    fn syntax(&mut self, left_side: TokenType, right_side: TokenType) -> TokenTree<'a> {
        self.lexer.panic_expect(left_side);
        self.expected_token_parse(right_side)
    }
    fn expression(&mut self, min_bp: u8) -> TokenTree<'a> {
        let token = match self.lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(error)) => return TokenTree::Atomic(Atomic::Nil),
            None => return TokenTree::Atomic(Atomic::Nil),
        };
        //println!("{:?}", token.get_type());
        // Short for Left Hand Side.
        let mut lhs: TokenTree = match token.token_type {
            TokenType::Float(num) => TokenTree::Atomic(Atomic::Float(num)),
            TokenType::Integer(num) => TokenTree::Atomic(Atomic::Integer(num)),
            TokenType::String(string) => TokenTree::Atomic(Atomic::String(string)),
            TokenType::Identifier(name) => TokenTree::Atomic(Atomic::Identifier(name)),
            TokenType::True => TokenTree::Atomic(Atomic::Boolean(true)),
            TokenType::False => TokenTree::Atomic(Atomic::Boolean(false)),
            TokenType::Plus | TokenType::Minus => {
                let operator = match token.get_type() {
                    TokenType::Plus => Operator::Plus,
                    TokenType::Minus => Operator::Minus,
                    _ => unreachable!("This is impossible to reach."),
                };

                let ((), right_bp) = prefix_binding_power(&operator);
                let rhs = self.expression(right_bp);
                TokenTree::PrefixExpression(operator, vec![rhs])
            }
            TokenType::ParenLeft => {
                let lhs = self.expected_token_parse(TokenType::ParenRight);
                lhs
            }
            TokenType::Let => {
                let lhs = self.expression(0);
                TokenTree::PrefixExpression(Operator::Let, vec![lhs])
            }
            TokenType::If => {
                // We want to see a condition, and after that a block. Maybe an else, and another block.
                let condition = self.expression(0);

                let positive = self.syntax(TokenType::BraceLeft, TokenType::BraceRight);

                let negative = if self.lexer.expect(TokenType::Else) {
                    let val = self.syntax(TokenType::BraceLeft, TokenType::BraceRight);
                    Some(Box::new(val))
                } else {
                    None
                };
                TokenTree::If {
                    condition: Box::new(condition),
                    positive: Box::new(positive),
                    negative: negative,
                }
            }
            TokenType::While => {
                let condition = self.expression(0);
                let body = self.syntax(TokenType::BraceLeft, TokenType::BraceRight);

                TokenTree::While {
                    condition: Box::new(condition),
                    body: Box::new(body),
                }
            }
            t => panic!("Bad token on left hand side. {:?}", t),
        };

        loop {
            // We peek, because it is recursive.
            // If this fails, we go back to the parent, but that parent is still in a loop.
            // 4D chess.
            let Some(Ok(token)) = self.lexer.peek() else {
                break;
            };

            let operator = match token.get_type() {
                TokenType::Plus => Operator::Plus,
                TokenType::Minus => Operator::Minus,
                TokenType::Star => Operator::Star,
                TokenType::Bang => Operator::Bang,
                TokenType::Slash => Operator::Slash,

                TokenType::BangEqual => Operator::NotEqual,
                TokenType::EqualEqual => Operator::EqualEqual,
                TokenType::Great => Operator::Greater,
                TokenType::GreatEqual => Operator::GreaterOrEqual,
                TokenType::Less => Operator::Less,
                TokenType::LessEqual => Operator::LessOrEqual,
                TokenType::And => Operator::And,
                TokenType::Or => Operator::Or,

                TokenType::Equal => {
                    if let TokenTree::Atomic(Atomic::Identifier(_)) = lhs {
                        Operator::Assign
                    } else {
                        panic!("Left hand side is not an identifier!")
                    }
                }
                t => {
                    //println!("Unidetified operator: {:?}", t);
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

                lhs = TokenTree::PostfixExpression(operator, vec![lhs]);
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

                lhs = TokenTree::InfixExpression(operator, vec![lhs, rhs]);
                continue;
            }
            break;
        }

        lhs
    }
}
/// Operator has two atomic neighbours.
fn infix_binding_power(operator: &Operator) -> Option<(u8, u8)> {
    match operator {
        Operator::Minus | Operator::Plus => Some((2, 3)),
        Operator::Assign => Some((1, 2)),
        Operator::Star | Operator::Slash => Some((4, 5)),
        Operator::Less | Operator::LessOrEqual | Operator::Greater | Operator::GreaterOrEqual | Operator::NotEqual | Operator::EqualEqual => Some((1 , 2)),
        Operator::And => Some((1, 2)),
        Operator::Or => Some((0, 1)),
        _ => None,
    }
}
/// Operator only has one atomic neighbour to its right.
fn prefix_binding_power(operator: &Operator) -> ((), u8) {
    match operator {
        Operator::Plus | Operator::Minus => ((), 5),
        _ => panic!("Bad prefix operator!"),
    }
}
/// Operator only has one atomic neighbour to its left.
fn postfix_binding_power(operator: &Operator) -> Option<(u8, ())> {
    match operator {
        Operator::Bang => Some((7, ())),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
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
