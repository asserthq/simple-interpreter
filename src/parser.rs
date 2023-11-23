use crate::token::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct PostfixParser {
    oper_prioriry_map: HashMap<Token, i32>,
}

impl PostfixParser {
    pub fn new(oper_prioriry_map: HashMap<Token, i32>) -> Self {
        PostfixParser {
            oper_prioriry_map
        }
    }

    pub fn parse(&self, tokens_infix: Vec<Token>) -> Result<Vec<Token>, &str> {
        let mut tokens_postfix: Vec<Token> = Vec::new();
        let mut temp_stack: Vec<Token> = Vec::new();
        for token in tokens_infix.iter().cloned() {
            match token {
                Token::LeftBrace => {
                    temp_stack.push(token);
                }
                Token::RightBrace => {
                    loop {
                        match temp_stack.last() {
                            Some(Token::LeftBrace) => {
                                temp_stack.pop();
                                break;
                            }
                            Some(t) => {
                                tokens_postfix.push(t.clone());
                                temp_stack.pop();
                            } 
                            None => {
                                return Err("Extra closing brace!");
                            }
                        }
                    }
                }
                Token::ConstVal(_) => {
                    tokens_postfix.push(token);
                }
                op => {
                    while let Some(top) = temp_stack.last() {
                        let prior_of_top = self.get_oper_priority(top);
                        let prior_of_cur = self.get_oper_priority(&op);
                        if prior_of_top.is_some() && prior_of_top >= prior_of_cur {
                            tokens_postfix.push(top.clone());
                            temp_stack.pop();
                        }
                        else {
                            break;
                        }
                    }
                    temp_stack.push(op);
                }
            }
        }
        while let Some(top) = temp_stack.last().cloned() {
            tokens_postfix.push(top);
            temp_stack.pop();
        }
        Ok(tokens_postfix)
    }
    
    fn get_oper_priority(&self, op: &Token) -> Option<i32> {
        self.oper_prioriry_map.get(op).copied()
    }
}



#[cfg(test)]
mod tests {
    use crate::parser::*;
    use crate::token::Token::*;
    use crate::lexer::Lexer;
    use lazy_static::*;

    lazy_static! {
        static ref LEXER: Lexer = Lexer::new(HashMap::from([
            ("&", AndOp),
            ("|", OrOp),
            ("^", XorOp),
            ("~", NotOp),
            ("(", LeftBrace),
            (")", RightBrace) 
        ]));
    }

    lazy_static! {
        static ref OPER_PRIORITY_MAP: HashMap<Token, i32> = HashMap::from([
            (OrOp, 1),
            (XorOp, 1),
            (AndOp, 2),
            (NotOp, 3)
        ]);
    }

    #[test]
    fn parse_to_postfix_works() {
        let tokens_infix = LEXER.tokenize("~3f| ab &( 4d | 05)");
        assert!(tokens_infix.is_ok());
        let parser = PostfixParser::new(OPER_PRIORITY_MAP.clone());
        assert_eq!(
            parser.parse(tokens_infix.unwrap()), 
            LEXER.tokenize("3f ~ ab 4d 05 | & |")
        );
    }
    
    #[test]
    fn get_oper_priority_method_works() {
        let parser = PostfixParser::new(OPER_PRIORITY_MAP.clone());
        assert_eq!(parser.get_oper_priority(&AndOp), Some(2));
        assert_eq!(parser.get_oper_priority(&OrOp), Some(1));
        assert_eq!(parser.get_oper_priority(&ConstVal(0xff)), None);
    }
}