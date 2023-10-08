use crate::lexer::{Tokens, Token, TokenType};
use super::{Node, NodeType};

pub struct Parser {
    tokens: Tokens,
    root: Node,
}

impl Parser {
    pub fn new(tokens: Tokens) -> Self {
        Self { tokens, root: Node::new(NodeType::ROOT, String::from("")) }
    }

    fn consume(&mut self) -> Token {
        self.tokens.remove(0)
    }

    fn add_to_root(&mut self, node: Node) {
        self.root.push(node);
    }

    pub fn parse(&mut self) -> Node {

        while !self.tokens.is_empty() {
            let token = self.consume();

            match token.kind {
                TokenType::SELECT => {
                    self.add_to_root(Node::new(NodeType::SELECT, String::from("")));
                },
                TokenType::FROM => {
                    self.add_to_root(Node::new(NodeType::FROM, String::from("")));
                },
                TokenType::FIELD => {
                    let node = Node::new(NodeType::FIELD, token.value);

                    if let Some(last_insert) = self.root.last_mut() {
                        last_insert.push(node);
                    }
                },
                TokenType::TABLE => {
                    let node = Node::new(NodeType::TABLE, token.value);

                    if let Some(last_insert) = self.root.last_mut() {
                        last_insert.push(node);
                    }
                }
                _ => {}
            }
        }

        self.root.clone()

    }
}