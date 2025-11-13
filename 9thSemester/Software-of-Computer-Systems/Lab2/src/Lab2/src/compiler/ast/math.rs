use crate::compiler::ast::tree::{
    AbstractSyntaxTree, AstError, AstNode, BinaryOperationKind, UnaryOperationKind,
};
use colored::Colorize;

impl AbstractSyntaxTree {
    pub fn compute(self) -> Result<AbstractSyntaxTree, AstError> {
        let computed = Self::compute_recursive(self.peek)?;

        Ok(Self::from_node(computed))
    }

    fn compute_recursive(node: AstNode) -> Result<AstNode, AstError> {
        match &node {
            AstNode::Number(_) | AstNode::Identifier(_) | AstNode::StringLiteral(_) => {
                Ok(node)
            },
            AstNode::UnaryOperation {
                operation: op,
                expression,
            } => match &op {
                UnaryOperationKind::Minus => {
                    let child = Self::compute_recursive(*expression.clone())?;
                    if let AstNode::Number(number) = child {
                        return Ok(AstNode::Number(-number));
                    };

                    if let AstNode::BinaryOperation {
                        operation,
                        left,
                        right,
                    } = child
                        && operation == BinaryOperationKind::Minus
                    {
                        return Ok(AstNode::BinaryOperation {
                            operation: BinaryOperationKind::Plus,
                            left: Box::new(AstNode::UnaryOperation {
                                operation: UnaryOperationKind::Minus,
                                expression: left,
                            }),
                            right,
                        });
                    }

                    Ok(node)
                },
                UnaryOperationKind::Not => Ok(node),
            },
            AstNode::BinaryOperation {
                operation,
                left,
                right,
            } => match operation {
                BinaryOperationKind::Plus
                | BinaryOperationKind::Minus
                | BinaryOperationKind::Multiply
                | BinaryOperationKind::Divide => {
                    let computed_left = Self::compute_recursive(*left.clone())?;
                    let computed_right = Self::compute_recursive(*right.clone())?;

                    // Case: (a + b) - (a + b) = 0
                    // Or: (a + b) / (a + b) = 1
                    if computed_left.eq(&computed_right) {
                        match operation {
                            BinaryOperationKind::Minus => {
                                return Ok(AstNode::Number(0.0));
                            },
                            BinaryOperationKind::Divide => {
                                if let AstNode::Number(number) = &computed_left
                                    && *number == 0.0
                                {
                                    // Case: (5 - 5) / (5 - 5)
                                    return Err(AstError::DivisionByZero(node));
                                }
                                return Ok(AstNode::Number(1.0));
                            },
                            _ => {},
                        }
                    }

                    if let (AstNode::Number(left_number), AstNode::Number(right_number)) =
                        (&computed_left, &computed_right)
                    {
                        let result = match operation {
                            BinaryOperationKind::Plus => left_number + right_number,
                            BinaryOperationKind::Minus => left_number - right_number,
                            BinaryOperationKind::Multiply => left_number * right_number,
                            BinaryOperationKind::Divide => {
                                if *right_number == 0.0 {
                                    return Err(AstError::DivisionByZero(node));
                                } else {
                                    left_number / right_number
                                }
                            },
                            _ => unreachable!(),
                        };
                        Ok(AstNode::Number(result))
                    } else if let AstNode::Number(number) = &computed_left {
                        if number == &0.0 {
                            if [
                                BinaryOperationKind::Multiply,
                                BinaryOperationKind::Divide,
                            ]
                            .contains(operation)
                            {
                                return Ok(AstNode::Number(0.0));
                            }
                            if BinaryOperationKind::Plus == *operation {
                                return Ok(computed_right);
                            }
                            if BinaryOperationKind::Minus == *operation {
                                return Ok(AstNode::UnaryOperation {
                                    operation: UnaryOperationKind::Minus,
                                    expression: Box::new(computed_right),
                                });
                            }
                        }
                        if number == &1.0 && BinaryOperationKind::Multiply == *operation {
                            return Ok(computed_right);
                        }

                        Ok(AstNode::BinaryOperation {
                            operation: operation.clone(),
                            left: Box::new(computed_left),
                            right: Box::new(computed_right),
                        })
                    } else if let AstNode::Number(number) = &computed_right {
                        if number == &0.0 {
                            if BinaryOperationKind::Divide == *operation {
                                return Err(AstError::DivisionByZero(node));
                            }
                            if BinaryOperationKind::Multiply == *operation {
                                return Ok(AstNode::Number(0.0));
                            }
                            if [BinaryOperationKind::Plus, BinaryOperationKind::Minus]
                                .contains(operation)
                            {
                                return Ok(computed_left);
                            }
                        }
                        if number == &1.0 && BinaryOperationKind::Multiply == *operation {
                            return Ok(computed_left);
                        }

                        if BinaryOperationKind::Minus == *operation
                            && let AstNode::UnaryOperation {
                                operation: UnaryOperationKind::Minus,
                                expression: inner_expr,
                            } = &computed_right
                        {
                            return Ok(AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Plus,
                                left: Box::new(computed_left),
                                right: inner_expr.clone(),
                            });
                        }

                        Ok(AstNode::BinaryOperation {
                            operation: operation.clone(),
                            left: Box::new(computed_left),
                            right: Box::new(computed_right),
                        })
                    } else {
                        Ok(AstNode::BinaryOperation {
                            operation: operation.clone(),
                            left: Box::new(computed_left),
                            right: Box::new(computed_right),
                        })
                    }
                },
                _ => Ok(node),
            },
            AstNode::FunctionCall { name, arguments } => {
                let mut computed_arguments = Vec::new();
                for arg in arguments {
                    let arg = Self::compute_recursive(arg.clone())?;
                    computed_arguments.push(arg);
                }

                Ok(AstNode::FunctionCall {
                    name: name.clone(),
                    arguments: computed_arguments,
                })
            },
            AstNode::ArrayAccess {
                identifier,
                indices,
            } => {
                let mut computed_indices = Vec::new();
                for index in indices {
                    let index = Self::compute_recursive(index.clone())?;
                    computed_indices.push(index);
                }
                Ok(AstNode::ArrayAccess {
                    identifier: identifier.clone(),
                    indices: computed_indices,
                })
            },
        }
    }
}

pub fn report_success(tree: &AbstractSyntaxTree, run: u8) {
    log::warn!(
        "Computing constants of Abstract-Syntax Tree (Run #{}) {}.",
        run.to_string().bright_magenta().italic(),
        "success".bold().green()
    );
    log::info!("{}", tree.pretty_print());
}

pub fn report_error(error: AstError, run: u8) {
    log::error!(
        "{} (Run #{}) {}",
        "Computing constants of Abstract-Syntax Tree:".bold().red(),
        run.to_string().bright_magenta().italic(),
        error
    );
}

pub fn check_finalization(tree: &AbstractSyntaxTree) -> bool {
    if let AstNode::Number(number) = &tree.peek {
        log::warn!(
            "{} = {}. {}.",
            "Computing solved code, result".bold().green(),
            number,
            "Further optimization is not needed".bold().red(),
        );
        return true;
    }
    false
}
