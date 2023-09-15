use std::fmt;

use crate::tokenizer::{TokenType, Token};

#[derive(Debug)]
enum AstType {
    Program,
    NumberLiteral,
    CallExpression,
    // 可以添加其他 AST 节点类型
}

#[derive(Debug)]
pub struct AstNode {
    ast_type: AstType,
    name: Option<String>,
    params: Vec<Ast>,
}

pub type Ast = Box<AstNode>;

impl AstNode {
    fn new(ast_type: AstType, name: Option<String>) -> Self {
        AstNode {
            ast_type,
            name,
            params: vec![],
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn  new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn walk(&mut self) -> Ast {
        let token = &self.tokens[self.current];
        if token.token_type == TokenType::Number {
            self.current += 1;
            Ast::new(AstNode::new(AstType::NumberLiteral, Some(token.value.clone())))
        } else if token.token_type == TokenType::Paren && token.value == "(" {
            self.current += 1;
            let name_token = &self.tokens[self.current];
            let mut expression = Ast::new(AstNode::new(AstType::CallExpression, Some(name_token.value.clone())));
            self.current += 1;
            let mut token = &self.tokens[self.current];
            while token.value != ")" {
                expression.params.push(self.walk());
                token = &self.tokens[self.current];
            }
            self.current += 1;
            expression
        } else {
            panic!("Unknown token: {:?}", token);
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut ast = Ast::new(AstNode::new(AstType::Program, None));
        ast.params.push(self.walk());
        ast
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}