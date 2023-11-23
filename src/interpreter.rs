use crate::parser::PostfixParser;
use crate::lexer::Lexer;
use crate::token::{Token, Token::*};
use std::collections::HashMap;
use std::ops::FnMut;

pub struct Interpreter<F> {
    oper_args_count_map: HashMap<Token, usize>,
    oper_eval_functor: F,
    lexer: Lexer,
    parser: PostfixParser
}

impl<F> Interpreter<F>
where F: FnMut(&mut Vec<u8>, &Token) -> Result<u8, String> {
    pub fn new(
        literal_token_map: HashMap<&'static str, Token>,
        oper_prioriry_map: HashMap<Token, i32>,
        oper_args_count_map: HashMap<Token, usize>,
        oper_eval_functor: F
    ) -> Self {
        Interpreter {
            oper_args_count_map,
            oper_eval_functor,
            lexer: Lexer::new(literal_token_map),
            parser: PostfixParser::new(oper_prioriry_map)
        }
    }

    pub fn evaluate(&mut self, expr: &str) -> Result<u8, String> {
        use Token::*;
        let infix = self.lexer.tokenize(expr)?;
        let postfix = self.parser.parse(infix)?;
        let mut eval_stack: Vec<u8> = Vec::new();
    
        for token in postfix.iter() {
            match self.get_oper_args_count(token) {
                Some(_) =>  {
                    let x = self.eval_op(&mut eval_stack, token)?;
                    eval_stack.push(x);
                }
                None => {
                    if let ConstVal(x) = token {
                        eval_stack.push(x.clone());
                    }
                    else {
                        return Err("Logic error: args_count() returned None for non-ConstVal token!".to_string());
                    }
                }
            }
        }
        
        if eval_stack.len() == 1 {
            Ok(eval_stack.pop().unwrap())
        } else {
            Err("Cannot evaluate expression!".to_string())
        }
    }
    
    fn eval_op(&mut self, stack: &mut Vec<u8>, op: &Token) -> Result<u8, String> {
        match self.get_oper_args_count(op) {
            Some(n) => {
                if stack.len() < n {
                    Err(format!("Too few operands for {op}!"))
                } else {
                    let x = (self.oper_eval_functor)(stack, op)?;
                    Ok(x)
                }
            }
            None => Err(format!("Logic error: {op} doesn't exist in ARGS_COUNT_MAP!"))
        }
    }

    fn get_oper_args_count(&self, op: &Token) -> Option<usize> {
        self.oper_args_count_map.get(op).copied()
    }
}

pub fn default_evaluate_operation(stack: &mut Vec<u8>, op: &Token) -> Result<u8, String> {
    match op {
        NotOp => {
            let x = stack.pop().unwrap() != 0;
            Ok(!x as u8)
        }
        _ => {
            let rhs = stack.pop().unwrap() != 0;
            let lhs = stack.pop().unwrap() != 0;
            match op {
                OrOp => Ok((lhs || rhs) as u8),
                XorOp => Ok((lhs ^ rhs) as u8),
                AndOp => Ok((lhs && rhs) as u8),
                _ => Err(format!("Logic error: unhandled operation {op}!"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::*;
    use lazy_static::*;

    lazy_static! {
        static ref LITERAL_TOKEN_MAP: HashMap<&'static str, Token> = HashMap::from([
            ("&", AndOp),
            ("|", OrOp),
            ("^", XorOp),
            ("~", NotOp),
            ("(", LeftBrace),
            (")", RightBrace) 
        ]);
    }

    lazy_static! {
        static ref OPER_PRIORITY_MAP: HashMap<Token, i32> = HashMap::from([
            (OrOp, 1),
            (XorOp, 1),
            (AndOp, 2),
            (NotOp, 3)
        ]);
    }

    lazy_static! {
        static ref ARGS_COUNT_MAP: HashMap<Token, usize> = HashMap::from([
            (OrOp, 2),
            (XorOp, 2),
            (AndOp, 2),
            (NotOp, 1)
        ]);
    }

    #[test]
    fn evaluate_expr_works() {
        let mut interpreter = Interpreter::new(
            LITERAL_TOKEN_MAP.clone(),
            OPER_PRIORITY_MAP.clone(),
            ARGS_COUNT_MAP.clone(),
            default_evaluate_operation
        );

        let x = interpreter.evaluate("~3f|ab &( c5^10 ) ");
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 0xc1_u8);

        assert!(interpreter.evaluate("~3f|ab &( c5^10 )) ").is_err());
        assert!(interpreter.evaluate("~3f|ab &) c5^10 ( ").is_err());
        //assert!(interpreter.evaluate("~3f|ab ~&( c5^10 ) ").is_err());
        assert!(interpreter.evaluate("~3f|ab &&( c5^10 ) ").is_err());
        assert!(interpreter.evaluate("~3f|ab bc &( c5^10 ) ").is_err());
    }

    #[test]
    fn default_evaluate_operation_fn_works() {
        let stack = vec![0x11_u8, 0x0f_u8];
        assert_eq!(
            default_evaluate_operation(&mut stack.clone(), &AndOp).unwrap(), 
            0x01_u8
        );
        assert_eq!(
            default_evaluate_operation(&mut stack.clone(), &NotOp).unwrap(), 
            0xf0_u8
        );
    }

    #[test]
    fn eval_op_method_works() {
        let mut stack = vec![0x11_u8, 0x0f_u8];

        let mut interpreter = Interpreter::new(
            LITERAL_TOKEN_MAP.clone(),
            OPER_PRIORITY_MAP.clone(),
            ARGS_COUNT_MAP.clone(),
            default_evaluate_operation
        );

        assert_eq!(
            interpreter.eval_op(&mut stack.clone(), &AndOp).unwrap(), 
            0x01_u8
        );
        assert_eq!(
            interpreter.eval_op(&mut stack.clone(), &NotOp).unwrap(), 
            0xf0_u8
        );
        stack.pop();
        assert!(interpreter.eval_op(&mut stack.clone(), &AndOp).is_err());
    }
}