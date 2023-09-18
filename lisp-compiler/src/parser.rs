use std::fmt;

use crate::tokenizer::{TokenType, Token};

#[derive(Debug, Clone)]
pub enum AstType {
    Program,
    NumberLiteral,
    CallExpression,
    AddExpression, // 新增加法操作类型
    SubExpression, // 新增减法操作类型
    // 未知表达是类型
    Unknown,
    // 可以添加其他 AST 节点类型
}

#[derive(Debug)]
pub struct AstNode {
    pub ast_type: AstType,
    pub name: Option<String>,
    pub params: Vec<Ast>,
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
impl Clone for AstNode {
    fn clone(&self) -> Self {
        // 实现 AstNode 的克隆逻辑，具体根据你的数据结构来实现
        AstNode {
            ast_type: self.ast_type.clone(),
            name: self.name.clone(),
            params: self.params.clone(),
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
            // 处理加和减操作符
            let mut expression = match name_token.value.as_str() {
                "add" => Ast::new(AstNode::new(AstType::AddExpression, Some(name_token.value.clone()))),
                "sub" => Ast::new(AstNode::new(AstType::SubExpression, Some(name_token.value.clone()))),
                _ => Ast::new(AstNode::new(AstType::Unknown, Some(name_token.value.clone()))),
            };
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