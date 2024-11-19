use super::{Atomic, EvalErr};

pub fn add_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Float(float1) => match rhs {
            Atomic::Integer(integer) => Ok(Atomic::Float(float1 + integer as f32)),
            Atomic::Float(float2) => Ok(Atomic::Float(float1 + float2)),
            _ => Err(EvalErr::CantAdd)
        },
        Atomic::Integer(integer1) => match rhs {
            Atomic::Integer(integer2) => Ok(Atomic::Integer(integer1 + integer2)),
            Atomic::Float(float) => Ok(Atomic::Integer(integer1 + float as i32)),
            _ => Err(EvalErr::CantAdd)
        },
        _ => Err(EvalErr::WrongType)
    }
}
pub fn sub_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Float(float1) => match rhs {
            Atomic::Integer(integer) => Ok(Atomic::Float(float1 - integer as f32)),
            Atomic::Float(float2) => Ok(Atomic::Float(float1 - float2)),
            _ => Err(EvalErr::CantAdd)
        },
        Atomic::Integer(integer1) => match rhs {
            Atomic::Integer(integer2) => Ok(Atomic::Integer(integer1 - integer2)),
            Atomic::Float(float) => Ok(Atomic::Integer(integer1 - float as i32)),
            _ => Err(EvalErr::CantAdd)
        },
        _ => Err(EvalErr::WrongType)
    }
}
pub fn mul_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Float(float1) => match rhs {
            Atomic::Integer(integer) => Ok(Atomic::Float(float1 * integer as f32)),
            Atomic::Float(float2) => Ok(Atomic::Float(float1 * float2)),
            _ => Err(EvalErr::CantAdd)
        },
        Atomic::Integer(integer1) => match rhs {
            Atomic::Integer(integer2) => Ok(Atomic::Integer(integer1 * integer2)),
            Atomic::Float(float) => Ok(Atomic::Integer(integer1 * float as i32)),
            _ => Err(EvalErr::CantAdd)
        },
        _ => Err(EvalErr::WrongType)
    }
}
pub fn div_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Float(float1) => match rhs {
            Atomic::Integer(integer) => Ok(Atomic::Float(float1 / integer as f32)),
            Atomic::Float(float2) => Ok(Atomic::Float(float1 / float2)),
            _ => Err(EvalErr::CantAdd)
        },
        Atomic::Integer(integer1) => match rhs {
            Atomic::Integer(integer2) => Ok(Atomic::Integer(integer1 / integer2)),
            Atomic::Float(float) => Ok(Atomic::Integer(integer1 / float as i32)),
            _ => Err(EvalErr::CantAdd)
        },
        _ => Err(EvalErr::WrongType)
    }
}
pub fn factor<'a>(lhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Integer(integer) => Ok(Atomic::Integer((1..=integer).product())),
        _ => Err(EvalErr::WrongType)
    }
}
pub fn and_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Boolean(boolean1) => match rhs {
            Atomic::Boolean(boolean2) => return Ok(Atomic::Boolean(boolean1 && boolean2)),
            _ => Err(EvalErr::WrongType)
        },
        _ => Err(EvalErr::WrongType)
    }
}
pub fn or_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Boolean(boolean1) => match rhs {
            Atomic::Boolean(boolean2) => return Ok(Atomic::Boolean(boolean1 || boolean2)),
            _ => Err(EvalErr::WrongType)
        },
        _ => Err(EvalErr::WrongType)
    }
}

pub fn less_than_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Float(float1) => match rhs {
            Atomic::Integer(integer) => Ok(Atomic::Boolean(float1 < integer as f32)),
            Atomic::Float(float2) => Ok(Atomic::Boolean(float1 < float2)),
            _ => Err(EvalErr::CantCompare)
        },
        Atomic::Integer(integer1) => match rhs {
            Atomic::Integer(integer2) => Ok(Atomic::Boolean(integer1 < integer2)),
            Atomic::Float(float) => Ok(Atomic::Boolean(integer1 < float as i32)),
            _ => Err(EvalErr::CantCompare)
        },
        _ => Err(EvalErr::WrongType)
    }
}
pub fn lessequal_than_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Float(float1) => match rhs {
            Atomic::Integer(integer) => Ok(Atomic::Boolean(float1 <= integer as f32)),
            Atomic::Float(float2) => Ok(Atomic::Boolean(float1 <= float2)),
            _ => Err(EvalErr::CantCompare)
        },
        Atomic::Integer(integer1) => match rhs {
            Atomic::Integer(integer2) => Ok(Atomic::Boolean(integer1 <= integer2)),
            Atomic::Float(float) => Ok(Atomic::Boolean(integer1 <= float as i32)),
            _ => Err(EvalErr::CantCompare)
        },
        _ => Err(EvalErr::WrongType)
    }
}
pub fn greater_than_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Float(float1) => match rhs {
            Atomic::Integer(integer) => Ok(Atomic::Boolean(float1 > integer as f32)),
            Atomic::Float(float2) => Ok(Atomic::Boolean(float1 > float2)),
            _ => Err(EvalErr::CantCompare)
        },
        Atomic::Integer(integer1) => match rhs {
            Atomic::Integer(integer2) => Ok(Atomic::Boolean(integer1 > integer2)),
            Atomic::Float(float) => Ok(Atomic::Boolean(integer1 > float as i32)),
            _ => Err(EvalErr::CantCompare)
        },
        _ => Err(EvalErr::WrongType)
    }
}
pub fn greaterequal_than_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Float(float1) => match rhs {
            Atomic::Integer(integer) => Ok(Atomic::Boolean(float1 >= integer as f32)),
            Atomic::Float(float2) => Ok(Atomic::Boolean(float1 >= float2)),
            _ => Err(EvalErr::CantCompare)
        },
        Atomic::Integer(integer1) => match rhs {
            Atomic::Integer(integer2) => Ok(Atomic::Boolean(integer1 >= integer2)),
            Atomic::Float(float) => Ok(Atomic::Boolean(integer1 >= float as i32)),
            _ => Err(EvalErr::CantCompare)
        },
        _ => Err(EvalErr::WrongType)
    }
}
pub fn equal_with_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Float(float1) => match rhs {
            Atomic::Integer(integer) => Ok(Atomic::Boolean(float1 == integer as f32)),
            Atomic::Float(float2) => Ok(Atomic::Boolean(float1 == float2)),
            _ => Err(EvalErr::CantCompare)
        },
        Atomic::Integer(integer1) => match rhs {
            Atomic::Integer(integer2) => Ok(Atomic::Boolean(integer1 == integer2)),
            Atomic::Float(float) => Ok(Atomic::Boolean(integer1 == float as i32)),
            _ => Err(EvalErr::CantCompare)
        },
        Atomic::Boolean(boolean1) => match rhs {
            Atomic::Boolean(boolean2) => Ok(Atomic::Boolean(boolean1 == boolean2)),
            _ => Err(EvalErr::CantCompare)
        },
        Atomic::String(string1) => match rhs {
            Atomic::String(string2) => Ok(Atomic::Boolean(string1 == string2)),
            _ => Err(EvalErr::CantCompare)
        }
        _ => Err(EvalErr::WrongType)
    }
}
pub fn notequal_with_together<'a>(lhs: Atomic, rhs: Atomic) -> Result<Atomic<'a>, EvalErr> {
    match lhs {
        Atomic::Float(float1) => match rhs {
            Atomic::Integer(integer) => Ok(Atomic::Boolean(float1 != integer as f32)),
            Atomic::Float(float2) => Ok(Atomic::Boolean(float1 != float2)),
            _ => Err(EvalErr::CantCompare)
        },
        Atomic::Integer(integer1) => match rhs {
            Atomic::Integer(integer2) => Ok(Atomic::Boolean(integer1 != integer2)),
            Atomic::Float(float) => Ok(Atomic::Boolean(integer1 != float as i32)),
            _ => Err(EvalErr::CantCompare)
        },
        Atomic::Boolean(boolean1) => match rhs {
            Atomic::Boolean(boolean2) => Ok(Atomic::Boolean(boolean1 != boolean2)),
            _ => Err(EvalErr::CantCompare)
        },
        Atomic::String(string1) => match rhs {
            Atomic::String(string2) => Ok(Atomic::Boolean(string1 != string2)),
            _ => Err(EvalErr::CantCompare)
        }
        _ => Err(EvalErr::WrongType)
    }
}