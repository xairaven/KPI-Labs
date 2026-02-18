use crate::compiler::reports::Reporter;
use crate::compiler::tokenizer::{Token, TokenType};
use crate::utils::{StringBuffer, StringExtension};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct SyntaxAnalyzer {
    tokens: Vec<Token>,
    current_index: usize,

    status: Status,
    errors: Vec<SyntaxError>,

    brackets_stack: VecDeque<Token>,
    parentheses_stack: VecDeque<Token>,
    quotation_marks_stack: VecDeque<Token>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SyntaxError {
    pub token: Token,
    pub kind: SyntaxErrorKind,
}

macro_rules! syntax_error {
    ($kind:ident, $token:expr) => {
        SyntaxError {
            token: $token.clone(),
            kind: SyntaxErrorKind::$kind,
        }
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxErrorKind {
    EmptyBrackets,
    EmptyParentheses,
    InvalidBinaryLiteral,
    InvalidFloat,
    InvalidFunctionName,
    InvalidHexLiteral,
    InvalidVariableName,
    MissingArgument,
    UnexpectedBrackets,
    UnexpectedComma,
    UnexpectedDot,
    UnexpectedEndOfExpression,
    UnexpectedNewLine,
    UnexpectedOperand,
    UnexpectedOperator,
    UnexpectedParenthesis,
    UnknownToken,
    UnmatchedBrackets,
    UnmatchedParenthesis,
    UnmatchedQuotationMark,
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self.kind {
            SyntaxErrorKind::EmptyBrackets => "Empty array access.",
            SyntaxErrorKind::EmptyParentheses => "Empty function or grouping.",
            SyntaxErrorKind::InvalidBinaryLiteral => match &self.token.value {
                None => "Invalid binary literal.",
                Some(value) => &format!("Invalid binary literal '0{}'.", value),
            },
            SyntaxErrorKind::InvalidFloat => "Invalid float.",
            SyntaxErrorKind::InvalidFunctionName => match &self.token.value {
                None => "Unexpected function name.",
                Some(value) => &format!("Unexpected function name '{}'.", value),
            },
            SyntaxErrorKind::InvalidHexLiteral => match &self.token.value {
                None => "Invalid hexadecimal literal.",
                Some(value) => &format!("Invalid hexadecimal literal '0{}'.", value),
            },
            SyntaxErrorKind::InvalidVariableName => "Invalid variable name.",
            SyntaxErrorKind::MissingArgument => "Missing function argument.",
            SyntaxErrorKind::UnexpectedBrackets => "Unexpected brackets.",
            SyntaxErrorKind::UnexpectedComma => "Unexpected comma.",
            SyntaxErrorKind::UnexpectedDot => "Unexpected dot.",
            SyntaxErrorKind::UnexpectedEndOfExpression => "Unexpected end of expression.",
            SyntaxErrorKind::UnexpectedNewLine => "Unexpected newline.",
            SyntaxErrorKind::UnexpectedOperand => match &self.token.value {
                None => "Unexpected operand.",
                Some(value) => &format!("Unexpected operand '{}'.", value),
            },
            SyntaxErrorKind::UnexpectedOperator => "Unexpected operator.",
            SyntaxErrorKind::UnexpectedParenthesis => "Unexpected parenthesis.",
            SyntaxErrorKind::UnknownToken => "Unknown token.",
            SyntaxErrorKind::UnmatchedBrackets => "Unmatched brackets.",
            SyntaxErrorKind::UnmatchedParenthesis => "Unmatched parenthesis.",
            SyntaxErrorKind::UnmatchedQuotationMark => "Unmatched quotation mark.",
        };

        write!(f, "{}", text)
    }
}

#[derive(Debug, Default)]
pub struct Status {
    pub expect_operand: bool,
    pub expect_operator: bool,
    pub in_string: bool,
}

impl SyntaxAnalyzer {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_owned(),
            current_index: 0,

            errors: Vec::new(),
            status: Status::default(),

            brackets_stack: VecDeque::new(),
            parentheses_stack: VecDeque::new(),
            quotation_marks_stack: VecDeque::new(),
        }
    }

    pub fn analyze(mut self) -> Vec<SyntaxError> {
        self.status = Status {
            expect_operand: true,
            expect_operator: false,
            in_string: false,
        };

        while self.current_index < self.tokens.len() {
            let token = &self.tokens[self.current_index];

            match &token.kind {
                TokenType::QuotationMark => {
                    // Toggle string state.
                    if !self.status.in_string {
                        // Start mark. We're expecting an operand here.
                        if !self.status.expect_operand {
                            // If we didn't expect an operand, it's an error.
                            self.errors.push(syntax_error!(UnexpectedOperator, token));
                        }
                        self.status.in_string = true;
                        // While inside string we're considering that operand is not finished
                        self.status.expect_operator = false;
                    } else {
                        // Closing mark
                        self.status.in_string = false;
                        // String literal is operand
                        self.status.expect_operator = true;
                    }

                    if self.quotation_marks_stack.is_empty() {
                        self.quotation_marks_stack.push_back(token.clone());
                    } else {
                        self.quotation_marks_stack.pop_back();
                    }

                    self.status.expect_operand = false;
                    self.current_index += 1;
                    continue;
                },

                _ if self.status.in_string => {
                    self.current_index += 1;
                    continue;
                },

                TokenType::ExclamationMark => {
                    // Used only like identifier part
                    if self.status.expect_operand {
                        self.status.expect_operand = true;
                        self.status.expect_operator = false;
                    } else {
                        self.errors.push(syntax_error!(UnexpectedOperator, token));
                        // Continuing, but considering that operator was read.
                    }
                    self.current_index += 1;
                    continue;
                },

                TokenType::Identifier => {
                    // Identifier - operand
                    if !self.status.expect_operand {
                        self.errors.push(syntax_error!(UnexpectedOperand, token));
                        // Continuing, but considering that operand was read
                    }
                    self.status.expect_operand = false;
                    self.status.expect_operator = true;
                    self.current_index += 1;
                    continue;
                },

                TokenType::Number => {
                    // Number - operand
                    if !self.status.expect_operand {
                        self.errors.push(syntax_error!(UnexpectedOperand, token));
                        self.current_index += 1;
                        continue;
                    }

                    // Binary and Hex validating
                    if let Some(prefix) = &token.value
                        && prefix.eq("0")
                        && let Some(next) = self.peek_next()
                        && next.kind == TokenType::Identifier
                        && let Some(value) = &next.value
                        && value.to_ascii_lowercase().starts_with(['x', 'b'])
                        && value.len() > 1
                    {
                        // Hex
                        if value.to_ascii_lowercase().starts_with('x')
                            && !value[1..].chars().all(|c| c.is_ascii_hexdigit())
                        {
                            // Incorrect hex literal
                            self.errors.push(syntax_error!(InvalidHexLiteral, next));
                        }
                        // Binary
                        else if value.to_ascii_lowercase().starts_with('b')
                            && !value[1..].chars().all(|c| c == '0' || c == '1')
                        {
                            // Incorrect binary literal
                            self.errors.push(syntax_error!(InvalidBinaryLiteral, next));
                        }

                        // Anyway, considering that identifier was read
                        self.current_index += 2;
                        self.status.expect_operand = false;
                        self.status.expect_operator = true;
                        continue;
                    }

                    // Float validating
                    if let Some(next) = self.peek_next()
                        && next.kind == TokenType::Dot
                    {
                        if let Some(second) = self.peek_next_by(2) {
                            if matches!(&second.kind, TokenType::Number) {
                                // Correct float! Number-Dot-Number
                                // Next token - the third
                                self.current_index += 3;
                            } else {
                                // Something else after dot - error
                                self.errors.push(syntax_error!(InvalidFloat, next));
                                // Skipping number with the dot
                                self.current_index += 2;
                            }
                        } else {
                            // Dot in the end - error
                            self.errors.push(syntax_error!(UnexpectedOperator, next));
                            self.current_index += 2;
                        }
                        self.status.expect_operand = false;
                        self.status.expect_operator = true;
                        continue;
                    }

                    // Bad variable name?
                    if let Some(next) = self.peek_next()
                        && next.kind == TokenType::Identifier
                    {
                        // But if second next identifier is left parentheses - it's function name
                        if let Some(second) = self.peek_next_by(2)
                            && second.kind == TokenType::LeftParenthesis
                        {
                            // Function name cannot start with a number
                            self.errors.push(syntax_error!(InvalidFunctionName, token));
                        } else {
                            // If next token is identifier, then it's bad variable name
                            self.errors.push(syntax_error!(InvalidVariableName, token));
                        }

                        // Skipping invalid identifier
                        self.current_index += 2;
                        self.status.expect_operand = false;
                        self.status.expect_operator = true;
                        continue;
                    }

                    // Integer literal
                    self.current_index += 1;
                    self.status.expect_operand = false;
                    self.status.expect_operator = true;
                    continue;
                },

                TokenType::Dot => {
                    self.errors.push(syntax_error!(UnexpectedDot, token));
                    self.current_index += 1;
                    continue;
                },

                // Mathematical and logical operations
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Asterisk
                | TokenType::Slash
                | TokenType::Percent
                | TokenType::Ampersand
                | TokenType::Pipe => {
                    // Unary operations
                    let unary = if [TokenType::Minus].contains(&token.kind)
                        && let Some(next) = self.peek_next()
                        && [
                            TokenType::Identifier,
                            TokenType::Number,
                            TokenType::LeftParenthesis,
                        ]
                        .contains(&next.kind)
                    {
                        true
                    } else {
                        false
                    };

                    if self.status.expect_operator || unary {
                        self.status.expect_operand = true;
                        self.status.expect_operator = false;
                    } else {
                        self.errors.push(syntax_error!(UnexpectedOperator, token));
                        // Waiting for operand still
                    }
                    self.current_index += 1;
                    continue;
                },

                TokenType::LeftBracket => {
                    // LeftBracket can be there if previous token is Identifier (array access)
                    // or that's array with more than one dimension (e.g. arr[2][3])
                    let allow = matches!(self.peek_previous(), Some(t) if matches!(t.kind, TokenType::Identifier))
                        || matches!(self.peek_previous(), Some(t) if matches!(t.kind, TokenType::RightBracket));
                    if !allow {
                        self.errors.push(syntax_error!(UnexpectedBrackets, token));
                        self.current_index += 1;
                        continue;
                    }

                    self.brackets_stack.push_back(token.clone());
                    self.status.expect_operand = true;
                    self.status.expect_operator = false;
                    self.current_index += 1;
                    continue;
                },

                TokenType::RightBracket => {
                    match self.brackets_stack.pop_back().is_some() {
                        true => {
                            // Correct
                            self.status.expect_operand = false;
                            self.status.expect_operator = true;
                        },
                        false => {
                            self.errors.push(syntax_error!(UnmatchedBrackets, token))
                        },
                    }

                    // Empty array access check
                    if let Some(previous) = self.peek_previous()
                        && matches!(previous.kind, TokenType::LeftBracket)
                    {
                        self.errors.push(syntax_error!(EmptyBrackets, token));
                    }

                    self.current_index += 1;
                    continue;
                },

                TokenType::LeftParenthesis => {
                    // LeftParenthesis can be there if we're waiting for operand (grouping)
                    // or previous token is Identifier (function call)
                    // Number - error (processing later)
                    // RightParenthesis - error (processing later)
                    let allow = self.status.expect_operand
                        || matches!(self.peek_previous(), Some(t) if matches!(t.kind, TokenType::Identifier))
                        || matches!(self.peek_previous(), Some(t) if matches!(t.kind, TokenType::RightParenthesis))
                        || matches!(self.peek_previous(), Some(t) if matches!(t.kind, TokenType::Number));
                    if !allow {
                        self.errors
                            .push(syntax_error!(UnexpectedParenthesis, token));
                    }

                    if let Some(previous) = self.peek_previous()
                        && matches!(previous.kind, TokenType::Number)
                    {
                        // Function name cannot start with a number
                        self.errors
                            .push(syntax_error!(InvalidFunctionName, previous));
                    }

                    if let Some(previous) = self.peek_previous()
                        && matches!(previous.kind, TokenType::RightParenthesis)
                    {
                        // Needed operation. but anyway, pushing to the stack
                        self.errors
                            .push(syntax_error!(UnexpectedParenthesis, token));
                    }

                    self.parentheses_stack.push_back(token.clone());
                    self.status.expect_operand = true;
                    self.status.expect_operator = false;
                    self.current_index += 1;
                    continue;
                },

                TokenType::RightParenthesis => {
                    // Empty grouping check. Also, empty function is not an error.
                    if let Some(possible_left_parentheses) = self.peek_previous()
                        && matches!(
                            possible_left_parentheses.kind,
                            TokenType::LeftParenthesis
                        )
                    {
                        // But, non-function
                        if let Some(possible_function_name) = self.peek_previous_by(2)
                            && matches!(
                                possible_function_name.kind,
                                TokenType::Identifier
                            )
                        {
                            self.status.expect_operand = false;
                            self.status.expect_operator = true;
                        } else {
                            self.errors.push(syntax_error!(EmptyParentheses, token));
                        }
                    } else if self.status.expect_operand {
                        self.errors
                            .push(syntax_error!(UnexpectedParenthesis, token));
                    }

                    match self.parentheses_stack.pop_back().is_some() {
                        true => {
                            // Correct
                            self.status.expect_operand = false;
                            self.status.expect_operator = true;
                        },
                        false => {
                            self.errors.push(syntax_error!(UnmatchedParenthesis, token))
                        },
                    }

                    self.current_index += 1;
                    continue;
                },

                TokenType::Comma => {
                    // Allowed only inside parentheses (function)
                    if self.parentheses_stack.is_empty() {
                        // Surely an error
                        self.errors.push(syntax_error!(UnexpectedComma, token));
                        self.status.expect_operand = true;
                        self.status.expect_operator = false;
                        self.current_index += 1;
                        continue;
                    }

                    // Inside parentheses comma need to be after operand and before new operand
                    if self.status.expect_operand {
                        // Empty argument
                        self.errors.push(syntax_error!(UnexpectedComma, token));
                        self.current_index += 1;
                        continue;
                    }

                    // Argument is not present
                    if let Some(next) = self.peek_next()
                        && matches!(next.kind, TokenType::RightParenthesis)
                    {
                        // Empty argument
                        self.errors.push(syntax_error!(MissingArgument, token));
                        self.current_index += 1;
                        continue;
                    }

                    // Expecting new operand
                    self.status.expect_operand = true;
                    self.status.expect_operator = false;
                    self.current_index += 1;
                    continue;
                },

                TokenType::Unknown => {
                    // Unknown — always an error
                    self.errors.push(syntax_error!(UnknownToken, token));
                    self.current_index += 1;
                    continue;
                },
                TokenType::NewLine => {
                    // Unexpected newline is error, if we're not in string
                    if !self.status.in_string {
                        self.errors.push(syntax_error!(UnexpectedNewLine, token));
                    }
                    self.current_index += 1;
                    continue;
                },
                TokenType::Space | TokenType::Tab => {
                    // Shouldn't be here, but skipping just in case
                    self.current_index += 1;
                    continue;
                },
            }
        }

        // Error for every unmatched left parenthesis
        for unmatched in self.parentheses_stack.into_iter() {
            self.errors
                .push(syntax_error!(UnmatchedParenthesis, unmatched));
        }

        // If operand is expected in the end, it's the error.
        if let Some(last) = self.tokens.last()
            && self.status.expect_operand
        {
            self.errors
                .push(syntax_error!(UnexpectedEndOfExpression, last));
        }

        // Unclosed string
        if let Some(token) = self.quotation_marks_stack.pop_back()
            && self.status.in_string
        {
            self.errors
                .push(syntax_error!(UnmatchedQuotationMark, token));
        }

        self.errors
            .sort_by(|a, b| a.token.position.start.cmp(&b.token.position.start));

        self.errors
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current_index + 1)
    }

    fn peek_next_by(&self, by: usize) -> Option<&Token> {
        self.tokens.get(self.current_index + by)
    }

    fn peek_previous(&self) -> Option<&Token> {
        self.tokens.get(self.current_index.checked_sub(1)?)
    }

    fn peek_previous_by(&self, by: usize) -> Option<&Token> {
        self.tokens.get(self.current_index.checked_sub(by)?)
    }
}

impl Reporter {
    pub fn syntax(
        &self, code: &str, pretty_output: bool, syntax_errors: &[SyntaxError],
    ) -> String {
        let mut buffer = StringBuffer::default();

        let first_line = match syntax_errors.len() {
            0 => "Tokenization & syntax analysis: OK!\n".to_string(),
            n => format!("Syntax analysis: Found {} errors.\n", n),
        };
        buffer.add_line(first_line);

        if syntax_errors.is_empty() {
            return buffer.get();
        }

        match pretty_output {
            true => self.format_errors_pretty(&mut buffer, code, syntax_errors),
            false => self.format_errors(&mut buffer, syntax_errors),
        };

        buffer.get()
    }

    fn format_errors_pretty(
        &self, buffer: &mut StringBuffer, code: &str, syntax_errors: &[SyntaxError],
    ) {
        buffer.add_line(format!("\n{}", code));

        // First line: Underlines
        let length = code.len();
        let mut first_line = " ".repeat(length);
        for error in syntax_errors {
            let underline_length = error.token.position.end - error.token.position.start;
            if underline_length == 1 {
                first_line.replace_char(error.token.position.start, '^');
            } else {
                for index in
                    (error.token.position.start + 1)..(error.token.position.end - 1)
                {
                    first_line.replace_char(index, '-');
                }

                first_line.replace_char(error.token.position.start, '^');
                first_line.replace_char(error.token.position.end - 1, '^');
            }
        }
        buffer.add_line(first_line);

        // Other lines
        for error in syntax_errors.iter().rev() {
            // One for -, another one for \n
            let mut line = " ".repeat(length + 2);
            for error in syntax_errors.iter() {
                line.replace_char(error.token.position.start, '|');
            }
            for index in (error.token.position.start + 1)..(length + 1) {
                line.replace_char(index, '_');
            }
            line.push_str(&error.to_string());
            buffer.add_line(line);
        }
    }

    fn format_errors(&self, buffer: &mut StringBuffer, syntax_errors: &[SyntaxError]) {
        for error in syntax_errors {
            let error = format!(
                "{:50} {}",
                error.to_string(),
                error.token.display_position()
            );
            buffer.add_line(error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::tokenizer::Tokenizer;

    macro_rules! test_error {
        ($error_kind:ident, $token_kind:expr, $position:literal) => {
            SyntaxError {
                token: Token {
                    kind: $token_kind,
                    position: $position..$position + 1,
                    value: None,
                },
                kind: SyntaxErrorKind::$error_kind,
            }
        };
        ($error_kind:ident, $token_kind:expr, $position:expr) => {
            SyntaxError {
                token: Token {
                    kind: $token_kind,
                    position: $position,
                    value: None,
                },
                kind: SyntaxErrorKind::$error_kind,
            }
        };
        ($error_kind:ident, $token_kind:expr, $position:literal, $value:expr) => {
            SyntaxError {
                token: Token {
                    kind: $token_kind,
                    position: $position..$position + 1,
                    value: Some($value),
                },
                kind: SyntaxErrorKind::$error_kind,
            }
        };
        ($error_kind:ident, $token_kind:expr, $position:expr, $value:expr) => {
            SyntaxError {
                token: Token {
                    kind: $token_kind,
                    position: $position,
                    value: Some($value),
                },
                kind: SyntaxErrorKind::$error_kind,
            }
        };
    }

    #[test]
    fn test_syntax_01() {
        let code = "-a ++ b - 2v*func((t+2 -, sin(x/*2.01.2), )/8(-)**";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnexpectedOperator, TokenType::Plus, 4),
            test_error!(InvalidVariableName, TokenType::Number, 10, "2".to_string()),
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 17),
            test_error!(UnexpectedComma, TokenType::Comma, 24),
            test_error!(UnexpectedOperator, TokenType::Asterisk, 32),
            test_error!(UnexpectedDot, TokenType::Dot, 37),
            test_error!(UnexpectedOperand, TokenType::Number, 38, "2".to_string()),
            test_error!(MissingArgument, TokenType::Comma, 40),
            test_error!(InvalidFunctionName, TokenType::Number, 44, "8".to_string()),
            test_error!(UnexpectedOperator, TokenType::Minus, 46),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 47),
            test_error!(UnexpectedOperator, TokenType::Asterisk, 49),
            test_error!(UnexpectedEndOfExpression, TokenType::Asterisk, 49),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_02() {
        let code = "*a + nb -";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnexpectedOperator, TokenType::Asterisk, 0),
            test_error!(UnexpectedEndOfExpression, TokenType::Minus, 8),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_03() {
        let code = "a ++ nb /* k -+/ g";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnexpectedOperator, TokenType::Plus, 3),
            test_error!(UnexpectedOperator, TokenType::Asterisk, 9),
            test_error!(UnexpectedOperator, TokenType::Plus, 14),
            test_error!(UnexpectedOperator, TokenType::Slash, 15),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_04() {
        let code = "a^b$c - d#h + q%t + !b&(z|t)";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnknownToken, TokenType::Unknown, 1, "^".to_string()),
            test_error!(UnexpectedOperand, TokenType::Identifier, 2, "b".to_string()),
            test_error!(UnknownToken, TokenType::Unknown, 3, "$".to_string()),
            test_error!(UnexpectedOperand, TokenType::Identifier, 4, "c".to_string()),
            test_error!(UnknownToken, TokenType::Unknown, 9, "#".to_string()),
            test_error!(
                UnexpectedOperand,
                TokenType::Identifier,
                10,
                "h".to_string()
            ),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_05() {
        let code = "x + var1 + var_2 + _var_3 + var#4 + var!5
    + 6var_ + $7 + ?8";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnknownToken, TokenType::Unknown, 31, "#".to_string()),
            test_error!(UnexpectedOperand, TokenType::Number, 32, "4".to_string()),
            test_error!(UnexpectedOperator, TokenType::ExclamationMark, 39),
            test_error!(UnexpectedOperand, TokenType::Number, 40, "5".to_string()),
            test_error!(UnexpectedNewLine, TokenType::NewLine, 41),
            test_error!(InvalidVariableName, TokenType::Number, 48, "6".to_string()),
            test_error!(UnknownToken, TokenType::Unknown, 56, "$".to_string()),
            test_error!(UnknownToken, TokenType::Unknown, 61, "?".to_string()),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_06() {
        let code = "125 + 2nb - 0xAB * 0x0R + 0b010 * 0b20+ ABh * 0Rh + 010b*20b";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(InvalidVariableName, TokenType::Number, 6, "2".to_string()),
            test_error!(
                InvalidHexLiteral,
                TokenType::Identifier,
                20..23,
                "x0R".to_string()
            ),
            test_error!(
                InvalidBinaryLiteral,
                TokenType::Identifier,
                35..38,
                "b20".to_string()
            ),
            test_error!(InvalidVariableName, TokenType::Number, 46, "0".to_string()),
            test_error!(
                InvalidVariableName,
                TokenType::Number,
                52..55,
                "010".to_string()
            ),
            test_error!(
                InvalidVariableName,
                TokenType::Number,
                57..59,
                "20".to_string()
            ),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_07() {
        let code = "0.71/0.72.3 + .3 + 127.0.0.1*8. + 6.07ab - 9f.89hgt";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnexpectedDot, TokenType::Dot, 9),
            test_error!(UnexpectedOperand, TokenType::Number, 10, "3".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 14),
            test_error!(UnexpectedDot, TokenType::Dot, 24),
            test_error!(UnexpectedOperand, TokenType::Number, 25, "0".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 26),
            test_error!(UnexpectedOperand, TokenType::Number, 27, "1".to_string()),
            test_error!(InvalidFloat, TokenType::Dot, 30),
            test_error!(
                UnexpectedOperand,
                TokenType::Identifier,
                38..40,
                "ab".to_string()
            ),
            test_error!(InvalidVariableName, TokenType::Number, 43, "9".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 45),
            test_error!(
                UnexpectedOperand,
                TokenType::Number,
                46..48,
                "89".to_string()
            ),
            test_error!(
                UnexpectedOperand,
                TokenType::Identifier,
                48..51,
                "hgt".to_string()
            ),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_08() {
        let code = ")a+b( -(g+h)(g-k))*()) + (-b(t-2*x*(5) + A[7][2-x]";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 0),
            test_error!(UnmatchedParenthesis, TokenType::RightParenthesis, 0),
            test_error!(UnexpectedParenthesis, TokenType::LeftParenthesis, 12),
            test_error!(EmptyParentheses, TokenType::RightParenthesis, 20),
            test_error!(UnmatchedParenthesis, TokenType::RightParenthesis, 21),
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 25),
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 28),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_09() {
        let code = "2(t) - f2(t) + g()/h(2, )*func(-t/q, f(4-t), - (x+2)*(y-2))";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(InvalidFunctionName, TokenType::Number, 0, "2".to_string()),
            test_error!(MissingArgument, TokenType::Comma, 22),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_10() {
        let code = "/a*b**c + m)*a*b + a*c - a*smn(j*k/m + m";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnexpectedOperator, TokenType::Slash, 0),
            test_error!(UnexpectedOperator, TokenType::Asterisk, 5),
            test_error!(UnmatchedParenthesis, TokenType::RightParenthesis, 11),
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 30),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_11() {
        let code =
            "-cos(-&t))/(*(*f)(127.0.0.1, \"/dev/null\", (t==0)?4more_errors:b^2) - .5";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnexpectedOperator, TokenType::Minus, 5),
            test_error!(UnexpectedOperator, TokenType::Ampersand, 6),
            test_error!(UnmatchedParenthesis, TokenType::RightParenthesis, 9),
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 11),
            test_error!(UnexpectedOperator, TokenType::Asterisk, 12),
            test_error!(UnexpectedOperator, TokenType::Asterisk, 14),
            test_error!(UnexpectedParenthesis, TokenType::LeftParenthesis, 17),
            test_error!(UnexpectedDot, TokenType::Dot, 23),
            test_error!(UnexpectedOperand, TokenType::Number, 24, "0".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 25),
            test_error!(UnexpectedOperand, TokenType::Number, 26, "1".to_string()),
            test_error!(UnknownToken, TokenType::Unknown, 44, "=".to_string()),
            test_error!(UnknownToken, TokenType::Unknown, 45, "=".to_string()),
            test_error!(UnexpectedOperand, TokenType::Number, 46, "0".to_string()),
            test_error!(UnknownToken, TokenType::Unknown, 48, "?".to_string()),
            test_error!(UnexpectedOperand, TokenType::Number, 49, "4".to_string()),
            test_error!(
                UnexpectedOperand,
                TokenType::Identifier,
                50..61,
                "more_errors".to_string()
            ),
            test_error!(UnknownToken, TokenType::Unknown, 61, ":".to_string()),
            test_error!(
                UnexpectedOperand,
                TokenType::Identifier,
                62,
                "b".to_string()
            ),
            test_error!(UnknownToken, TokenType::Unknown, 63, "^".to_string()),
            test_error!(UnexpectedOperand, TokenType::Number, 64, "2".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 69),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_12() {
        let code = "//(*0)- an*0p(a+b)-1.000.5//6(*f(-b, 1.8-0*(2-6) %1 + (++a)/(6x^2+4x-1) + d/dt*(smn(at+q)/(4cos(at)-ht^2)";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnexpectedOperator, TokenType::Slash, 0),
            test_error!(UnexpectedOperator, TokenType::Slash, 1),
            test_error!(UnexpectedOperator, TokenType::Asterisk, 3),
            test_error!(InvalidFunctionName, TokenType::Number, 11, "0".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 24),
            test_error!(UnexpectedOperand, TokenType::Number, 25, "5".to_string()),
            test_error!(UnexpectedOperator, TokenType::Slash, 27),
            test_error!(InvalidFunctionName, TokenType::Number, 28, "6".to_string()),
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 29),
            test_error!(UnexpectedOperator, TokenType::Asterisk, 30),
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 32),
            test_error!(UnexpectedOperator, TokenType::Plus, 55),
            test_error!(UnexpectedOperator, TokenType::Plus, 56),
            test_error!(InvalidVariableName, TokenType::Number, 61, "6".to_string()),
            test_error!(UnknownToken, TokenType::Unknown, 63, "^".to_string()),
            test_error!(UnexpectedOperand, TokenType::Number, 64, "2".to_string()),
            test_error!(InvalidVariableName, TokenType::Number, 66, "4".to_string()),
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 79),
            test_error!(InvalidFunctionName, TokenType::Number, 91, "4".to_string()),
            test_error!(UnknownToken, TokenType::Unknown, 102, "^".to_string()),
            test_error!(UnexpectedOperand, TokenType::Number, 103, "2".to_string()),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_13() {
        let code = "-(-5x((int*)exp())/t - 3.14.15k/(2x^2-5x-1)*y - A[N*(i++)+j]";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 1),
            test_error!(InvalidFunctionName, TokenType::Number, 3, "5".to_string()),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 11),
            test_error!(
                UnexpectedOperand,
                TokenType::Identifier,
                12..15,
                "exp".to_string()
            ),
            test_error!(UnexpectedDot, TokenType::Dot, 27),
            test_error!(
                UnexpectedOperand,
                TokenType::Number,
                28..30,
                "15".to_string()
            ),
            test_error!(
                UnexpectedOperand,
                TokenType::Identifier,
                30,
                "k".to_string()
            ),
            test_error!(InvalidVariableName, TokenType::Number, 33, "2".to_string()),
            test_error!(UnknownToken, TokenType::Unknown, 35, "^".to_string()),
            test_error!(UnexpectedOperand, TokenType::Number, 36, "2".to_string()),
            test_error!(InvalidVariableName, TokenType::Number, 38, "5".to_string()),
            test_error!(UnexpectedOperator, TokenType::Plus, 55),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 56),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_14() {
        let code = "-(-exp(3et/4.0.2, 2i-1)/L + )((void*)*f()) + ((i++) + (++i/(i--))/k//) + 6.000.500.5";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(InvalidVariableName, TokenType::Number, 7, "3".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 14),
            test_error!(UnexpectedOperand, TokenType::Number, 15, "2".to_string()),
            test_error!(InvalidVariableName, TokenType::Number, 18, "2".to_string()),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 28),
            test_error!(UnexpectedParenthesis, TokenType::LeftParenthesis, 29),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 36),
            test_error!(UnexpectedOperator, TokenType::Plus, 49),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 50),
            test_error!(UnexpectedOperator, TokenType::Plus, 55),
            test_error!(UnexpectedOperator, TokenType::Plus, 56),
            test_error!(UnexpectedOperator, TokenType::Minus, 62),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 63),
            test_error!(UnexpectedOperator, TokenType::Slash, 68),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 69),
            test_error!(UnexpectedDot, TokenType::Dot, 78),
            test_error!(
                UnexpectedOperand,
                TokenType::Number,
                79..82,
                "500".to_string()
            ),
            test_error!(UnexpectedDot, TokenType::Dot, 82),
            test_error!(UnexpectedOperand, TokenType::Number, 83, "5".to_string()),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_15() {
        let code = "**f(*k, -p+1, ))2.1.1 + 1.8q((-5x ++ i)";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnexpectedOperator, TokenType::Asterisk, 0),
            test_error!(UnexpectedOperator, TokenType::Asterisk, 1),
            test_error!(UnexpectedOperator, TokenType::Asterisk, 4),
            test_error!(MissingArgument, TokenType::Comma, 12),
            test_error!(UnmatchedParenthesis, TokenType::RightParenthesis, 15),
            test_error!(UnexpectedOperand, TokenType::Number, 16, "2".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 17),
            test_error!(UnexpectedOperand, TokenType::Number, 18, "1".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 19),
            test_error!(UnexpectedOperand, TokenType::Number, 20, "1".to_string()),
            test_error!(
                UnexpectedOperand,
                TokenType::Identifier,
                27,
                "q".to_string()
            ),
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 28),
            test_error!(InvalidVariableName, TokenType::Number, 31, "5".to_string()),
            test_error!(UnexpectedOperator, TokenType::Plus, 35),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_16() {
        let code = "/.1(2x^2-5x+7)-(-i)+ (j++)/0 - )(*f)(2, 7-x, )/q + send(-(2x+7)/A[j, i], 127.0.0.1 ) + )/";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnexpectedOperator, TokenType::Slash, 0),
            test_error!(UnexpectedDot, TokenType::Dot, 1),
            test_error!(InvalidFunctionName, TokenType::Number, 2, "1".to_string()),
            test_error!(InvalidVariableName, TokenType::Number, 4, "2".to_string()),
            test_error!(UnknownToken, TokenType::Unknown, 6, "^".to_string()),
            test_error!(UnexpectedOperand, TokenType::Number, 7, "2".to_string()),
            test_error!(InvalidVariableName, TokenType::Number, 9, "5".to_string()),
            test_error!(UnexpectedOperator, TokenType::Plus, 24),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 25),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 31),
            test_error!(UnmatchedParenthesis, TokenType::RightParenthesis, 31),
            test_error!(UnexpectedParenthesis, TokenType::LeftParenthesis, 32),
            test_error!(UnexpectedOperator, TokenType::Asterisk, 33),
            test_error!(UnexpectedParenthesis, TokenType::LeftParenthesis, 36),
            test_error!(MissingArgument, TokenType::Comma, 43),
            test_error!(InvalidVariableName, TokenType::Number, 58, "2".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 78),
            test_error!(UnexpectedOperand, TokenType::Number, 79, "0".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 80),
            test_error!(UnexpectedOperand, TokenType::Number, 81, "1".to_string()),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 87),
            test_error!(UnmatchedParenthesis, TokenType::RightParenthesis, 87),
            test_error!(UnexpectedOperator, TokenType::Slash, 88),
            test_error!(UnexpectedEndOfExpression, TokenType::Slash, 88),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_17() {
        let code =
            "*101*1#(t-q)(t+q)//dt - (int*)f(8t, -(k/h)A[i+6.]), exp(), ))(t-k*8.00.1/.0";

        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnexpectedOperator, TokenType::Asterisk, 0),
            test_error!(UnknownToken, TokenType::Unknown, 6, "#".to_string()),
            test_error!(UnexpectedParenthesis, TokenType::LeftParenthesis, 7),
            test_error!(UnexpectedParenthesis, TokenType::LeftParenthesis, 12),
            test_error!(UnexpectedOperator, TokenType::Slash, 18),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 29),
            test_error!(
                UnexpectedOperand,
                TokenType::Identifier,
                30,
                "f".to_string()
            ),
            test_error!(InvalidVariableName, TokenType::Number, 32, "8".to_string()),
            test_error!(
                UnexpectedOperand,
                TokenType::Identifier,
                42,
                "A".to_string()
            ),
            test_error!(InvalidFloat, TokenType::Dot, 47),
            test_error!(UnexpectedComma, TokenType::Comma, 50),
            test_error!(UnexpectedComma, TokenType::Comma, 57),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 59),
            test_error!(UnmatchedParenthesis, TokenType::RightParenthesis, 59),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 60),
            test_error!(UnmatchedParenthesis, TokenType::RightParenthesis, 60),
            test_error!(UnexpectedParenthesis, TokenType::LeftParenthesis, 61),
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 61),
            test_error!(UnexpectedDot, TokenType::Dot, 70),
            test_error!(UnexpectedOperand, TokenType::Number, 71, "1".to_string()),
            test_error!(UnexpectedDot, TokenType::Dot, 73),
        ];
        assert_eq!(errors_actual, errors_expected);
    }

    #[test]
    fn test_syntax_18() {
        let code = "-(-5x((int*)exp())/t - \"sdds + 4.5+3";
        let errors_actual: Vec<SyntaxError> =
            SyntaxAnalyzer::new(&Tokenizer::process(code)).analyze();
        let errors_expected: Vec<SyntaxError> = vec![
            test_error!(UnmatchedParenthesis, TokenType::LeftParenthesis, 1),
            test_error!(InvalidFunctionName, TokenType::Number, 3, "5".to_string()),
            test_error!(UnexpectedParenthesis, TokenType::RightParenthesis, 11),
            test_error!(
                UnexpectedOperand,
                TokenType::Identifier,
                12..15,
                "exp".to_string()
            ),
            test_error!(UnmatchedQuotationMark, TokenType::QuotationMark, 23),
        ];
        assert_eq!(errors_actual, errors_expected);
    }
}
