use crate::tokenizer::{tokenizer, Token};
use crate::parser::{ Parser, Ast};

/* 
  编译器
  1. Lexical Analysis
  2. Syntactic Analysis
  3. Semantic Analysis
  4. Code Generation
  5. Optimization
  return js code
*/
pub fn compiler (input: &str) -> Ast {
  let tokens: Vec<Token> = tokenizer(input);
  let mut parser = Parser::new(tokens);
  let ast = parser.parse();
  ast
}