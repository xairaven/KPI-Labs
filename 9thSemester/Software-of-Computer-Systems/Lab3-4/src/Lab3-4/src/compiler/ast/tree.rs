use crate::compiler::lexer::Lexeme;
use crate::compiler::reports::Reporter;
use crate::utils::StringBuffer;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractSyntaxTree {
    pub peek: AstNode,
}

impl AbstractSyntaxTree {
    pub fn from_node(node: AstNode) -> Self {
        Self { peek: node }
    }

    pub fn pretty_print(&self) -> String {
        let mut buffer = StringBuffer::default();
        Self::print_recursive(&self.peek, &mut buffer, "".to_string(), true);
        buffer.get()
    }

    fn print_recursive(
        node: &AstNode, buffer: &mut StringBuffer, prefix: String, is_last: bool,
    ) {
        let connector = if is_last { "└── " } else { "├── " };

        buffer.add(format!("{}{}", prefix, connector));

        let node_text = match node {
            AstNode::Number(n) => format!("{n:.3}"),
            AstNode::Identifier(s) => s.to_string(),
            AstNode::StringLiteral(s) => format!("\"{}\"", s),
            AstNode::UnaryOperation { operation, .. } => operation.to_string(),
            AstNode::BinaryOperation { operation, .. } => operation.to_string(),
            AstNode::FunctionCall { name, .. } => format!("{}(...)", name),
            AstNode::ArrayAccess { identifier, .. } => {
                format!("{}[...]", identifier)
            },
        };
        buffer.add_line(node_text);

        let new_prefix = prefix + if is_last { "    " } else { "│   " };

        match node {
            AstNode::Number(_) | AstNode::Identifier(_) | AstNode::StringLiteral(_) => {},

            AstNode::UnaryOperation { expression, .. } => {
                Self::print_recursive(expression, buffer, new_prefix, true);
            },

            AstNode::BinaryOperation { left, right, .. } => {
                Self::print_recursive(left, buffer, new_prefix.clone(), false);
                Self::print_recursive(right, buffer, new_prefix, true);
            },

            AstNode::FunctionCall { arguments, .. } => {
                let arg_count = arguments.len();
                for (i, arg) in arguments.iter().enumerate() {
                    let is_last_arg = i == arg_count - 1;
                    Self::print_recursive(arg, buffer, new_prefix.clone(), is_last_arg);
                }
            },

            AstNode::ArrayAccess {
                identifier: _,
                indices,
            } => {
                let dimensions = indices.len();
                for (i, index) in indices.iter().enumerate() {
                    let is_last_arg = i == dimensions - 1;
                    Self::print_recursive(index, buffer, new_prefix.clone(), is_last_arg);
                }
            },
        }
    }

    pub fn to_canonical_string(&self) -> String {
        Self::node_to_canonical_string(&self.peek)
    }

    fn node_to_canonical_string(node: &AstNode) -> String {
        match node {
            AstNode::Number(n) => format!("{:.2}", n),
            AstNode::Identifier(s) => s.clone(),
            AstNode::StringLiteral(s) => format!("\"{}\"", s),
            AstNode::UnaryOperation {
                operation,
                expression,
            } => {
                format!(
                    "({}{})",
                    operation,
                    Self::node_to_canonical_string(expression)
                )
            },
            AstNode::FunctionCall { name, arguments } => {
                let args = arguments
                    .iter()
                    .map(Self::node_to_canonical_string)
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}({})", name, args)
            },
            AstNode::ArrayAccess {
                identifier,
                indices,
            } => {
                let idx = indices
                    .iter()
                    .map(Self::node_to_canonical_string)
                    .map(|s| format!("[{}]", s))
                    .collect::<String>();
                format!("{}{}", identifier, idx)
            },
            AstNode::BinaryOperation {
                operation,
                left,
                right,
            } => {
                let l_str = Self::node_to_canonical_string(left);
                let r_str = Self::node_to_canonical_string(right);

                // Sorting for commutative operations
                match operation {
                    BinaryOperationKind::Plus | BinaryOperationKind::Multiply => {
                        let mut parts = [l_str, r_str];
                        parts.sort();
                        format!("({} {} {})", parts[0], operation, parts[1])
                    },
                    _ => {
                        format!("({} {} {})", l_str, operation, r_str)
                    },
                }
            },
        }
    }

    /// Creates a readable string representation, adding parentheses only
    /// when required by operator precedence.
    pub fn to_pretty_string(&self) -> String {
        // Start recursion with the lowest parent precedence (0).
        Self::node_to_pretty_string(&self.peek, 0)
    }

    /// Recursive helper for `to_pretty_string`.
    fn node_to_pretty_string(node: &AstNode, parent_precedence: u8) -> String {
        match node {
            // Atomic nodes just return their string.
            AstNode::Number(n) => format!("{n:.2}"),
            AstNode::Identifier(s) => s.clone(),
            AstNode::StringLiteral(s) => format!("\"{}\"", s),

            AstNode::FunctionCall { name, arguments } => {
                let args = arguments
                    .iter()
                    .map(|arg| Self::node_to_pretty_string(arg, 0))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}({})", name, args)
            },

            AstNode::ArrayAccess {
                identifier,
                indices,
            } => {
                let idx = indices
                    .iter()
                    .map(|idx| Self::node_to_pretty_string(idx, 0))
                    .map(|s| format!("[{}]", s))
                    .collect::<String>();
                format!("{}{}", identifier, idx)
            },

            AstNode::UnaryOperation {
                operation,
                expression,
            } => {
                let my_precedence = 3;
                let expr_str = Self::node_to_pretty_string(expression, my_precedence);
                let result = format!("{}{}", operation, expr_str);

                if my_precedence < parent_precedence {
                    format!("({})", result)
                } else {
                    result
                }
            },

            AstNode::BinaryOperation {
                operation,
                left,
                right,
            } => {
                let my_precedence = operation.precedence();

                if *operation == BinaryOperationKind::Plus {
                    // Case 1: A + (-B)  =>  "A - B"
                    if let AstNode::UnaryOperation {
                        operation: UnaryOperationKind::Minus,
                        expression: inner_right,
                    } = right.as_ref()
                    {
                        let l_str = Self::node_to_pretty_string(left, my_precedence);
                        let r_str =
                            Self::node_to_pretty_string(inner_right, my_precedence + 1);
                        let result = format!("{} - {}", l_str, r_str);
                        if my_precedence < parent_precedence {
                            return format!("({})", result);
                        } else {
                            return result;
                        }
                    }

                    // Case 2 (NEW): (-A) + B  =>  "B - A"
                    if let AstNode::UnaryOperation {
                        operation: UnaryOperationKind::Minus,
                        expression: inner_left,
                    } = left.as_ref()
                    {
                        // We format this as "B - A"
                        let l_str_inner =
                            Self::node_to_pretty_string(inner_left, my_precedence + 1);
                        let r_str = Self::node_to_pretty_string(right, my_precedence);
                        // Note the swap: r_str - l_str_inner
                        let result = format!("{} - {}", r_str, l_str_inner);
                        if my_precedence < parent_precedence {
                            return format!("({})", result);
                        } else {
                            return result;
                        }
                    }
                }

                let (left_prec, right_prec) = match operation {
                    // For `A - B` or `A / B`, the right side (B)
                    // needs parentheses if it has the same precedence.
                    // e.g., A - (B - C) must keep its parentheses.
                    BinaryOperationKind::Minus | BinaryOperationKind::Divide => {
                        (my_precedence, my_precedence + 1)
                    },
                    // For associative ops `+` and `*`, just pass our own precedence.
                    _ => (my_precedence, my_precedence),
                };

                let l_str = Self::node_to_pretty_string(left, left_prec);
                let r_str = Self::node_to_pretty_string(right, right_prec);

                let result = format!("{} {} {}", l_str, operation, r_str);

                if my_precedence < parent_precedence {
                    format!("({})", result)
                } else {
                    result
                }
            },
        }
    }
}

impl BinaryOperationKind {
    /// Returns the precedence level for this operator.
    fn precedence(&self) -> u8 {
        match self {
            Self::Plus | Self::Minus | Self::Or => 1,
            Self::Multiply | Self::Divide | Self::And => 2,
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

impl Reporter {
    pub fn tree_build(&self, result: &Result<AbstractSyntaxTree, AstError>) -> String {
        let mut buffer = StringBuffer::default();

        match result {
            Ok(tree) => {
                buffer.add_line("Abstract-Syntax Tree generation success!\n".to_string());
                buffer.add_line(tree.pretty_print());
            },
            Err(error) => buffer.add_line(format!("AST error: {}", error)),
        }

        buffer.get()
    }
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
    use crate::compiler::lexer;
    use crate::compiler::tokenizer::Tokenizer;

    fn process(code: &str) -> AbstractSyntaxTree {
        let tokens = Tokenizer::process(code);
        let lexemes = lexer::Lexer::new(tokens).run();
        assert!(lexemes.is_ok());
        let lexemes = lexemes.unwrap();
        let result = AstParser::new(lexemes).parse();
        assert!(result.is_ok());
        result.unwrap_or_else(|_| panic!())
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
        let tokens = Tokenizer::process(code);
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
