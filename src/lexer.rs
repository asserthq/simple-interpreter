use crate::token::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Lexer {
    literal_token_map: HashMap<&'static str, Token>
}

impl Lexer {
    pub fn new(literal_token_map: HashMap<&'static str, Token>) -> Self {
        Lexer {
            literal_token_map
        }
    }

    pub fn tokenize(&self, expr: &str) -> Result<Vec<Token>, &str> {
        let splitted = Self::split(expr);
        let mut tokens: Vec<Token> = Vec::with_capacity(splitted.len());
        for token in splitted.iter() {
            match self.try_token_from(token.as_str()) {
                Ok(t) => tokens.push(t),
                Err(str) => return Err(str)
            }
        }
        Ok(tokens)
    }
    
    fn split(expr: &str) -> Vec<String> {
        let mut splitted: Vec<String> = Vec::new();
        let mut reading_num = false;
        for ch in expr.chars() {
            match ch {
                ' ' => {
                    reading_num = false;
                }
                '0'..='9' | 'a'..='f' => {
                    if !reading_num {
                        splitted.push(ch.to_string());
                        reading_num = true;
                    } else {
                        splitted.last_mut().unwrap().push(ch);
                    }
                }
                _ => {
                    reading_num = false;
                    splitted.push(ch.to_string());
                }
            }
        }
        splitted
    }

    fn try_token_from(&self, token_str: &str) -> Result<Token, &str> {
        match self.literal_token_map.get(token_str) {
            Some(token) => Ok(token.clone()),
            None => {
                match u8::from_str_radix(token_str, 16) {
                    Ok(val) => Ok(Token::ConstVal(val)),
                    Err(_) => Err("Incorrect token found!")
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use crate::token::Token::*;
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

    #[test]
    fn tokenize_from_strings_vec_works() {
        let lexer = Lexer::new(LITERAL_TOKEN_MAP.clone());

        assert_eq!(
            lexer.tokenize("ab &(c5 ^10 ) ").unwrap(),
            vec![ConstVal(0xAB), AndOp, LeftBrace, ConstVal(0xC5), XorOp, ConstVal(0x10), RightBrace]
        );
        assert!(lexer.tokenize("a b").is_ok());
        assert!(lexer.tokenize("g5").is_err());
        assert!(lexer.tokenize("ff,33").is_err());
    }

    #[test]
    fn split_on_strings_works() {
        assert_eq!(
            Lexer::split("~3f|ab &( c5^10 ) "), 
            vec!["~", "3f", "|", "ab", "&", "(", "c5", "^", "10", ")"]
        );
    }

    #[test]
    fn try_token_from_str_works() {
        let lexer = Lexer::new(LITERAL_TOKEN_MAP.clone());

        assert_eq!(lexer.try_token_from("&").unwrap(), Token::AndOp);
        assert_eq!(lexer.try_token_from("~").unwrap(), Token::NotOp);
        assert_eq!(lexer.try_token_from("ff").unwrap(), Token::ConstVal(0xff));
        assert!(lexer.try_token_from("m55").is_err());
    }
}