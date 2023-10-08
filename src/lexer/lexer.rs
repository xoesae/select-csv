use super::{Token, TokenType, Chars, Tokens};

struct Looking {
    fields: bool,
    tables: bool,
}

pub struct Lexer {
    chars: Chars,
    current_index: usize,
    tokens: Tokens,
    buffer: String,
    looking: Looking,
}

impl Looking {
    pub fn new() -> Self {
        Self { fields: false, tables: false }
    }

    fn is_looking_fields(&self) -> bool {
        return self.fields.clone();
    }

    fn is_looking_tables(&self) -> bool {
        return self.tables.clone();
    }
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let chars: Vec<char> = source.chars().collect();
        Self {
            chars,
            current_index: 0,
            tokens: Vec::new(),
            buffer: String::from(""),
            looking: Looking::new(),
        }
    }

    fn get_char(&self, index: usize) -> char {
        return self.chars.get(index).unwrap().clone();
    }

    fn push_token(&mut self, kind: TokenType, value: String) {
        self.tokens.push(Token::new(kind, value));
    }

    fn next_index(&mut self) {
        self.current_index = self.current_index + 1;
    }

    fn match_select(&mut self) {
        self.push_token(TokenType::SELECT, String::from(""));
        self.looking.fields = true;
        self.buffer.clear();
        self.next_index();
    }

    fn match_from(&mut self) {
        self.push_token(TokenType::SELECT, String::from(""));
        
        self.buffer.clear();
        self.next_index();

        self.push_token(TokenType::FROM, String::from(""));
        self.looking.fields = false;
        self.looking.tables = true;
        self.buffer.clear();
    }

    pub fn tokenizer(&mut self) -> Tokens {

        while self.current_index < self.chars.len() {
            let current_char = self.get_char(self.current_index);
    
            if current_char.is_alphanumeric() {
                self.buffer.push(current_char);
            }
    
            match self.buffer.to_lowercase().as_str() {
                "select" => {
                    self.match_select();
                    continue;
                },
                "from" => { 
                    self.match_from();
                },
                _ => {},
            }
    
            if self.chars.len() > self.current_index + 1 {
                let next_char = self.get_char(self.current_index + 1);
    
                if next_char == ';' || next_char == ',' || (next_char.is_whitespace() && (! self.buffer.is_empty())) {
                    let mut kind: TokenType = TokenType::IDENTIFIER;
                    
                    if self.looking.is_looking_fields() {
                        kind = TokenType::FIELD;
                    }
    
                    if self.looking.is_looking_tables() {
                        kind = TokenType::TABLE;
                    }

                    self.push_token(kind, self.buffer.clone());
                    self.buffer.clear();
                }
            }

            if current_char == ';' || current_char == ',' {
                let mut kind: TokenType = TokenType::IDENTIFIER;
                
                if current_char == ';' {
                    kind = TokenType::SEMICOLON;
                }

                if current_char == ',' {
                    kind = TokenType::COMMA;
                }

                self.push_token(kind, String::from(""));
                self.buffer.pop();
            }
    
            self.next_index();
        }

        self.tokens.clone()
    }

    // pub fn print_tokens(&self) {
    //     println!("{:?}", self.tokens);
    // }
}