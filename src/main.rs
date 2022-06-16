pub mod ast;
pub mod environment;
pub mod eval;
pub mod tests;
use ast::*;
use environment::*;
use eval::*;
use std::cell::RefCell;
use std::rc::Rc;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

pub fn std_print(vals: Vec<Value>) -> Result<Value, String> {
    println!("{:?}", &vals);

    Ok(vals[0].clone())
}

fn main() {
    let env = Environment::default();
    // env.define("println".to_string(), Value::BuiltinFunction(std_print))
    //     .unwrap();
    let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
    let input = std::fs::read_to_string("fib.mrt").expect("Cannot read source file");
    let source = parser::ProgParser::new().parse(&input).unwrap();
    let res = interpreter.eval(&source).unwrap();
    println!("{}", res);
}
