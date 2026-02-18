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
        match node {
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

                            // Others - it is what it is
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
                // First we recursively transform the subtrees so that they are also optimized
                // But for chains (Minus/Divide) we do this inside helpers,
                // so here we just pass the structure on.

                match operation {
                    // Rule: A - S - D - F => A - (S + D + F)
                    BinaryOperationKind::Minus => {
                        // First we recursively transform the left and right parts
                        let transformed_left = Self::transform_recursive(*left)?;
                        let transformed_right = Self::transform_recursive(*right)?;

                        // Creating node: A + (-B)
                        Ok(AstNode::BinaryOperation {
                            operation: BinaryOperationKind::Plus, // Changing operation to Plus
                            left: Box::new(transformed_left),
                            right: Box::new(AstNode::UnaryOperation {
                                operation: UnaryOperationKind::Minus, // Negating the right part
                                expression: Box::new(transformed_right),
                            }),
                        })
                    },

                    // Rule: A / S / D / F => A / (S * D * F)
                    BinaryOperationKind::Divide => {
                        let node = AstNode::BinaryOperation {
                            operation,
                            left,
                            right,
                        };
                        Self::optimize_division_chain(node)
                    },

                    // For Plus, Multiply and others, we simply process the children recursively
                    _ => {
                        let transformed_left = Self::transform_recursive(*left)?;
                        let transformed_right = Self::transform_recursive(*right)?;
                        Ok(AstNode::BinaryOperation {
                            operation: operation.clone(),
                            left: Box::new(transformed_left),
                            right: Box::new(transformed_right),
                        })
                    },
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

    /// Optimizes the division chain: A / B / C -> A / (B * C)
    fn optimize_division_chain(node: AstNode) -> Result<AstNode, AstError> {
        // 1. We collect the chain: head=A, terms=[B, C]
        let (head, terms) =
            Self::collect_left_associative_chain(node, BinaryOperationKind::Divide);

        let transformed_head = Self::transform_recursive(head)?;

        if terms.is_empty() {
            return Ok(transformed_head);
        }

        let mut transformed_terms = Vec::new();
        for term in terms {
            transformed_terms.push(Self::transform_recursive(term)?);
        }

        // 2. We construct the product of the divisors: (B * C)
        let product_node = Self::build_left_associative_tree(
            transformed_terms,
            BinaryOperationKind::Multiply,
        );

        // 3. Return: A / (Product)
        Ok(AstNode::BinaryOperation {
            operation: BinaryOperationKind::Divide,
            left: Box::new(transformed_head),
            right: Box::new(product_node),
        })
    }

    /// Helper function: expands left associative operations into a flat list.
    /// (A op B) op C -> return (A, [B, C])
    fn collect_left_associative_chain(
        mut node: AstNode, target_op: BinaryOperationKind,
    ) -> (AstNode, Vec<AstNode>) {
        let mut terms = Vec::new();

        // We move down the left side until the operation matches
        loop {
            match node {
                AstNode::BinaryOperation {
                    operation,
                    left,
                    right,
                } if operation == target_op => {
                    terms.push(*right); // We store the right operand (C, then B...)
                    node = *left; // Let's go left.
                },
                _ => {
                    // Reached the "head" (A), which is not this operation
                    terms.reverse(); // We collected from the end ([C, B]), so we unwrap ([B, C])
                    return (node, terms);
                },
            }
        }
    }

    /// Helper function: reassembles the tree using a new operation.
    /// terms=[B, C, D], op=Plus -> ((B + C) + D)
    fn build_left_associative_tree(
        terms: Vec<AstNode>, op: BinaryOperationKind,
    ) -> AstNode {
        let mut iter = terms.into_iter();

        let mut current = match iter.next() {
            Some(term) => term,
            None => unreachable!(
                "Guaranteed to have at least one element, as we check !is_empty() above"
            ),
        };

        for term in iter {
            current = AstNode::BinaryOperation {
                operation: op.clone(),
                left: Box::new(current),
                right: Box::new(term),
            };
        }

        current
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
