use crate::compiler::ast::tree::{
    AbstractSyntaxTree, AstError, AstNode, BinaryOperationKind, UnaryOperationKind,
};
use crate::compiler::reports::Reporter;
use crate::utils::StringBuffer;

impl AbstractSyntaxTree {
    pub fn transform(self) -> Result<AbstractSyntaxTree, AstError> {
        let peek = Self::transform_recursive(self.peek)?;

        Ok(Self::from_node(peek))
    }

    pub fn transform_recursive(node: AstNode) -> Result<AstNode, AstError> {
        match &node {
            AstNode::Number(_) | AstNode::Identifier(_) | AstNode::StringLiteral(_) => {
                Ok(node)
            },

            AstNode::UnaryOperation {
                operation,
                expression,
            } => {
                let transformed_expression =
                    Self::transform_recursive(*expression.clone())?;
                match operation {
                    UnaryOperationKind::Not => Ok(AstNode::UnaryOperation {
                        operation: operation.clone(),
                        expression: Box::new(transformed_expression),
                    }),

                    UnaryOperationKind::Minus => {
                        match &transformed_expression {
                            // Rule: -(-A) => A
                            AstNode::UnaryOperation {
                                operation: UnaryOperationKind::Minus,
                                expression: inner_expr,
                            } => Ok(*inner_expr.clone()),

                            // Rule: -(A + B) => (-A) + (-B)
                            AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Plus,
                                left,
                                right,
                            } => {
                                let new_left = AstNode::UnaryOperation {
                                    operation: UnaryOperationKind::Minus,
                                    expression: left.clone(),
                                };
                                let new_right = AstNode::UnaryOperation {
                                    operation: UnaryOperationKind::Minus,
                                    expression: right.clone(),
                                };
                                Self::transform_recursive(AstNode::BinaryOperation {
                                    operation: BinaryOperationKind::Plus,
                                    left: Box::new(new_left),
                                    right: Box::new(new_right),
                                })
                            },

                            // Rule: -(A - B) => (-A) + B
                            AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Minus,
                                left,
                                right,
                            } => {
                                let new_left = AstNode::UnaryOperation {
                                    operation: UnaryOperationKind::Minus,
                                    expression: left.clone(),
                                };
                                Self::transform_recursive(AstNode::BinaryOperation {
                                    operation: BinaryOperationKind::Plus,
                                    left: Box::new(new_left),
                                    right: right.clone(),
                                })
                            },

                            // Rule: -(Num * B) => -Num * B
                            AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Multiply,
                                left,
                                right,
                            } => {
                                if let AstNode::Number(number) = *left.clone() {
                                    return Ok(AstNode::BinaryOperation {
                                        operation: BinaryOperationKind::Multiply,
                                        left: Box::new(AstNode::Number(-number)),
                                        right: Box::new(*right.clone()),
                                    });
                                }

                                Ok(AstNode::UnaryOperation {
                                    operation: UnaryOperationKind::Minus,
                                    expression: Box::new(transformed_expression),
                                })
                            },

                            // Other cases (for example, -(A*B) or just -A)
                            // just leaving them (for example -(A*B) or just -A)
                            _ => Ok(AstNode::UnaryOperation {
                                operation: UnaryOperationKind::Minus,
                                expression: Box::new(transformed_expression),
                            }),
                        }
                    },
                }
            },

            AstNode::FunctionCall { name, arguments } => {
                let mut transformed_arguments = vec![];
                for argument in arguments {
                    transformed_arguments
                        .push(Self::transform_recursive(argument.clone())?);
                }

                Ok(AstNode::FunctionCall {
                    name: name.clone(),
                    arguments: transformed_arguments,
                })
            },

            AstNode::BinaryOperation {
                operation,
                left,
                right,
            } => {
                let transformed_left = Self::transform_recursive(*left.clone())?;
                let transformed_right = Self::transform_recursive(*right.clone())?;

                match operation {
                    // Rule 1: A - B  =>  A + (-B)
                    BinaryOperationKind::Minus => {
                        match &transformed_right {
                            AstNode::Number(number) if number.is_sign_negative() => {
                                // Rule 1: A - (-B)  =>  A + B
                                Ok(AstNode::BinaryOperation {
                                    operation: BinaryOperationKind::Plus,
                                    left: Box::new(transformed_left),
                                    right: Box::new(AstNode::Number(f64::abs(*number))),
                                })
                            },
                            AstNode::Number(number) => {
                                // Rule 1: A - B  =>  A + (-B)
                                Ok(AstNode::BinaryOperation {
                                    operation: BinaryOperationKind::Plus,
                                    left: Box::new(transformed_left),
                                    right: Box::new(AstNode::Number(-number)),
                                })
                            },
                            _ => {
                                let new_right = AstNode::UnaryOperation {
                                    operation: UnaryOperationKind::Minus,
                                    expression: Box::new(transformed_right),
                                };

                                let result_node = AstNode::BinaryOperation {
                                    operation: BinaryOperationKind::Plus,
                                    left: Box::new(transformed_left),
                                    right: Box::new(new_right),
                                };
                                Self::transform_recursive(result_node)
                            },
                        }
                    },

                    // Rule 2: A / B  =>  A * (1 / B)
                    BinaryOperationKind::Divide => {
                        if let AstNode::Number(number) = transformed_left
                            && number == 1.0
                        {
                            return Ok(AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Divide,
                                left: Box::new(transformed_left),
                                right: Box::new(transformed_right),
                            });
                        }

                        if let AstNode::Number(number) = transformed_right {
                            return if number == 0.0 {
                                Err(AstError::DivisionByZero(node))
                            } else {
                                Ok(AstNode::BinaryOperation {
                                    operation: BinaryOperationKind::Multiply,
                                    left: Box::new(transformed_left),
                                    right: Box::new(AstNode::Number(1.0 / number)),
                                })
                            };
                        }
                        Ok(AstNode::BinaryOperation {
                            operation: BinaryOperationKind::Multiply,
                            left: Box::new(transformed_left),
                            right: Box::new(AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Divide,
                                left: Box::new(AstNode::Number(1.0)), // "1"
                                right: Box::new(transformed_right),   // "B"
                            }),
                        })
                    },

                    // Other operations (Plus, Multiply, And, Or)
                    // left without editing, but with transformed kids.
                    _ => Ok(AstNode::BinaryOperation {
                        operation: operation.clone(),
                        left: Box::new(transformed_left),
                        right: Box::new(transformed_right),
                    }),
                }
            },

            AstNode::ArrayAccess {
                identifier,
                indices,
            } => {
                let mut transformed_indices = vec![];
                for index in indices {
                    transformed_indices.push(Self::transform_recursive(index.clone())?);
                }
                Ok(AstNode::ArrayAccess {
                    identifier: identifier.clone(),
                    indices: transformed_indices,
                })
            },
        }
    }
}

impl Reporter {
    pub fn transforming(&self, result: &Result<AbstractSyntaxTree, AstError>) -> String {
        let mut buffer = StringBuffer::default();

        match result {
            Ok(tree) => {
                buffer.add_line(
                    "Transformed Abstract-Syntax Tree generation success!\n".to_string(),
                );
                buffer.add_line(tree.pretty_print());
            },
            Err(error) => buffer.add_line(format!(
                "Transformed Abstract-Syntax Tree generation error: {}",
                error
            )),
        }

        buffer.get()
    }
}
