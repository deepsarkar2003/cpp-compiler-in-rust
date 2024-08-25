// src/parser.rs
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: None,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Result<AST, String> {
        let mut ast = AST::new();
        while self.current_token.is_some() {
            match self.current_token {
                Some(Token::Identifier(_)) => {
                    let node = self.parse_expression()?;
                    ast.add_node(node);
                }
                // Handle more token types
                _ => return Err("Unexpected token".to_string()),
            }
            self.advance();
        }
        Ok(ast)
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        // Parse an expression and return the corresponding AST node
        Ok(ASTNode::Expression)
    }
}

pub struct AST {
    nodes: Vec<ASTNode>,
}

impl AST {
    pub fn new() -> Self {
        AST { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: ASTNode) {
        self.nodes.push(node);
    }

    // Public method to access nodes
    pub fn get_nodes(&self) -> &Vec<ASTNode> {
        &self.nodes
    }
}

pub enum ASTNode {
    Expression,
    // Add more node types as needed
}
