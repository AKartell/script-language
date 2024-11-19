use super::*;

impl<'a> Display for TokenTree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenTree::Atomic(atom) => write!(f, "{}", atom),
            TokenTree::InfixExpression(op, token_tree)
            | TokenTree::PostfixExpression(op, token_tree)
            | TokenTree::PrefixExpression(op, token_tree) => {
                write!(f, "({}", op)?;
                for tree in token_tree {
                    write!(f, " {}", tree)?;
                }
                write!(f, ")")
            }
            TokenTree::If {
                condition,
                positive,
                negative,
            } => {
                write!(f, "(if")?;
                write!(f, " {}", condition)?;
                write!(f, " : {}", positive)?;

                if let Some(token_tree) = negative {
                    write!(f, " ? {}", token_tree)?;
                }
                write!(f, ")")
            }
            TokenTree::While { condition, body } => {
                write!(f, "(while")?;
                write!(f, " {}", condition)?;
                write!(f, " : {}", body)
            }
        }
    }
}
impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Minus => write!(f, "-"),
            Operator::Plus => write!(f, "+"),
            Operator::Star => write!(f, "*"),
            Operator::Bang => write!(f, "!"),
            Operator::Let => write!(f, "let"),
            Operator::Assign => write!(f, "="),
            Operator::Slash => write!(f, "/"),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterOrEqual => write!(f, ">="),
            Operator::Less => write!(f, "<"),
            Operator::LessOrEqual => write!(f, "<="),
            Operator::EqualEqual => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::And => write!(f, "&&"),
            Operator::Or => write!(f, "||"),
            
        }
    }
}
impl<'a> Display for Atomic<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atomic::Float(num) => write!(f, "{}", num),
            Atomic::String(string) => write!(f, "{}", string),
            Atomic::Integer(num) => write!(f, "{}", num),
            Atomic::Nil => write!(f, "()"),
            Atomic::Identifier(name) => write!(f, "{}", name),
            Atomic::Boolean(boolean) => write!(f, "{}", boolean)
        }
    }
}
