use super::*;

impl<'a> Display for TokenTree<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenTree::Atomic(atom) => write!(f, "{}", atom),
            TokenTree::Expression(op, token_tree) => {
                write!(f, "({}", op)?;
                for tree in token_tree {
                    write!(f, " {}", tree)?;
                }
                write!(f, ")")
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
        }
    }
}
impl<'a> Display for Atomic<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atomic::Float(num) => write!(f, "{}", num),
            Atomic::String(string) => write!(f, "{}", string),
            Atomic::Integer(num) => write!(f, "{}", num),
            Atomic::Nil => write!(f, "Nil")
        }
    }
}