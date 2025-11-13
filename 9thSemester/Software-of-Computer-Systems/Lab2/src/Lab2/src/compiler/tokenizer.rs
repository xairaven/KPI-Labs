use std::ops::Range;
use strum_macros::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenType,
    pub position: Range<usize>,
    pub value: Option<String>,
}

impl Token {
    pub fn display_position(&self) -> String {
        if self.position.start + 1 == self.position.end {
            format!("[Position: {}]", self.position.start + 1)
        } else {
            format!(
                "[Position: {}..{}]",
                self.position.start + 1,
                self.position.end
            )
        }
    }

    pub fn display_value(&self) -> String {
        let text = match self.kind {
            TokenType::Identifier | TokenType::Number => match &self.value {
                Some(value) => value.as_str(),
                None => "NONE",
            },
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::Percent => "%",
            TokenType::LeftParenthesis => "(",
            TokenType::RightParenthesis => ")",
            TokenType::LeftBracket => "[",
            TokenType::RightBracket => "]",
            TokenType::ExclamationMark => "!",
            TokenType::Ampersand => "&",
            TokenType::Pipe => "|",
            TokenType::Dot => ".",
            TokenType::Comma => ",",
            TokenType::QuotationMark => "\"",
            TokenType::Space => " ",
            TokenType::Tab => "\\t",
            TokenType::NewLine => "\\n",
            TokenType::Unknown => "<UNKNOWN>",
        };

        text.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum TokenType {
    Identifier,
    Number,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,

    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,

    ExclamationMark,
    Ampersand,
    Pipe,

    Dot,
    Comma,

    QuotationMark,

    Space,
    Tab,
    NewLine,

    Unknown,
}

#[macro_export]
macro_rules! token {
    ($token_type:expr, $position:literal) => {
        Token {
            kind: $token_type,
            position: $position..($position + 1),
            value: None,
        }
    };
    ($token_type:expr, $position:expr) => {
        Token {
            kind: $token_type,
            position: $position,
            value: None,
        }
    };
    ($token_type:expr, $value:expr, $position:literal) => {
        Token {
            kind: $token_type,
            position: $position..($position + 1),
            value: Some($value),
        }
    };
    ($token_type:expr, $value:expr, $position:expr) => {
        Token {
            kind: $token_type,
            position: $position,
            value: Some($value),
        }
    };
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();

    let mut in_string = false;
    for (index, symbol) in chars.iter().enumerate() {
        if let Some(last_token) = tokens.last()
            && last_token.position.end > index
        {
            continue;
        }

        let token = match symbol {
            symbol if symbol.is_alphabetic() || symbol.eq(&'_') => {
                let start = index;
                let mut end = index + 1;

                while end < chars.len()
                    && (chars[end].is_alphanumeric() || chars[end] == '_')
                {
                    end += 1;
                }

                let value: String = chars[start..end].iter().collect();
                token!(TokenType::Identifier, value, start..end)
            },
            '0'..='9' => {
                let start = index;
                let mut end = index + 1;

                while end < chars.len() && chars[end].is_numeric() {
                    end += 1;
                }

                let value: String = chars[start..end].iter().collect();
                token!(TokenType::Number, value, start..end)
            },
            '+' => token!(TokenType::Plus, index..index + 1),
            '-' => token!(TokenType::Minus, index..index + 1),
            '*' => token!(TokenType::Asterisk, index..index + 1),
            '/' => token!(TokenType::Slash, index..index + 1),
            '%' => token!(TokenType::Percent, index..index + 1),
            '(' => token!(TokenType::LeftParenthesis, index..index + 1),
            ')' => token!(TokenType::RightParenthesis, index..index + 1),
            '[' => token!(TokenType::LeftBracket, index..index + 1),
            ']' => token!(TokenType::RightBracket, index..index + 1),
            '!' => token!(TokenType::ExclamationMark, index..index + 1),
            '&' => token!(TokenType::Ampersand, index..index + 1),
            '|' => token!(TokenType::Pipe, index..index + 1),
            '.' => token!(TokenType::Dot, index..index + 1),
            ',' => token!(TokenType::Comma, index..index + 1),
            '"' => {
                in_string = !in_string;
                token!(TokenType::QuotationMark, index..index + 1)
            },
            '\n' => token!(TokenType::NewLine, index..index + 1),
            c if c.eq(&'\t') => token!(TokenType::Tab, index..index + 1),
            c if c.is_whitespace() => {
                let start = index;
                let mut end = index + 1;

                while end < chars.len() && chars[end].is_whitespace() {
                    end += 1;
                }

                if !in_string {
                    continue;
                }

                token!(TokenType::Space, start..end)
            },
            c => token!(TokenType::Unknown, c.to_string(), index..index + 1),
        };

        tokens.push(token);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_01() {
        let code = "-a ++ b - 2v*func((t+2 -, sin(x/*2.01.2), )/8(-)**";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Minus, 0),
            token!(TokenType::Identifier, "a".to_string(), 1),
            token!(TokenType::Plus, 3),
            token!(TokenType::Plus, 4),
            token!(TokenType::Identifier, "b".to_string(), 6),
            token!(TokenType::Minus, 8),
            token!(TokenType::Number, "2".to_string(), 10),
            token!(TokenType::Identifier, "v".to_string(), 11),
            token!(TokenType::Asterisk, 12),
            token!(TokenType::Identifier, "func".to_string(), 13..17),
            token!(TokenType::LeftParenthesis, 17),
            token!(TokenType::LeftParenthesis, 18),
            token!(TokenType::Identifier, "t".to_string(), 19),
            token!(TokenType::Plus, 20),
            token!(TokenType::Number, "2".to_string(), 21),
            token!(TokenType::Minus, 23),
            token!(TokenType::Comma, 24),
            token!(TokenType::Identifier, "sin".to_string(), 26..29),
            token!(TokenType::LeftParenthesis, 29),
            token!(TokenType::Identifier, "x".to_string(), 30),
            token!(TokenType::Slash, 31),
            token!(TokenType::Asterisk, 32),
            token!(TokenType::Number, "2".to_string(), 33),
            token!(TokenType::Dot, 34),
            token!(TokenType::Number, "01".to_string(), 35..37),
            token!(TokenType::Dot, 37),
            token!(TokenType::Number, "2".to_string(), 38),
            token!(TokenType::RightParenthesis, 39),
            token!(TokenType::Comma, 40),
            token!(TokenType::RightParenthesis, 42),
            token!(TokenType::Slash, 43),
            token!(TokenType::Number, "8".to_string(), 44),
            token!(TokenType::LeftParenthesis, 45),
            token!(TokenType::Minus, 46),
            token!(TokenType::RightParenthesis, 47),
            token!(TokenType::Asterisk, 48),
            token!(TokenType::Asterisk, 49),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_02() {
        let code = "*a + nb -";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Asterisk, 0),
            token!(TokenType::Identifier, "a".to_string(), 1),
            token!(TokenType::Plus, 3),
            token!(TokenType::Identifier, "nb".to_string(), 5..7),
            token!(TokenType::Minus, 8),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_03() {
        let code = "a ++ nb /* k -+/ g";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Identifier, "a".to_string(), 0),
            token!(TokenType::Plus, 2),
            token!(TokenType::Plus, 3),
            token!(TokenType::Identifier, "nb".to_string(), 5..7),
            token!(TokenType::Slash, 8),
            token!(TokenType::Asterisk, 9),
            token!(TokenType::Identifier, "k".to_string(), 11),
            token!(TokenType::Minus, 13),
            token!(TokenType::Plus, 14),
            token!(TokenType::Slash, 15),
            token!(TokenType::Identifier, "g".to_string(), 17),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_04() {
        let code = "a^b$c - d#h + q%t + !b&(z|t)";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Identifier, "a".to_string(), 0),
            token!(TokenType::Unknown, '^'.to_string(), 1),
            token!(TokenType::Identifier, "b".to_string(), 2),
            token!(TokenType::Unknown, '$'.to_string(), 3),
            token!(TokenType::Identifier, "c".to_string(), 4),
            token!(TokenType::Minus, 6),
            token!(TokenType::Identifier, "d".to_string(), 8),
            token!(TokenType::Unknown, '#'.to_string(), 9),
            token!(TokenType::Identifier, "h".to_string(), 10),
            token!(TokenType::Plus, 12),
            token!(TokenType::Identifier, "q".to_string(), 14),
            token!(TokenType::Percent, 15),
            token!(TokenType::Identifier, "t".to_string(), 16),
            token!(TokenType::Plus, 18),
            token!(TokenType::ExclamationMark, 20),
            token!(TokenType::Identifier, "b".to_string(), 21),
            token!(TokenType::Ampersand, 22),
            token!(TokenType::LeftParenthesis, 23),
            token!(TokenType::Identifier, "z".to_string(), 24),
            token!(TokenType::Pipe, 25),
            token!(TokenType::Identifier, "t".to_string(), 26),
            token!(TokenType::RightParenthesis, 27),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_05() {
        let code = "x + var1 + var_2 + _var_3 + var#4 + var!5 + 6var_ + $7 + ?8";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Identifier, "x".to_string(), 0),
            token!(TokenType::Plus, 2),
            token!(TokenType::Identifier, "var1".to_string(), 4..8),
            token!(TokenType::Plus, 9),
            token!(TokenType::Identifier, "var_2".to_string(), 11..16),
            token!(TokenType::Plus, 17),
            token!(TokenType::Identifier, "_var_3".to_string(), 19..25),
            token!(TokenType::Plus, 26),
            token!(TokenType::Identifier, "var".to_string(), 28..31),
            token!(TokenType::Unknown, '#'.to_string(), 31),
            token!(TokenType::Number, "4".to_string(), 32),
            token!(TokenType::Plus, 34),
            token!(TokenType::Identifier, "var".to_string(), 36..39),
            token!(TokenType::ExclamationMark, 39),
            token!(TokenType::Number, "5".to_string(), 40),
            token!(TokenType::Plus, 42),
            token!(TokenType::Number, "6".to_string(), 44),
            token!(TokenType::Identifier, "var_".to_string(), 45..49),
            token!(TokenType::Plus, 50),
            token!(TokenType::Unknown, '$'.to_string(), 52),
            token!(TokenType::Number, "7".to_string(), 53),
            token!(TokenType::Plus, 55),
            token!(TokenType::Unknown, '?'.to_string(), 57),
            token!(TokenType::Number, "8".to_string(), 58),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_06() {
        let code = "125 + 2nb - 0xAB * 0x0R + 0b010 * 0b20 + ABh * 0Rh + 010b*20b";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Number, "125".to_string(), 0..3),
            token!(TokenType::Plus, 4),
            token!(TokenType::Number, "2".to_string(), 6),
            token!(TokenType::Identifier, "nb".to_string(), 7..9),
            token!(TokenType::Minus, 10),
            token!(TokenType::Number, "0".to_string(), 12),
            token!(TokenType::Identifier, "xAB".to_string(), 13..16),
            token!(TokenType::Asterisk, 17),
            token!(TokenType::Number, "0".to_string(), 19),
            token!(TokenType::Identifier, "x0R".to_string(), 20..23),
            token!(TokenType::Plus, 24),
            token!(TokenType::Number, "0".to_string(), 26),
            token!(TokenType::Identifier, "b010".to_string(), 27..31),
            token!(TokenType::Asterisk, 32),
            token!(TokenType::Number, "0".to_string(), 34),
            token!(TokenType::Identifier, "b20".to_string(), 35..38),
            token!(TokenType::Plus, 39),
            token!(TokenType::Identifier, "ABh".to_string(), 41..44),
            token!(TokenType::Asterisk, 45),
            token!(TokenType::Number, "0".to_string(), 47),
            token!(TokenType::Identifier, "Rh".to_string(), 48..50),
            token!(TokenType::Plus, 51),
            token!(TokenType::Number, "010".to_string(), 53..56),
            token!(TokenType::Identifier, "b".to_string(), 56),
            token!(TokenType::Asterisk, 57),
            token!(TokenType::Number, "20".to_string(), 58..60),
            token!(TokenType::Identifier, "b".to_string(), 60),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_07() {
        let code = "0.71/0.72.3 + .3 + 127.0.0.1*8. + 6.07ab - 9f.89hgt";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Number, "0".to_string(), 0),
            token!(TokenType::Dot, 1),
            token!(TokenType::Number, "71".to_string(), 2..4),
            token!(TokenType::Slash, 4),
            token!(TokenType::Number, "0".to_string(), 5),
            token!(TokenType::Dot, 6),
            token!(TokenType::Number, "72".to_string(), 7..9),
            token!(TokenType::Dot, 9),
            token!(TokenType::Number, "3".to_string(), 10),
            token!(TokenType::Plus, 12),
            token!(TokenType::Dot, 14),
            token!(TokenType::Number, "3".to_string(), 15),
            token!(TokenType::Plus, 17),
            token!(TokenType::Number, "127".to_string(), 19..22),
            token!(TokenType::Dot, 22),
            token!(TokenType::Number, "0".to_string(), 23),
            token!(TokenType::Dot, 24),
            token!(TokenType::Number, "0".to_string(), 25),
            token!(TokenType::Dot, 26),
            token!(TokenType::Number, "1".to_string(), 27),
            token!(TokenType::Asterisk, 28),
            token!(TokenType::Number, "8".to_string(), 29),
            token!(TokenType::Dot, 30),
            token!(TokenType::Plus, 32),
            token!(TokenType::Number, "6".to_string(), 34),
            token!(TokenType::Dot, 35),
            token!(TokenType::Number, "07".to_string(), 36..38),
            token!(TokenType::Identifier, "ab".to_string(), 38..40),
            token!(TokenType::Minus, 41),
            token!(TokenType::Number, "9".to_string(), 43),
            token!(TokenType::Identifier, "f".to_string(), 44),
            token!(TokenType::Dot, 45),
            token!(TokenType::Number, "89".to_string(), 46..48),
            token!(TokenType::Identifier, "hgt".to_string(), 48..51),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_08() {
        let code = ")a+b( -(g+h)(g-k))*()) + (-b(t-2*x*(5) + A[7][2-x]";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::RightParenthesis, 0),
            token!(TokenType::Identifier, "a".to_string(), 1),
            token!(TokenType::Plus, 2),
            token!(TokenType::Identifier, "b".to_string(), 3),
            token!(TokenType::LeftParenthesis, 4),
            token!(TokenType::Minus, 6),
            token!(TokenType::LeftParenthesis, 7),
            token!(TokenType::Identifier, "g".to_string(), 8),
            token!(TokenType::Plus, 9),
            token!(TokenType::Identifier, "h".to_string(), 10),
            token!(TokenType::RightParenthesis, 11),
            token!(TokenType::LeftParenthesis, 12),
            token!(TokenType::Identifier, "g".to_string(), 13),
            token!(TokenType::Minus, 14),
            token!(TokenType::Identifier, "k".to_string(), 15),
            token!(TokenType::RightParenthesis, 16),
            token!(TokenType::RightParenthesis, 17),
            token!(TokenType::Asterisk, 18),
            token!(TokenType::LeftParenthesis, 19),
            token!(TokenType::RightParenthesis, 20),
            token!(TokenType::RightParenthesis, 21),
            token!(TokenType::Plus, 23),
            token!(TokenType::LeftParenthesis, 25),
            token!(TokenType::Minus, 26),
            token!(TokenType::Identifier, "b".to_string(), 27),
            token!(TokenType::LeftParenthesis, 28),
            token!(TokenType::Identifier, "t".to_string(), 29),
            token!(TokenType::Minus, 30),
            token!(TokenType::Number, "2".to_string(), 31),
            token!(TokenType::Asterisk, 32),
            token!(TokenType::Identifier, "x".to_string(), 33),
            token!(TokenType::Asterisk, 34),
            token!(TokenType::LeftParenthesis, 35),
            token!(TokenType::Number, "5".to_string(), 36),
            token!(TokenType::RightParenthesis, 37),
            token!(TokenType::Plus, 39),
            token!(TokenType::Identifier, "A".to_string(), 41),
            token!(TokenType::LeftBracket, 42),
            token!(TokenType::Number, "7".to_string(), 43),
            token!(TokenType::RightBracket, 44),
            token!(TokenType::LeftBracket, 45),
            token!(TokenType::Number, "2".to_string(), 46),
            token!(TokenType::Minus, 47),
            token!(TokenType::Identifier, "x".to_string(), 48),
            token!(TokenType::RightBracket, 49),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_09() {
        let code = "2(t) - f2(t) + g()/h(2, )*func(-t/q, f(4-t), -(x+2)*(y-2))";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Number, "2".to_string(), 0),
            token!(TokenType::LeftParenthesis, 1),
            token!(TokenType::Identifier, "t".to_string(), 2),
            token!(TokenType::RightParenthesis, 3),
            token!(TokenType::Minus, 5),
            token!(TokenType::Identifier, "f2".to_string(), 7..9),
            token!(TokenType::LeftParenthesis, 9),
            token!(TokenType::Identifier, "t".to_string(), 10),
            token!(TokenType::RightParenthesis, 11),
            token!(TokenType::Plus, 13),
            token!(TokenType::Identifier, "g".to_string(), 15),
            token!(TokenType::LeftParenthesis, 16),
            token!(TokenType::RightParenthesis, 17),
            token!(TokenType::Slash, 18),
            token!(TokenType::Identifier, "h".to_string(), 19),
            token!(TokenType::LeftParenthesis, 20),
            token!(TokenType::Number, "2".to_string(), 21),
            token!(TokenType::Comma, 22),
            token!(TokenType::RightParenthesis, 24),
            token!(TokenType::Asterisk, 25),
            token!(TokenType::Identifier, "func".to_string(), 26..30),
            token!(TokenType::LeftParenthesis, 30),
            token!(TokenType::Minus, 31),
            token!(TokenType::Identifier, "t".to_string(), 32),
            token!(TokenType::Slash, 33),
            token!(TokenType::Identifier, "q".to_string(), 34),
            token!(TokenType::Comma, 35),
            token!(TokenType::Identifier, "f".to_string(), 37),
            token!(TokenType::LeftParenthesis, 38),
            token!(TokenType::Number, "4".to_string(), 39),
            token!(TokenType::Minus, 40),
            token!(TokenType::Identifier, "t".to_string(), 41),
            token!(TokenType::RightParenthesis, 42),
            token!(TokenType::Comma, 43),
            token!(TokenType::Minus, 45),
            token!(TokenType::LeftParenthesis, 46),
            token!(TokenType::Identifier, "x".to_string(), 47),
            token!(TokenType::Plus, 48),
            token!(TokenType::Number, "2".to_string(), 49),
            token!(TokenType::RightParenthesis, 50),
            token!(TokenType::Asterisk, 51),
            token!(TokenType::LeftParenthesis, 52),
            token!(TokenType::Identifier, "y".to_string(), 53),
            token!(TokenType::Minus, 54),
            token!(TokenType::Number, "2".to_string(), 55),
            token!(TokenType::RightParenthesis, 56),
            token!(TokenType::RightParenthesis, 57),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_10() {
        let code = "/a*b**c + m)*a*b + a*c - a*smn(j*k/m + m";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Slash, 0),
            token!(TokenType::Identifier, "a".to_string(), 1),
            token!(TokenType::Asterisk, 2),
            token!(TokenType::Identifier, "b".to_string(), 3),
            token!(TokenType::Asterisk, 4),
            token!(TokenType::Asterisk, 5),
            token!(TokenType::Identifier, "c".to_string(), 6),
            token!(TokenType::Plus, 8),
            token!(TokenType::Identifier, "m".to_string(), 10),
            token!(TokenType::RightParenthesis, 11),
            token!(TokenType::Asterisk, 12),
            token!(TokenType::Identifier, "a".to_string(), 13),
            token!(TokenType::Asterisk, 14),
            token!(TokenType::Identifier, "b".to_string(), 15),
            token!(TokenType::Plus, 17),
            token!(TokenType::Identifier, "a".to_string(), 19),
            token!(TokenType::Asterisk, 20),
            token!(TokenType::Identifier, "c".to_string(), 21),
            token!(TokenType::Minus, 23),
            token!(TokenType::Identifier, "a".to_string(), 25),
            token!(TokenType::Asterisk, 26),
            token!(TokenType::Identifier, "smn".to_string(), 27..30),
            token!(TokenType::LeftParenthesis, 30),
            token!(TokenType::Identifier, "j".to_string(), 31),
            token!(TokenType::Asterisk, 32),
            token!(TokenType::Identifier, "k".to_string(), 33),
            token!(TokenType::Slash, 34),
            token!(TokenType::Identifier, "m".to_string(), 35),
            token!(TokenType::Plus, 37),
            token!(TokenType::Identifier, "m".to_string(), 39),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_11() {
        let code =
            "-cos(-&t))/(*(*f)(127.0.0.1, \"/dev/null\", (t==0)?4more_errors:b^2) - .5";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Minus, 0),
            token!(TokenType::Identifier, "cos".to_string(), 1..4),
            token!(TokenType::LeftParenthesis, 4),
            token!(TokenType::Minus, 5),
            token!(TokenType::Ampersand, 6),
            token!(TokenType::Identifier, "t".to_string(), 7),
            token!(TokenType::RightParenthesis, 8),
            token!(TokenType::RightParenthesis, 9),
            token!(TokenType::Slash, 10),
            token!(TokenType::LeftParenthesis, 11),
            token!(TokenType::Asterisk, 12),
            token!(TokenType::LeftParenthesis, 13),
            token!(TokenType::Asterisk, 14),
            token!(TokenType::Identifier, "f".to_string(), 15),
            token!(TokenType::RightParenthesis, 16),
            token!(TokenType::LeftParenthesis, 17),
            token!(TokenType::Number, "127".to_string(), 18..21),
            token!(TokenType::Dot, 21),
            token!(TokenType::Number, "0".to_string(), 22),
            token!(TokenType::Dot, 23),
            token!(TokenType::Number, "0".to_string(), 24),
            token!(TokenType::Dot, 25),
            token!(TokenType::Number, "1".to_string(), 26),
            token!(TokenType::Comma, 27),
            token!(TokenType::QuotationMark, 29),
            token!(TokenType::Slash, 30),
            token!(TokenType::Identifier, "dev".to_string(), 31..34),
            token!(TokenType::Slash, 34),
            token!(TokenType::Identifier, "null".to_string(), 35..39),
            token!(TokenType::QuotationMark, 39),
            token!(TokenType::Comma, 40),
            token!(TokenType::LeftParenthesis, 42),
            token!(TokenType::Identifier, "t".to_string(), 43),
            token!(TokenType::Unknown, '='.to_string(), 44),
            token!(TokenType::Unknown, '='.to_string(), 45),
            token!(TokenType::Number, "0".to_string(), 46),
            token!(TokenType::RightParenthesis, 47),
            token!(TokenType::Unknown, '?'.to_string(), 48),
            token!(TokenType::Number, "4".to_string(), 49),
            token!(TokenType::Identifier, "more_errors".to_string(), 50..61),
            token!(TokenType::Unknown, ':'.to_string(), 61),
            token!(TokenType::Identifier, "b".to_string(), 62),
            token!(TokenType::Unknown, '^'.to_string(), 63),
            token!(TokenType::Number, "2".to_string(), 64),
            token!(TokenType::RightParenthesis, 65),
            token!(TokenType::Minus, 67),
            token!(TokenType::Dot, 69),
            token!(TokenType::Number, "5".to_string(), 70),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_12() {
        let code = "//(*0)- an*0p(a+b)-1.000.5//6(*f(-b, 1.8-0*(2-6) %1 + (++a)/(6x^2+4x-1) + d/dt*(smn(at+q)/(4cos(at)-ht^2)";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Slash, 0),
            token!(TokenType::Slash, 1),
            token!(TokenType::LeftParenthesis, 2),
            token!(TokenType::Asterisk, 3),
            token!(TokenType::Number, "0".to_string(), 4),
            token!(TokenType::RightParenthesis, 5),
            token!(TokenType::Minus, 6),
            token!(TokenType::Identifier, "an".to_string(), 8..10),
            token!(TokenType::Asterisk, 10),
            token!(TokenType::Number, "0".to_string(), 11),
            token!(TokenType::Identifier, "p".to_string(), 12),
            token!(TokenType::LeftParenthesis, 13),
            token!(TokenType::Identifier, "a".to_string(), 14),
            token!(TokenType::Plus, 15),
            token!(TokenType::Identifier, "b".to_string(), 16),
            token!(TokenType::RightParenthesis, 17),
            token!(TokenType::Minus, 18),
            token!(TokenType::Number, "1".to_string(), 19),
            token!(TokenType::Dot, 20),
            token!(TokenType::Number, "000".to_string(), 21..24),
            token!(TokenType::Dot, 24),
            token!(TokenType::Number, "5".to_string(), 25),
            token!(TokenType::Slash, 26),
            token!(TokenType::Slash, 27),
            token!(TokenType::Number, "6".to_string(), 28),
            token!(TokenType::LeftParenthesis, 29),
            token!(TokenType::Asterisk, 30),
            token!(TokenType::Identifier, "f".to_string(), 31),
            token!(TokenType::LeftParenthesis, 32),
            token!(TokenType::Minus, 33),
            token!(TokenType::Identifier, "b".to_string(), 34),
            token!(TokenType::Comma, 35),
            token!(TokenType::Number, "1".to_string(), 37),
            token!(TokenType::Dot, 38),
            token!(TokenType::Number, "8".to_string(), 39),
            token!(TokenType::Minus, 40),
            token!(TokenType::Number, "0".to_string(), 41),
            token!(TokenType::Asterisk, 42),
            token!(TokenType::LeftParenthesis, 43),
            token!(TokenType::Number, "2".to_string(), 44),
            token!(TokenType::Minus, 45),
            token!(TokenType::Number, "6".to_string(), 46),
            token!(TokenType::RightParenthesis, 47),
            token!(TokenType::Percent, 49),
            token!(TokenType::Number, "1".to_string(), 50),
            token!(TokenType::Plus, 52),
            token!(TokenType::LeftParenthesis, 54),
            token!(TokenType::Plus, 55),
            token!(TokenType::Plus, 56),
            token!(TokenType::Identifier, "a".to_string(), 57),
            token!(TokenType::RightParenthesis, 58),
            token!(TokenType::Slash, 59),
            token!(TokenType::LeftParenthesis, 60),
            token!(TokenType::Number, "6".to_string(), 61),
            token!(TokenType::Identifier, "x".to_string(), 62),
            token!(TokenType::Unknown, '^'.to_string(), 63),
            token!(TokenType::Number, "2".to_string(), 64),
            token!(TokenType::Plus, 65),
            token!(TokenType::Number, "4".to_string(), 66),
            token!(TokenType::Identifier, "x".to_string(), 67),
            token!(TokenType::Minus, 68),
            token!(TokenType::Number, "1".to_string(), 69),
            token!(TokenType::RightParenthesis, 70),
            token!(TokenType::Plus, 72),
            token!(TokenType::Identifier, "d".to_string(), 74),
            token!(TokenType::Slash, 75),
            token!(TokenType::Identifier, "dt".to_string(), 76..78),
            token!(TokenType::Asterisk, 78),
            token!(TokenType::LeftParenthesis, 79),
            token!(TokenType::Identifier, "smn".to_string(), 80..83),
            token!(TokenType::LeftParenthesis, 83),
            token!(TokenType::Identifier, "at".to_string(), 84..86),
            token!(TokenType::Plus, 86),
            token!(TokenType::Identifier, "q".to_string(), 87),
            token!(TokenType::RightParenthesis, 88),
            token!(TokenType::Slash, 89),
            token!(TokenType::LeftParenthesis, 90),
            token!(TokenType::Number, "4".to_string(), 91),
            token!(TokenType::Identifier, "cos".to_string(), 92..95),
            token!(TokenType::LeftParenthesis, 95),
            token!(TokenType::Identifier, "at".to_string(), 96..98),
            token!(TokenType::RightParenthesis, 98),
            token!(TokenType::Minus, 99),
            token!(TokenType::Identifier, "ht".to_string(), 100..102),
            token!(TokenType::Unknown, '^'.to_string(), 102),
            token!(TokenType::Number, "2".to_string(), 103),
            token!(TokenType::RightParenthesis, 104),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_13() {
        let code = "-(-5x((int*)exp())/t - 3.14.15k/(2x^2-5x-1)*y - A[N*(i++)+j]";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Minus, 0),
            token!(TokenType::LeftParenthesis, 1),
            token!(TokenType::Minus, 2),
            token!(TokenType::Number, "5".to_string(), 3),
            token!(TokenType::Identifier, "x".to_string(), 4),
            token!(TokenType::LeftParenthesis, 5),
            token!(TokenType::LeftParenthesis, 6),
            token!(TokenType::Identifier, "int".to_string(), 7..10),
            token!(TokenType::Asterisk, 10),
            token!(TokenType::RightParenthesis, 11),
            token!(TokenType::Identifier, "exp".to_string(), 12..15),
            token!(TokenType::LeftParenthesis, 15),
            token!(TokenType::RightParenthesis, 16),
            token!(TokenType::RightParenthesis, 17),
            token!(TokenType::Slash, 18),
            token!(TokenType::Identifier, "t".to_string(), 19),
            token!(TokenType::Minus, 21),
            token!(TokenType::Number, "3".to_string(), 23),
            token!(TokenType::Dot, 24),
            token!(TokenType::Number, "14".to_string(), 25..27),
            token!(TokenType::Dot, 27),
            token!(TokenType::Number, "15".to_string(), 28..30),
            token!(TokenType::Identifier, "k".to_string(), 30),
            token!(TokenType::Slash, 31),
            token!(TokenType::LeftParenthesis, 32),
            token!(TokenType::Number, "2".to_string(), 33),
            token!(TokenType::Identifier, "x".to_string(), 34),
            token!(TokenType::Unknown, '^'.to_string(), 35),
            token!(TokenType::Number, "2".to_string(), 36),
            token!(TokenType::Minus, 37),
            token!(TokenType::Number, "5".to_string(), 38),
            token!(TokenType::Identifier, "x".to_string(), 39),
            token!(TokenType::Minus, 40),
            token!(TokenType::Number, "1".to_string(), 41),
            token!(TokenType::RightParenthesis, 42),
            token!(TokenType::Asterisk, 43),
            token!(TokenType::Identifier, "y".to_string(), 44),
            token!(TokenType::Minus, 46),
            token!(TokenType::Identifier, "A".to_string(), 48),
            token!(TokenType::LeftBracket, 49),
            token!(TokenType::Identifier, "N".to_string(), 50),
            token!(TokenType::Asterisk, 51),
            token!(TokenType::LeftParenthesis, 52),
            token!(TokenType::Identifier, "i".to_string(), 53),
            token!(TokenType::Plus, 54),
            token!(TokenType::Plus, 55),
            token!(TokenType::RightParenthesis, 56),
            token!(TokenType::Plus, 57),
            token!(TokenType::Identifier, "j".to_string(), 58),
            token!(TokenType::RightBracket, 59),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_14() {
        let code = "-(-exp(3et/4.0.2, 2i-1)/L + )((void*)*f()) + ((i++) + (++i/(i--))/k//) + 6.000.500.5";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Minus, 0),
            token!(TokenType::LeftParenthesis, 1),
            token!(TokenType::Minus, 2),
            token!(TokenType::Identifier, "exp".to_string(), 3..6),
            token!(TokenType::LeftParenthesis, 6),
            token!(TokenType::Number, "3".to_string(), 7),
            token!(TokenType::Identifier, "et".to_string(), 8..10),
            token!(TokenType::Slash, 10),
            token!(TokenType::Number, "4".to_string(), 11),
            token!(TokenType::Dot, 12),
            token!(TokenType::Number, "0".to_string(), 13),
            token!(TokenType::Dot, 14),
            token!(TokenType::Number, "2".to_string(), 15),
            token!(TokenType::Comma, 16),
            token!(TokenType::Number, "2".to_string(), 18),
            token!(TokenType::Identifier, "i".to_string(), 19),
            token!(TokenType::Minus, 20),
            token!(TokenType::Number, "1".to_string(), 21),
            token!(TokenType::RightParenthesis, 22),
            token!(TokenType::Slash, 23),
            token!(TokenType::Identifier, "L".to_string(), 24),
            token!(TokenType::Plus, 26),
            token!(TokenType::RightParenthesis, 28),
            token!(TokenType::LeftParenthesis, 29),
            token!(TokenType::LeftParenthesis, 30),
            token!(TokenType::Identifier, "void".to_string(), 31..35),
            token!(TokenType::Asterisk, 35),
            token!(TokenType::RightParenthesis, 36),
            token!(TokenType::Asterisk, 37),
            token!(TokenType::Identifier, "f".to_string(), 38),
            token!(TokenType::LeftParenthesis, 39),
            token!(TokenType::RightParenthesis, 40),
            token!(TokenType::RightParenthesis, 41),
            token!(TokenType::Plus, 43),
            token!(TokenType::LeftParenthesis, 45),
            token!(TokenType::LeftParenthesis, 46),
            token!(TokenType::Identifier, "i".to_string(), 47),
            token!(TokenType::Plus, 48),
            token!(TokenType::Plus, 49),
            token!(TokenType::RightParenthesis, 50),
            token!(TokenType::Plus, 52),
            token!(TokenType::LeftParenthesis, 54),
            token!(TokenType::Plus, 55),
            token!(TokenType::Plus, 56),
            token!(TokenType::Identifier, "i".to_string(), 57),
            token!(TokenType::Slash, 58),
            token!(TokenType::LeftParenthesis, 59),
            token!(TokenType::Identifier, "i".to_string(), 60),
            token!(TokenType::Minus, 61),
            token!(TokenType::Minus, 62),
            token!(TokenType::RightParenthesis, 63),
            token!(TokenType::RightParenthesis, 64),
            token!(TokenType::Slash, 65),
            token!(TokenType::Identifier, "k".to_string(), 66),
            token!(TokenType::Slash, 67),
            token!(TokenType::Slash, 68),
            token!(TokenType::RightParenthesis, 69),
            token!(TokenType::Plus, 71),
            token!(TokenType::Number, "6".to_string(), 73),
            token!(TokenType::Dot, 74),
            token!(TokenType::Number, "000".to_string(), 75..78),
            token!(TokenType::Dot, 78),
            token!(TokenType::Number, "500".to_string(), 79..82),
            token!(TokenType::Dot, 82),
            token!(TokenType::Number, "5".to_string(), 83),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_15() {
        let code = "**f(*k, -p+1, ))2.1.1 + 1.8q((-5x ++ i)";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Asterisk, 0),
            token!(TokenType::Asterisk, 1),
            token!(TokenType::Identifier, "f".to_string(), 2),
            token!(TokenType::LeftParenthesis, 3),
            token!(TokenType::Asterisk, 4),
            token!(TokenType::Identifier, "k".to_string(), 5),
            token!(TokenType::Comma, 6),
            token!(TokenType::Minus, 8),
            token!(TokenType::Identifier, "p".to_string(), 9),
            token!(TokenType::Plus, 10),
            token!(TokenType::Number, "1".to_string(), 11),
            token!(TokenType::Comma, 12),
            token!(TokenType::RightParenthesis, 14),
            token!(TokenType::RightParenthesis, 15),
            token!(TokenType::Number, "2".to_string(), 16),
            token!(TokenType::Dot, 17),
            token!(TokenType::Number, "1".to_string(), 18),
            token!(TokenType::Dot, 19),
            token!(TokenType::Number, "1".to_string(), 20),
            token!(TokenType::Plus, 22),
            token!(TokenType::Number, "1".to_string(), 24),
            token!(TokenType::Dot, 25),
            token!(TokenType::Number, "8".to_string(), 26),
            token!(TokenType::Identifier, "q".to_string(), 27),
            token!(TokenType::LeftParenthesis, 28),
            token!(TokenType::LeftParenthesis, 29),
            token!(TokenType::Minus, 30),
            token!(TokenType::Number, "5".to_string(), 31),
            token!(TokenType::Identifier, "x".to_string(), 32),
            token!(TokenType::Plus, 34),
            token!(TokenType::Plus, 35),
            token!(TokenType::Identifier, "i".to_string(), 37),
            token!(TokenType::RightParenthesis, 38),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_16() {
        let code = "/.1(2x^2-5x+7)-(-i)+ (j++)/0 - )(*f)(2, 7-x, )/q + send(-(2x+7)/A[j, i], 127.0.0.1 ) + )/";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Slash, 0),
            token!(TokenType::Dot, 1),
            token!(TokenType::Number, "1".to_string(), 2),
            token!(TokenType::LeftParenthesis, 3),
            token!(TokenType::Number, "2".to_string(), 4),
            token!(TokenType::Identifier, "x".to_string(), 5),
            token!(TokenType::Unknown, '^'.to_string(), 6),
            token!(TokenType::Number, "2".to_string(), 7),
            token!(TokenType::Minus, 8),
            token!(TokenType::Number, "5".to_string(), 9),
            token!(TokenType::Identifier, "x".to_string(), 10),
            token!(TokenType::Plus, 11),
            token!(TokenType::Number, "7".to_string(), 12),
            token!(TokenType::RightParenthesis, 13),
            token!(TokenType::Minus, 14),
            token!(TokenType::LeftParenthesis, 15),
            token!(TokenType::Minus, 16),
            token!(TokenType::Identifier, "i".to_string(), 17),
            token!(TokenType::RightParenthesis, 18),
            token!(TokenType::Plus, 19),
            token!(TokenType::LeftParenthesis, 21),
            token!(TokenType::Identifier, "j".to_string(), 22),
            token!(TokenType::Plus, 23),
            token!(TokenType::Plus, 24),
            token!(TokenType::RightParenthesis, 25),
            token!(TokenType::Slash, 26),
            token!(TokenType::Number, "0".to_string(), 27),
            token!(TokenType::Minus, 29),
            token!(TokenType::RightParenthesis, 31),
            token!(TokenType::LeftParenthesis, 32),
            token!(TokenType::Asterisk, 33),
            token!(TokenType::Identifier, "f".to_string(), 34),
            token!(TokenType::RightParenthesis, 35),
            token!(TokenType::LeftParenthesis, 36),
            token!(TokenType::Number, "2".to_string(), 37),
            token!(TokenType::Comma, 38),
            token!(TokenType::Number, "7".to_string(), 40),
            token!(TokenType::Minus, 41),
            token!(TokenType::Identifier, "x".to_string(), 42),
            token!(TokenType::Comma, 43),
            token!(TokenType::RightParenthesis, 45),
            token!(TokenType::Slash, 46),
            token!(TokenType::Identifier, "q".to_string(), 47),
            token!(TokenType::Plus, 49),
            token!(TokenType::Identifier, "send".to_string(), 51..55),
            token!(TokenType::LeftParenthesis, 55),
            token!(TokenType::Minus, 56),
            token!(TokenType::LeftParenthesis, 57),
            token!(TokenType::Number, "2".to_string(), 58),
            token!(TokenType::Identifier, "x".to_string(), 59),
            token!(TokenType::Plus, 60),
            token!(TokenType::Number, "7".to_string(), 61),
            token!(TokenType::RightParenthesis, 62),
            token!(TokenType::Slash, 63),
            token!(TokenType::Identifier, "A".to_string(), 64),
            token!(TokenType::LeftBracket, 65),
            token!(TokenType::Identifier, "j".to_string(), 66),
            token!(TokenType::Comma, 67),
            token!(TokenType::Identifier, "i".to_string(), 69),
            token!(TokenType::RightBracket, 70),
            token!(TokenType::Comma, 71),
            token!(TokenType::Number, "127".to_string(), 73..76),
            token!(TokenType::Dot, 76),
            token!(TokenType::Number, "0".to_string(), 77),
            token!(TokenType::Dot, 78),
            token!(TokenType::Number, "0".to_string(), 79),
            token!(TokenType::Dot, 80),
            token!(TokenType::Number, "1".to_string(), 81),
            token!(TokenType::RightParenthesis, 83),
            token!(TokenType::Plus, 85),
            token!(TokenType::RightParenthesis, 87),
            token!(TokenType::Slash, 88),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }

    #[test]
    fn test_tokenize_17() {
        let code =
            "*101*1#(t-q)(t+q)//dt - (int*)f(8t, -(k/h)A[i+6.]), exp(), ))(t-k*8.00.1/.0";

        let tokens_actual = tokenize(code);
        let tokens_expected = vec![
            token!(TokenType::Asterisk, 0),
            token!(TokenType::Number, "101".to_string(), 1..4),
            token!(TokenType::Asterisk, 4),
            token!(TokenType::Number, "1".to_string(), 5),
            token!(TokenType::Unknown, '#'.to_string(), 6),
            token!(TokenType::LeftParenthesis, 7),
            token!(TokenType::Identifier, "t".to_string(), 8),
            token!(TokenType::Minus, 9),
            token!(TokenType::Identifier, "q".to_string(), 10),
            token!(TokenType::RightParenthesis, 11),
            token!(TokenType::LeftParenthesis, 12),
            token!(TokenType::Identifier, "t".to_string(), 13),
            token!(TokenType::Plus, 14),
            token!(TokenType::Identifier, "q".to_string(), 15),
            token!(TokenType::RightParenthesis, 16),
            token!(TokenType::Slash, 17),
            token!(TokenType::Slash, 18),
            token!(TokenType::Identifier, "dt".to_string(), 19..21),
            token!(TokenType::Minus, 22),
            token!(TokenType::LeftParenthesis, 24),
            token!(TokenType::Identifier, "int".to_string(), 25..28),
            token!(TokenType::Asterisk, 28),
            token!(TokenType::RightParenthesis, 29),
            token!(TokenType::Identifier, "f".to_string(), 30),
            token!(TokenType::LeftParenthesis, 31),
            token!(TokenType::Number, "8".to_string(), 32),
            token!(TokenType::Identifier, "t".to_string(), 33),
            token!(TokenType::Comma, 34),
            token!(TokenType::Minus, 36),
            token!(TokenType::LeftParenthesis, 37),
            token!(TokenType::Identifier, "k".to_string(), 38),
            token!(TokenType::Slash, 39),
            token!(TokenType::Identifier, "h".to_string(), 40),
            token!(TokenType::RightParenthesis, 41),
            token!(TokenType::Identifier, "A".to_string(), 42),
            token!(TokenType::LeftBracket, 43),
            token!(TokenType::Identifier, "i".to_string(), 44),
            token!(TokenType::Plus, 45),
            token!(TokenType::Number, "6".to_string(), 46),
            token!(TokenType::Dot, 47),
            token!(TokenType::RightBracket, 48),
            token!(TokenType::RightParenthesis, 49),
            token!(TokenType::Comma, 50),
            token!(TokenType::Identifier, "exp".to_string(), 52..55),
            token!(TokenType::LeftParenthesis, 55),
            token!(TokenType::RightParenthesis, 56),
            token!(TokenType::Comma, 57),
            token!(TokenType::RightParenthesis, 59),
            token!(TokenType::RightParenthesis, 60),
            token!(TokenType::LeftParenthesis, 61),
            token!(TokenType::Identifier, "t".to_string(), 62),
            token!(TokenType::Minus, 63),
            token!(TokenType::Identifier, "k".to_string(), 64),
            token!(TokenType::Asterisk, 65),
            token!(TokenType::Number, "8".to_string(), 66),
            token!(TokenType::Dot, 67),
            token!(TokenType::Number, "00".to_string(), 68..70),
            token!(TokenType::Dot, 70),
            token!(TokenType::Number, "1".to_string(), 71),
            token!(TokenType::Slash, 72),
            token!(TokenType::Dot, 73),
            token!(TokenType::Number, "0".to_string(), 74),
        ];

        assert_eq!(tokens_actual, tokens_expected);
    }
}
