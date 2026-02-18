use crate::compiler::lexer::Lexeme;
use colored::Colorize;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractSyntaxTree {
    pub peek: AstNode,
}

impl AbstractSyntaxTree {
    pub fn from_node(node: AstNode) -> Self {
        Self { peek: node }
    }

    pub fn pretty_print(&self) -> String {
        let mut tree = String::new();
        Self::print_recursive(&self.peek, &mut tree, "".to_string(), true);
        tree
    }

    fn print_recursive(node: &AstNode, tree: &mut String, prefix: String, is_last: bool) {
        let connector = if is_last { "└── " } else { "├── " };

        tree.push_str(&format!("{}{}", prefix.dimmed(), connector.dimmed()));

        let node_text = match node {
            AstNode::Number(n) => format!("{n:.3}").bright_blue(),
            AstNode::Identifier(s) => s.to_string().green(),
            AstNode::StringLiteral(s) => format!("\"{}\"", s).bright_magenta(),
            AstNode::UnaryOperation { operation, .. } => {
                operation.to_string().yellow().bold()
            },
            AstNode::BinaryOperation { operation, .. } => {
                operation.to_string().yellow().bold()
            },
            AstNode::FunctionCall { name, .. } => format!("{}(...)", name).cyan().bold(),
            AstNode::ArrayAccess { identifier, .. } => {
                format!("{}[...]", identifier).blue().bold()
            },
        };
        tree.push_str(&format!("{}\n", node_text));

        let new_prefix = prefix + if is_last { "    " } else { "│   " };

        match node {
            AstNode::Number(_) | AstNode::Identifier(_) | AstNode::StringLiteral(_) => {},

            AstNode::UnaryOperation { expression, .. } => {
                Self::print_recursive(expression, tree, new_prefix, true);
            },

            AstNode::BinaryOperation { left, right, .. } => {
                Self::print_recursive(left, tree, new_prefix.clone(), false);
                Self::print_recursive(right, tree, new_prefix, true);
            },

            AstNode::FunctionCall { arguments, .. } => {
                let arg_count = arguments.len();
                for (i, arg) in arguments.iter().enumerate() {
                    let is_last_arg = i == arg_count - 1;
                    Self::print_recursive(arg, tree, new_prefix.clone(), is_last_arg);
                }
            },

            AstNode::ArrayAccess {
                identifier: _,
                indices,
            } => {
                let dimensions = indices.len();
                for (i, index) in indices.iter().enumerate() {
                    let is_last_arg = i == dimensions - 1;
                    Self::print_recursive(index, tree, new_prefix.clone(), is_last_arg);
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Number(f64),
    Identifier(String),
    StringLiteral(String),
    UnaryOperation {
        operation: UnaryOperationKind,
        expression: Box<AstNode>,
    },
    BinaryOperation {
        operation: BinaryOperationKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<AstNode>,
    },
    ArrayAccess {
        identifier: String,
        indices: Vec<AstNode>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperationKind {
    Minus,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperationKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Or,
    And,
}

pub struct AstParser {
    lexemes: Vec<Lexeme>,
    current_index: usize,
}

impl AstParser {
    pub fn new(lexemes: Vec<Lexeme>) -> Self {
        Self {
            lexemes,
            current_index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<AbstractSyntaxTree, AstError> {
        let node = self.parse_logical_or()?;

        if self.peek().is_some()
            && let Some(peek) = self.consume()
        {
            Err(AstError::NotExpectedLexeme(peek.clone()))
        } else {
            Ok(AbstractSyntaxTree { peek: node })
        }
    }

    fn parse_logical_or(&mut self) -> Result<AstNode, AstError> {
        let mut left_node = self.parse_logical_and()?;

        while let Some(Lexeme::Or) = self.peek()
            && let Some(_) = self.consume()
        {
            let right_node = self.parse_logical_and()?;
            left_node = AstNode::BinaryOperation {
                operation: BinaryOperationKind::Or,
                left: Box::new(left_node),
                right: Box::new(right_node),
            };
        }
        Ok(left_node)
    }

    fn parse_logical_and(&mut self) -> Result<AstNode, AstError> {
        let mut left_node = self.parse_expression()?;

        while let Some(Lexeme::And) = self.peek()
            && let Some(_) = self.consume()
        {
            let right_node = self.parse_expression()?;
            left_node = AstNode::BinaryOperation {
                operation: BinaryOperationKind::And,
                left: Box::new(left_node),
                right: Box::new(right_node),
            };
        }
        Ok(left_node)
    }

    fn parse_expression(&mut self) -> Result<AstNode, AstError> {
        let mut left_node = self.parse_term()?;

        while let Some(Lexeme::Plus) | Some(Lexeme::Minus) = self.peek()
            && let Some(lexeme) = self.consume()
        {
            let operation = match lexeme {
                Lexeme::Plus => BinaryOperationKind::Plus,
                Lexeme::Minus => BinaryOperationKind::Minus,
                _ => return Err(AstError::UnreachableLexeme(lexeme.clone())),
            };

            let right_node = self.parse_term()?;

            left_node = AstNode::BinaryOperation {
                operation,
                left: Box::new(left_node),
                right: Box::new(right_node),
            };
        }

        Ok(left_node)
    }

    fn parse_term(&mut self) -> Result<AstNode, AstError> {
        let mut left_node = self.parse_unary()?;

        while let Some(Lexeme::Multiply) | Some(Lexeme::Divide) = self.peek()
            && let Some(lexeme) = self.consume()
        {
            let operation = match lexeme {
                Lexeme::Multiply => BinaryOperationKind::Multiply,
                Lexeme::Divide => BinaryOperationKind::Divide,
                _ => return Err(AstError::UnreachableLexeme(lexeme.clone())),
            };

            let right_node = self.parse_unary()?;

            left_node = AstNode::BinaryOperation {
                operation,
                left: Box::new(left_node),
                right: Box::new(right_node),
            };
        }

        Ok(left_node)
    }

    fn parse_unary(&mut self) -> Result<AstNode, AstError> {
        if let Some(Lexeme::Not) | Some(Lexeme::Minus) = self.peek()
            && let Some(lexeme) = self.consume()
        {
            let operation_kind = match lexeme {
                Lexeme::Not => UnaryOperationKind::Not,
                Lexeme::Minus => UnaryOperationKind::Minus,
                _ => return Err(AstError::UnreachableLexeme(lexeme.clone())),
            };

            let child_node = self.parse_unary()?;

            Ok(AstNode::UnaryOperation {
                operation: operation_kind,
                expression: Box::new(child_node),
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<AstNode, AstError> {
        if let Some(lexeme) = self.consume() {
            match lexeme {
                Lexeme::Number(value) => Ok(AstNode::Number(value)),
                Lexeme::String(value) => {
                    match (matches!(self.peek(), Some(Lexeme::Comma)))
                        || (matches!(self.peek_previous_by(2), Some(Lexeme::Comma)))
                    {
                        true => Ok(AstNode::StringLiteral(value.clone())),
                        false => Err(AstError::StringOutsideFunction(value.clone())),
                    }
                },

                Lexeme::LeftParenthesis => {
                    let inner_node = self.parse_logical_or()?;

                    if self.peek() == Some(&Lexeme::RightParenthesis) {
                        self.consume();
                        Ok(inner_node)
                    } else {
                        Err(AstError::ExpectedRightParenthesis)
                    }
                },

                Lexeme::Identifier(name) => {
                    if self.peek() == Some(&Lexeme::LeftParenthesis)
                        && let Some(_) = self.consume()
                    {
                        let function_name = name.clone();
                        let mut args = Vec::new();

                        if self.peek() != Some(&Lexeme::RightParenthesis) {
                            loop {
                                args.push(self.parse_logical_or()?);

                                let peek = self.peek();

                                if peek == Some(&Lexeme::Comma) {
                                    let _ = self.consume();
                                } else if peek == Some(&Lexeme::RightParenthesis) {
                                    break;
                                } else {
                                    return Err(match peek {
                                        None => AstError::NotExpectedEndOfExpression,
                                        Some(lexeme) => {
                                            AstError::ExpectedCommaOrRightParenthesis(
                                                lexeme.clone(),
                                            )
                                        },
                                    });
                                }
                            }
                        }

                        let _ = self.consume();

                        Ok(AstNode::FunctionCall {
                            name: function_name,
                            arguments: args,
                        })
                    } else if self.peek() == Some(&Lexeme::LeftBracket) {
                        let identifier = name.clone();
                        let mut indices: Vec<AstNode> = Vec::new();

                        loop {
                            let _ = self.consume();
                            let index = self.parse_logical_or()?;
                            if self.peek() == Some(&Lexeme::RightBracket) {
                                let _ = self.consume();
                                indices.push(index);
                                if self.peek() == Some(&Lexeme::LeftBracket) {
                                    continue;
                                } else {
                                    break;
                                }
                            } else {
                                return Err(AstError::ExpectedRightBracket);
                            }
                        }
                        Ok(AstNode::ArrayAccess {
                            identifier,
                            indices,
                        })
                    } else {
                        Ok(AstNode::Identifier(name.clone()))
                    }
                },

                _ => Err(AstError::NotExpectedLexeme(lexeme.clone())),
            }
        } else {
            Err(AstError::NotExpectedEndOfExpression)
        }
    }

    fn consume(&mut self) -> Option<Lexeme> {
        if let Some(lexeme) = self.peek() {
            let lexeme = lexeme.clone();
            self.current_index += 1;
            return Some(lexeme);
        }
        None
    }

    fn peek(&self) -> Option<&Lexeme> {
        self.lexemes.get(self.current_index)
    }

    fn peek_previous_by(&self, by: usize) -> Option<&Lexeme> {
        self.lexemes.get(self.current_index - by)
    }
}

pub fn report_success(tree: &AbstractSyntaxTree) {
    log::warn!(
        "{} {}.",
        "Abstract-Syntax Tree generation",
        "success".bold().green()
    );
    log::info!("{}", tree.pretty_print());
}

pub fn report_error(error: AstError) {
    log::error!("{} {}", "AST error:".bold().red(), error);
}

#[derive(Debug, PartialEq)]
pub enum AstError {
    ExpectedRightBracket,
    ExpectedRightParenthesis,
    ExpectedCommaOrRightParenthesis(Lexeme),
    NotExpectedEndOfExpression,
    NotExpectedLexeme(Lexeme),
    StringOutsideFunction(String),
    UnreachableLexeme(Lexeme),

    CannotBuildEmptyTree,
    FailedPopFromQueue,
    DivisionByZero(AstNode),
}

impl std::fmt::Display for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::ExpectedCommaOrRightParenthesis(lexeme) => &format!(
                "Expected ',' or ')', but found \"{}\".",
                lexeme.display_type()
            ),
            Self::ExpectedRightBracket => "Expected right bracket.",
            Self::ExpectedRightParenthesis => "Expected right parenthesis.",
            Self::NotExpectedEndOfExpression => "Not expected end of expression.",
            Self::NotExpectedLexeme(lexeme) => {
                &format!("Not expected lexeme \"{}\".", lexeme.display_type())
            },
            Self::StringOutsideFunction(string) => {
                &format!("String literal \"{}\" outside function call.", string)
            },
            Self::UnreachableLexeme(lexeme) => {
                &format!("Unreachable lexeme \"{}\".", lexeme.display_type())
            },

            Self::CannotBuildEmptyTree => {
                "Cannot build a balanced tree from zero operands"
            },
            Self::FailedPopFromQueue => {
                "Failed to pop node from the queue during tree construction"
            },
            Self::DivisionByZero(node) => &format!("Division by zero. Node: {:#?}", node),
        };

        write!(f, "{}", text)
    }
}

impl std::fmt::Display for UnaryOperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

impl std::fmt::Display for BinaryOperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Or => write!(f, "|"),
            Self::And => write!(f, "&"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{lexer, tokenizer};
    use crate::logger;
    use log::LevelFilter;

    use std::sync::Once;

    static INIT_LOGGER: Once = Once::new();

    pub fn initialize_logger() {
        INIT_LOGGER.call_once(|| {
            logger::LogSettings {
                level: LevelFilter::Info,
                output_file: None,
            }
            .setup()
            .expect("ERROR: Logging");
        });
    }

    fn process(code: &str) -> AbstractSyntaxTree {
        initialize_logger();
        let tokens = tokenizer::tokenize(code);
        let lexemes = lexer::Lexer::new(tokens).run();
        assert!(lexemes.is_ok());
        let lexemes = lexemes.unwrap();
        let result = AstParser::new(lexemes).parse();
        assert!(result.is_ok());
        match result {
            Ok(ast) => {
                report_success(&ast);
                ast
            },
            Err(error) => {
                report_error(error);
                panic!()
            },
        }
    }

    #[test]
    fn test_1() {
        let code = "a + b * c";
        let actual_ast = process(code);
        let expected_ast = AstNode::BinaryOperation {
            operation: BinaryOperationKind::Plus,
            left: Box::new(AstNode::Identifier("a".to_string())),
            right: Box::new(AstNode::BinaryOperation {
                operation: BinaryOperationKind::Multiply,
                left: Box::new(AstNode::Identifier("b".to_string())),
                right: Box::new(AstNode::Identifier("c".to_string())),
            }),
        };
        assert_eq!(AbstractSyntaxTree::from_node(expected_ast), actual_ast);
    }

    #[test]
    fn test_2() {
        let code = "a + b * func(a, (b - c) * !d)";
        let actual_ast = process(code);
        let expected_ast = AstNode::BinaryOperation {
            operation: BinaryOperationKind::Plus,
            left: Box::new(AstNode::Identifier("a".to_string())),
            right: Box::new(AstNode::BinaryOperation {
                operation: BinaryOperationKind::Multiply,
                left: Box::new(AstNode::Identifier("b".to_string())),
                right: Box::new(AstNode::FunctionCall {
                    name: "func".to_string(),
                    arguments: vec![
                        AstNode::Identifier("a".to_string()),
                        AstNode::BinaryOperation {
                            operation: BinaryOperationKind::Multiply,
                            left: Box::new(AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Minus,
                                left: Box::new(AstNode::Identifier("b".to_string())),
                                right: Box::new(AstNode::Identifier("c".to_string())),
                            }),
                            right: Box::new(AstNode::UnaryOperation {
                                operation: UnaryOperationKind::Not,
                                expression: Box::new(AstNode::Identifier(
                                    "d".to_string(),
                                )),
                            }),
                        },
                    ],
                }),
            }),
        };
        assert_eq!(AbstractSyntaxTree::from_node(expected_ast), actual_ast);
    }

    #[test]
    fn test_3() {
        let code = "a + b * c + \"hello\"";
        let tokens = tokenizer::tokenize(code);
        let lexemes = lexer::Lexer::new(tokens).run();
        assert!(lexemes.is_ok());
        let lexemes = lexemes.unwrap();
        let result = AstParser::new(lexemes).parse();
        let actual_error = Err(AstError::StringOutsideFunction("hello".to_string()));
        assert_eq!(actual_error, result);
    }

    #[test]
    fn test_4() {
        let code = "a + b * func(a, \"hello\", (b - c) * !d)";
        let actual_ast = process(code);
        let expected_ast = AstNode::BinaryOperation {
            operation: BinaryOperationKind::Plus,
            left: Box::new(AstNode::Identifier("a".to_string())),
            right: Box::new(AstNode::BinaryOperation {
                operation: BinaryOperationKind::Multiply,
                left: Box::new(AstNode::Identifier("b".to_string())),
                right: Box::new(AstNode::FunctionCall {
                    name: "func".to_string(),
                    arguments: vec![
                        AstNode::Identifier("a".to_string()),
                        AstNode::StringLiteral("hello".to_string()),
                        AstNode::BinaryOperation {
                            operation: BinaryOperationKind::Multiply,
                            left: Box::new(AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Minus,
                                left: Box::new(AstNode::Identifier("b".to_string())),
                                right: Box::new(AstNode::Identifier("c".to_string())),
                            }),
                            right: Box::new(AstNode::UnaryOperation {
                                operation: UnaryOperationKind::Not,
                                expression: Box::new(AstNode::Identifier(
                                    "d".to_string(),
                                )),
                            }),
                        },
                    ],
                }),
            }),
        };
        assert_eq!(AbstractSyntaxTree::from_node(expected_ast), actual_ast);
    }

    #[test]
    fn test_5() {
        let code = "a + b * func(a, (b - c) * !d, \"hello\")";
        let actual_ast = process(code);
        let expected_ast = AstNode::BinaryOperation {
            operation: BinaryOperationKind::Plus,
            left: Box::new(AstNode::Identifier("a".to_string())),
            right: Box::new(AstNode::BinaryOperation {
                operation: BinaryOperationKind::Multiply,
                left: Box::new(AstNode::Identifier("b".to_string())),
                right: Box::new(AstNode::FunctionCall {
                    name: "func".to_string(),
                    arguments: vec![
                        AstNode::Identifier("a".to_string()),
                        AstNode::BinaryOperation {
                            operation: BinaryOperationKind::Multiply,
                            left: Box::new(AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Minus,
                                left: Box::new(AstNode::Identifier("b".to_string())),
                                right: Box::new(AstNode::Identifier("c".to_string())),
                            }),
                            right: Box::new(AstNode::UnaryOperation {
                                operation: UnaryOperationKind::Not,
                                expression: Box::new(AstNode::Identifier(
                                    "d".to_string(),
                                )),
                            }),
                        },
                        AstNode::StringLiteral("hello".to_string()),
                    ],
                }),
            }),
        };
        assert_eq!(AbstractSyntaxTree::from_node(expected_ast), actual_ast);
    }

    #[test]
    fn test_6() {
        let code = "a + b * c + a[5] * sdsf[10 * 32 / 2]";
        let actual_ast = process(code);
        let expected_ast = AstNode::BinaryOperation {
            operation: BinaryOperationKind::Plus,
            left: Box::new(AstNode::BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(AstNode::Identifier("a".to_string())),
                right: Box::new(AstNode::BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left: Box::new(AstNode::Identifier("b".to_string())),
                    right: Box::new(AstNode::Identifier("c".to_string())),
                }),
            }),
            right: Box::new(AstNode::BinaryOperation {
                operation: BinaryOperationKind::Multiply,
                left: Box::new(AstNode::ArrayAccess {
                    identifier: "a".to_string(),
                    indices: vec![AstNode::Number(5.0)],
                }),
                right: Box::new(AstNode::ArrayAccess {
                    identifier: "sdsf".to_string(),
                    indices: vec![AstNode::BinaryOperation {
                        operation: BinaryOperationKind::Divide,
                        left: Box::new(AstNode::BinaryOperation {
                            operation: BinaryOperationKind::Multiply,
                            left: Box::new(AstNode::Number(10.0)),
                            right: Box::new(AstNode::Number(32.0)),
                        }),
                        right: Box::new(AstNode::Number(2.0)),
                    }],
                }),
            }),
        };
        assert_eq!(AbstractSyntaxTree::from_node(expected_ast), actual_ast);
    }

    #[test]
    fn test_7() {
        let code = "a + b * c + a[5] * sdsf[10 * 32 / 2][5 - 3 * c] * s";
        let actual_ast = process(code);
        let expected_ast = AstNode::BinaryOperation {
            operation: BinaryOperationKind::Plus,
            left: Box::new(AstNode::BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(AstNode::Identifier("a".to_string())),
                right: Box::new(AstNode::BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left: Box::new(AstNode::Identifier("b".to_string())),
                    right: Box::new(AstNode::Identifier("c".to_string())),
                }),
            }),
            right: Box::new(AstNode::BinaryOperation {
                operation: BinaryOperationKind::Multiply,
                left: Box::new(AstNode::BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left: Box::new(AstNode::ArrayAccess {
                        identifier: "a".to_string(),
                        indices: vec![AstNode::Number(5.0)],
                    }),
                    right: Box::new(AstNode::ArrayAccess {
                        identifier: "sdsf".to_string(),
                        indices: vec![
                            AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Divide,
                                left: Box::new(AstNode::BinaryOperation {
                                    operation: BinaryOperationKind::Multiply,
                                    left: Box::new(AstNode::Number(10.0)),
                                    right: Box::new(AstNode::Number(32.0)),
                                }),
                                right: Box::new(AstNode::Number(2.0)),
                            },
                            AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Minus,
                                left: Box::new(AstNode::Number(5.0)),
                                right: Box::new(AstNode::BinaryOperation {
                                    operation: BinaryOperationKind::Multiply,
                                    left: Box::new(AstNode::Number(3.0)),
                                    right: Box::new(AstNode::Identifier("c".to_string())),
                                }),
                            },
                        ],
                    }),
                }),
                right: Box::new(AstNode::Identifier("s".to_string())),
            }),
        };
        assert_eq!(AbstractSyntaxTree::from_node(expected_ast), actual_ast);
    }
}
