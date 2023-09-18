use crate::parser::{AstType, Ast, AstNode};

pub struct Transformer;

impl Transformer {
    pub fn transform(ast: Ast) -> String {
        match *ast {
            AstNode {
                ast_type: AstType::Program,
                ref params,
                ..
            } => Transformer::transform(params[0].clone()), // Assume there's only one expression in the program.
            AstNode {
                ast_type: AstType::CallExpression,
                ref name, // Include the name field
                ref params,
            } => {
                let operator = name.clone().unwrap_or("".to_string()); // Use the name field
                let args: Vec<String> = params.iter().map(|param| Transformer::transform(param.clone())).collect();
                format!("({} {})", operator, args.join(" "))
            },
            AstNode {
                ast_type: AstType::AddExpression, // 处理加法表达式
                ref params,
                ..
            } => {
                let args: Vec<String> = params.iter().map(|param| Transformer::transform(param.clone())).collect();
                format!("({})", args.join(" + "))
            },
            AstNode {
                ast_type: AstType::SubExpression, // 处理减法表达式
                ref params,
                ..
            } => {
                let args: Vec<String> = params.iter().map(|param| Transformer::transform(param.clone())).collect();
                format!("({})", args.join(" - "))
            },
            AstNode {
                ast_type: AstType::NumberLiteral,
                ref name, // Include the name field
                ..
            } => name.clone().unwrap_or("".to_string()), // Use the name field
            _ => "".to_string(), // Handle other cases if needed
        }
    }
}
