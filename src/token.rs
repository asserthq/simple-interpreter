use std::fmt;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Token {
    AndOp,
    OrOp,
    XorOp,
    NotOp,
    ConstVal(u8),
    LeftBrace,
    RightBrace
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        use Token::*;
        let str: String = match *self {
            AndOp => "AND".to_string(),
            OrOp => "OR".to_string(),
            XorOp => "XOR".to_string(),
            NotOp => "NOT".to_string(),
            ConstVal(val) => format!("{:x}", val),
            LeftBrace => "(".to_string(),
            RightBrace => ")".to_string()
        };
        write!(f, "{str}")?;
        Ok(())
    }
}
