use crate::compiler::reports::Reporter;
use crate::compiler::tokenizer::{Token, TokenType};
use crate::utils::StringBuffer;
use std::num::ParseFloatError;

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    current_index: usize,
    in_string: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Lexeme {
    Identifier(String),
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    Not,
    And,
    Or,
    Comma,
    String(String),
}

impl Lexeme {
    pub fn display_type(&self) -> &str {
        match self {
            Lexeme::Identifier(_) => "Identifier",
            Lexeme::Number(_) => "Number",
            Lexeme::Plus => "Plus",
            Lexeme::Minus => "Minus",
            Lexeme::Multiply => "Multiply",
            Lexeme::Divide => "Divide",
            Lexeme::Modulus => "Modulus",
            Lexeme::LeftParenthesis => "Left Parenthesis",
            Lexeme::RightParenthesis => "Right Parenthesis",
            Lexeme::LeftBracket => "Left Bracket",
            Lexeme::RightBracket => "Right Bracket",
            Lexeme::Not => "Not",
            Lexeme::And => "And",
            Lexeme::Or => "Or",
            Lexeme::Comma => "Comma",
            Lexeme::String(_) => "String",
        }
    }
}

impl Lexer {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_index: 0,
            in_string: false,
        }
    }

    pub fn run(&mut self) -> Result<Vec<Lexeme>, LexerError> {
        type Error = LexerError;
        let mut lexemes: Vec<Lexeme> = Vec::new();
        let mut string_buffer = String::new();

        while self.current_index < self.tokens.len() {
            let token = &self.tokens[self.current_index];

            if self.in_string && token.kind != TokenType::QuotationMark {
                string_buffer.push_str(token.display_value().as_str());
                self.current_index += 1;
                continue;
            }

            let mut push_current_index_for = 1;

            let lexeme = match &token.kind {
                TokenType::Number => {
                    let mut number = token
                        .value
                        .as_ref()
                        .ok_or(Error::TokenMissingValue(token.clone()))?
                        .to_string();

                    if let Some(possible_dot) = self.peek_next()
                        && possible_dot.kind == TokenType::Dot
                        && let Some(fractional_part_token) = self.peek_next_by(2)
                        && fractional_part_token.kind == TokenType::Number
                        && let Some(fractional_part) = &fractional_part_token.value
                    {
                        number = format!("{}.{}", number, fractional_part);
                        push_current_index_for += 2;
                    }

                    let number: f64 = number
                        .parse()
                        .map_err(|e| Error::ParseFloatError(token.clone(), e))?;
                    Lexeme::Number(number)
                },
                TokenType::Identifier => {
                    let identifier = token
                        .value
                        .as_ref()
                        .ok_or(Error::TokenMissingValue(token.clone()))?
                        .to_string();
                    Lexeme::Identifier(identifier)
                },
                TokenType::Plus => Lexeme::Plus,
                TokenType::Minus => Lexeme::Minus,
                TokenType::Asterisk => Lexeme::Multiply,
                TokenType::Slash => Lexeme::Divide,
                TokenType::Percent => Lexeme::Modulus,
                TokenType::LeftParenthesis => Lexeme::LeftParenthesis,
                TokenType::RightParenthesis => Lexeme::RightParenthesis,
                TokenType::LeftBracket => Lexeme::LeftBracket,
                TokenType::RightBracket => Lexeme::RightBracket,
                TokenType::ExclamationMark => Lexeme::Not,
                TokenType::Ampersand => Lexeme::And,
                TokenType::Pipe => Lexeme::Or,
                TokenType::Comma => Lexeme::Comma,
                TokenType::QuotationMark => {
                    self.in_string = !self.in_string;
                    if !self.in_string {
                        let lexeme = Lexeme::String(string_buffer.clone());
                        string_buffer.clear();
                        lexeme
                    } else {
                        self.current_index += 1;
                        continue;
                    }
                },
                TokenType::Dot
                | TokenType::Space
                | TokenType::Tab
                | TokenType::NewLine
                | TokenType::Unknown => {
                    return Err(Error::NotExpectedToken(token.clone()));
                },
            };

            lexemes.push(lexeme);
            self.current_index += push_current_index_for;
        }

        Ok(lexemes)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current_index + 1)
    }

    fn peek_next_by(&self, by: usize) -> Option<&Token> {
        self.tokens.get(self.current_index + by)
    }
}

impl Reporter {
    pub fn lexemes_creation(
        &self, lexemes_result: &Result<Vec<Lexeme>, LexerError>,
    ) -> String {
        let mut buffer = StringBuffer::default();

        match lexemes_result {
            Ok(lexemes) => {
                let first_line =
                    format!("Lexer successfully produced {} lexemes.\n", lexemes.len());
                buffer.add_line(first_line);

                let lexemes_list = lexemes
                    .iter()
                    .map(|lexeme| format!("- {:?}", lexeme))
                    .collect::<Vec<String>>()
                    .join("\n");
                buffer.add_line(lexemes_list);
            },
            Err(error) => buffer.add_line(format!("Lexer error: {}", error)),
        }

        buffer.get()
    }
}

#[derive(Debug)]
pub enum LexerError {
    NotExpectedToken(Token),
    ParseFloatError(Token, ParseFloatError),
    TokenMissingValue(Token),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::NotExpectedToken(token) => format!(
                "Not expected token with kind \"{}\" [{}..{}]",
                token.kind,
                token.position.start,
                token.position.end - 1
            ),
            Self::ParseFloatError(token, error) => format!(
                "Failed to parse float [{}..{}]: {}",
                token.position.start,
                token.position.end - 1,
                error
            ),
            Self::TokenMissingValue(token) => format!(
                "Token with kind \"{}\" [{}..{}] is missing a value",
                token.kind,
                token.position.start,
                token.position.end - 1
            ),
        };

        write!(f, "Lexer error. {:?}", text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::tokenizer::Tokenizer;

    #[test]
    fn test_1() {
        let code = "a + b + c - 4.5";

        let tokens = Tokenizer::process(code);
        let lexer_result = Lexer::new(tokens).run();
        assert!(lexer_result.is_ok());

        let actual_lexemes = lexer_result.unwrap();
        let expected_lexemes = vec![
            Lexeme::Identifier("a".to_string()),
            Lexeme::Plus,
            Lexeme::Identifier("b".to_string()),
            Lexeme::Plus,
            Lexeme::Identifier("c".to_string()),
            Lexeme::Minus,
            Lexeme::Number(4.5),
        ];
        assert_eq!(actual_lexemes, expected_lexemes);
    }

    #[test]
    fn test_2() {
        let code = "a + sin((x - 12.34) / 2.0) + \"ddf.fd s 2.3\" + b";

        let tokens = Tokenizer::process(code);
        let lexer_result = Lexer::new(tokens).run();
        assert!(lexer_result.is_ok());

        let actual_lexemes = lexer_result.unwrap();
        let expected_lexemes = vec![
            Lexeme::Identifier("a".to_string()),
            Lexeme::Plus,
            Lexeme::Identifier("sin".to_string()),
            Lexeme::LeftParenthesis,
            Lexeme::LeftParenthesis,
            Lexeme::Identifier("x".to_string()),
            Lexeme::Minus,
            Lexeme::Number(12.34),
            Lexeme::RightParenthesis,
            Lexeme::Divide,
            Lexeme::Number(2.0),
            Lexeme::RightParenthesis,
            Lexeme::Plus,
            Lexeme::String("ddf.fd s 2.3".to_string()),
            Lexeme::Plus,
            Lexeme::Identifier("b".to_string()),
        ];
        assert_eq!(actual_lexemes, expected_lexemes);
    }
}
