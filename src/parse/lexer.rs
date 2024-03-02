use uuid::Uuid;

#[derive(Debug, PartialEq, Default, Clone)]
pub enum TokenType {
    String,
    TagPlus,
    TagMinus,
    FilterStatus,
    Int,
    Uuid,
    #[default]
    Eof,
    LeftParenthesis,
    RightParenthesis,
    OperatorAnd,
    OperatorOr,
    OperatorXor,
}
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_str = match self {
            TokenType::String => "String",
            TokenType::TagPlus => "TagPlus",
            TokenType::TagMinus => "TagMinus",
            TokenType::FilterStatus => "FilterStatus",
            TokenType::Int => "Int",
            TokenType::Uuid => "Uuid",
            TokenType::Eof => "Eof",
            TokenType::LeftParenthesis => "LeftParenthesis",
            TokenType::RightParenthesis => "RightParenthesis",
            TokenType::OperatorAnd => "OperatorAnd",
            TokenType::OperatorOr => "OperatorOr",
            TokenType::OperatorXor => "OperatorXor",
        };
        write!(f, "{}", token_str)
    }
}

#[path = "lexer_test.rs"]
mod lexer_test;

#[derive(Debug, Default, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

fn is_segment_character(ch: &char) -> bool {
    ch.is_whitespace() || *ch == '(' || *ch == ')' || *ch == '\0'
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input.chars().nth(self.read_position).unwrap());
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // Helper method to check if the current character is a digit
    fn is_digit(&self) -> bool {
        matches!(self.ch, Some(ch) if ch.is_digit(10))
    }

    // Method to read an integer
    fn read_int(&mut self) -> String {
        let starting_pos = self.position;
        while self.is_digit() {
            self.read_char();
        }
        self.input[starting_pos..self.position].to_string()
    }

    // Method to check if the current substring is a valid UUID
    fn is_uuid(&self) -> bool {
        let end_pos = self.position + 36; // UUID length is 36
        if end_pos > self.input.len() {
            return false;
        }

        Uuid::parse_str(&self.input[self.position..end_pos]).is_ok()
    }

    // Method to check and read a UUID
    fn read_uuid(&mut self) -> Result<String, String> {
        let end_pos = self.position + 36;
        if end_pos > self.input.len() {
            return Err("Not a valid UUID string".to_string());
        }

        let uuid_str = &self.input[self.position..end_pos];
        if Uuid::parse_str(uuid_str).is_ok() {
            self.position = end_pos;
            self.read_position = end_pos;
            self.ch = self.input.chars().nth(end_pos);
            Ok(uuid_str.to_string())
        } else {
            Err("Not a valid UUID string".to_string())
        }
    }

    fn is_tag_prefix(&self) -> bool {
        if !matches!(self.ch, Some('+') | Some('-')) {
            return false;
        }
        if self.read_position >= self.input.len() {
            return false;
        }
        if let Some(ch) = self.input.chars().nth(self.read_position) {
            return ch.is_alphanumeric() || ch == '_';
        }
        false
    }

    // Method to read a tag
    fn read_tag(&mut self) -> String {
        assert!(self.is_tag_prefix());
        let starting_pos = self.position;
        self.read_char(); // Skip tag prefix

        // Check for word characters after tag prefix
        while self.is_word_character() {
            self.read_char();
        }

        self.input[starting_pos..self.position].to_string()
    }

    // Helper method to check if the current character is part of a word
    fn is_word_character(&self) -> bool {
        matches!(self.ch, Some(ch) if ch.is_alphanumeric() || ch == '_')
    }

    // Method to match a specific keyword
    fn match_keyword(&self, word: &str) -> bool {
        self.input[self.position..].starts_with(word)
    }

    // Method to read the next word
    fn read_next_word(&mut self) -> String {
        let starting_pos = self.position;
        while let Some(ch) = self.ch {
            if is_segment_character(&ch) {
                break;
            }
            self.read_char();
        }
        self.input[starting_pos..self.position].to_string()
    }

    fn read_word(&mut self, word: &str) -> String {
        if !self.match_keyword(word) {
            panic!("error in read_word: Trying to read a word that can't be found");
        }

        let start_pos = self.position;
        for _ in 0..word.len() {
            self.read_char();
        }

        self.input[start_pos..self.position].to_string()
    }

    // Method to tokenize the next part of the input
    pub fn next_token(&mut self) -> Result<Token, String> {
        // Skip whitespace
        while matches!(self.ch, Some(ch) if ch.is_whitespace()) {
            self.read_char();
        }

        // Define the token based on the current character
        let token = match self.ch {
            None => Token {
                token_type: TokenType::Eof,
                literal: String::new(),
            },
            Some(ch) => match ch {
                _ if self.is_uuid() => Token {
                    literal: self.read_uuid()?,
                    token_type: TokenType::Uuid,
                },
                _ if self.is_digit() => Token {
                    token_type: TokenType::Int,
                    literal: self.read_int(),
                },
                _ if self.is_tag_prefix() => {
                    let tag_prefix = ch;
                    let tag_value = self.read_tag();
                    match tag_prefix {
                        '+' => Token {
                            token_type: TokenType::TagPlus,
                            literal: tag_value,
                        },
                        '-' => Token {
                            token_type: TokenType::TagMinus,
                            literal: tag_value,
                        },
                        _ => return Err("Error in parsing tag token".to_string()),
                    }
                }
                _ if self.match_keyword("and") => {
                    let mut literal_value = self.read_word("and");

                    let token_type = match self.ch {
                        Some(c) if !is_segment_character(&c) => {
                            literal_value += &self.read_next_word();
                            TokenType::String
                        }
                        _ => TokenType::OperatorAnd,
                    };

                    Token {
                        literal: literal_value,
                        token_type,
                    }
                }
                _ if self.match_keyword("or") => {
                    let mut literal_value = self.read_word("or");

                    let token_type = match self.ch {
                        Some(c) if !is_segment_character(&c) => {
                            literal_value += &self.read_next_word();
                            TokenType::String
                        }
                        _ => TokenType::OperatorOr,
                    };

                    Token {
                        literal: literal_value,
                        token_type,
                    }
                }
                _ if self.match_keyword("xor") => {
                    let mut literal_value = self.read_word("xor");

                    let token_type = match self.ch {
                        Some(c) if !is_segment_character(&c) => {
                            literal_value += &self.read_next_word();
                            TokenType::String
                        }
                        _ => TokenType::OperatorXor,
                    };

                    Token {
                        literal: literal_value,
                        token_type,
                    }
                }
                _ if self.match_keyword("status:") => Token {
                    literal: self.read_word("status:"),
                    token_type: TokenType::FilterStatus,
                },
                _ if ch == ')' => {
                    self.read_char();
                    Token {
                        literal: ")".to_string(),
                        token_type: TokenType::RightParenthesis,
                    }
                }
                _ if ch == '(' => {
                    self.read_char();
                    Token {
                        literal: "(".to_string(),
                        token_type: TokenType::LeftParenthesis,
                    }
                }
                _ => Token {
                    literal: self.read_next_word(),
                    token_type: TokenType::String,
                },
            },
        };

        // Read the next character and return the token
        Ok(token)
    }

    // Other helper methods...
}
