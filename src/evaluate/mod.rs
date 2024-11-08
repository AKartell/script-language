pub struct Evaluator {
    table: HashMap<String, Box<dyn Any>>
}
use std::{any::Any, collections::HashMap, ops::Add};

use crate::parse::*;

struct Object<T> {
    val: T
}

impl Evaluator {
    fn eval_if(tree: TokenTree){
        if let TokenTree::If { condition, positive, negative } = tree {
            
        }
    }
    pub fn evaluate(&mut self, tree: TokenTree) -> Option<Box<dyn Any>>{
        match TokenTree {
            TokenTree::Atomic(val) => match val {
                Atomic::Float(float) => return Some(Box::new(float)),
                Atomic::Integer(integer) => return Some(Box::new(integer)),
                Atomic::String(string) => return Some(Box::new(string)),
                Atomic::Identifier(ident) => todo!(),
                Atomic::Nil => None,
                
            },
            TokenTree::InfixExpression(op, arr) => {
                let lhs = self.evaluate(arr[0].clone())?;
                let rhs = self.evaluate(arr[1].clone())?;
                match op {
                    Operator::Minus | Operator::Plus | Operator::Star => {
                        let operation = match op {
                            Operator::Minus => {
                                |x|     
                            }
                        }
                        let (left, right) = if let (Some(l), Some(r)) = (lhs.downcast_ref::<f32>(), rhs.downcast_ref::<f32>()) {
                            (l, r)
                        }
                        else if let (Some(l), Some(r)) = (lhs.downcast_ref::<i32>(), rhs.downcast_ref::<i32>()) {
                            (l, r)
                        }
                        else {
                            panic!("Cannot add values of different or incompatible types.")
                        }
                        return Some(Box::new(left + right));
                    },
                    Operator::Assign => {
                        todo!()
                    }
                }
                
            }
            
        };
        unreachable!("HOOOOOOOOL");
    }
}