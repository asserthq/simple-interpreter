use std::io;
use std::io::BufRead;
use std::collections::HashMap;

use simple_interpreter::Token::*;
use simple_interpreter::{Interpreter, default_evaluate_operation};

fn main() {
    let literal_token_map = HashMap::from([
        ("&", AndOp),
        ("|", OrOp),
        ("^", XorOp),
        ("~", NotOp),
        ("(", LeftBrace),
        (")", RightBrace) 
    ]);

    let oper_prioriry_map = HashMap::from([
        (OrOp, 1),
        (XorOp, 1),
        (AndOp, 2),
        (NotOp, 3)
    ]);

    let oper_args_count_map = HashMap::from([
        (OrOp, 2),
        (XorOp, 2),
        (AndOp, 2),
        (NotOp, 1)
    ]);

    let mut interpreter = Interpreter::new(
        literal_token_map,
        oper_prioriry_map,
        oper_args_count_map,
        default_evaluate_operation
    );

    let mut lines = io::stdin().lock().lines();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        if last_input.len() == 0 {
            break;
        }

        match interpreter.evaluate(&(last_input.trim())) {
            Ok(val) => println!("> {val}"),
            Err(str) => println!("! {str}")
        }
    }
}
