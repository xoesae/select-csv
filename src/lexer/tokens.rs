use std::fmt;

pub type Tokens = Vec<Token>;
pub type Chars = Vec<char>;

#[derive(Debug, Clone)]
pub enum TokenType {
    SELECT,
    COMMA,
    SEMICOLON,
    FROM,
    IDENTIFIER,
    FIELD,
    TABLE,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

// impl TokenType {
//     pub fn from(string: &str) -> Self {
//         match string {
//             "select" => TokenType::SELECT,
//             "from" => TokenType::FROM,    
//             "field" => TokenType::FIELD,
//             "table" => TokenType::TABLE,
//             "," => TokenType::COMMA,
//             ";" => TokenType::SEMICOLON,
//             _ => TokenType::IDENTIFIER,
//         }
//     }
// }

impl Token {
    pub fn new(kind: TokenType, value: String) -> Self {
        Self { kind, value }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::SELECT => write!(f, "SELECT"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::FROM => write!(f, "FROM"),
            TokenType::SEMICOLON => write!(f, "SEMICOLON"),
            TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
            TokenType::FIELD => write!(f, "FIELD"),
            TokenType::TABLE => write!(f, "TABLE"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.kind.to_string(), self.value)
    }
}