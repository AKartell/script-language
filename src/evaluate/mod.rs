pub struct Evaluator {
    table: HashMap<String, Box<dyn Any>>,
}
use std::{any::Any, collections::HashMap, ops::Add};

use crate::parse::{self, *};
mod operations;
use operations::*;
#[derive(Debug)]
pub enum EvalErr {
    CantAdd,
    WrongType,
    CantCompare
}
pub enum ValueTypes {
    Float(f32),
    Integer(i32),
    String(String),
    Boolean(bool),
}

impl Evaluator {
    pub fn new() -> Evaluator{
        Evaluator {
            table: HashMap::new()
        }
    }
    fn eval_if(tree: TokenTree) {
        if let TokenTree::If {
            condition,
            positive,
            negative,
        } = tree
        {}
    }
    pub fn evaluate<'a>(&mut self, tree: TokenTree<'a>) -> Option<Atomic<'a>> {
        //println!("{}", tree);
        match tree {
            TokenTree::Atomic(val) => return Some(val),
            TokenTree::InfixExpression(op, arr) => {
                let lhs = self.evaluate(arr[0].clone())?;
                let rhs = self.evaluate(arr[1].clone())?;
                let op_result = match op {
                    Operator::Plus => add_together(lhs, rhs),
                    Operator::Minus => sub_together(lhs, rhs),
                    Operator::Star => mul_together(lhs, rhs),
                    Operator::Slash => div_together(lhs, rhs),

                    Operator::Less => less_than_together(lhs, rhs),
                    Operator::LessOrEqual => lessequal_than_together(lhs, rhs),
                    Operator::EqualEqual => equal_with_together(lhs, rhs),
                    Operator::NotEqual => notequal_with_together(lhs, rhs),
                    Operator::Greater => greater_than_together(lhs, rhs),
                    Operator::GreaterOrEqual => greaterequal_than_together(lhs, rhs),

                    Operator::And => and_together(lhs, rhs),
                    Operator::Or => or_together(lhs, rhs),

                    _ => unreachable!("This is all the possibilites.")
                };
                if let Ok(atomic) = op_result {
                    return Some(atomic);
                }
                else if let Err(err) = op_result {
                    panic!("{:?}", err);
                }
            },
            TokenTree::PostfixExpression(op, arr) => {
                let lhs = self.evaluate(arr[0].clone())?;
                let op_result = match op {
                    Operator::Bang => factor(lhs),
                    _ => unreachable!("This is all the possibilites.")
                };
                if let Ok(atomic) = op_result {
                    return Some(atomic);
                }
                else {
                    panic!("TODO better error handling!")
                }
            },
            TokenTree::PrefixExpression(op, arr) => {
                let rhs = self.evaluate(arr[0].clone())?;
                let op_result = match op {
                    Operator::Plus => Ok(rhs),
                    Operator::Minus => match rhs {
                        Atomic::Float(float) => Ok(Atomic::Float(-float)),
                        Atomic::Integer(integer) => Ok(Atomic::Integer(-integer)),
                        _ => Err(EvalErr::WrongType)
                    },
                    _ => unreachable!("This is all the possibilites")
                };
                if let Ok(atomic) = op_result {
                    return Some(atomic);
                }
                else {
                    panic!("TODO better error handling!")
                }
            },
            _ => todo!()
        };
        unreachable!("HOOOOOOOOL");
    }
}
