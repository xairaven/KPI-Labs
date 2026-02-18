use crate::compiler::ast::tree::{
    AbstractSyntaxTree, AstError, AstNode, BinaryOperationKind,
};
use crate::compiler::reports::Reporter;
use crate::utils::StringBuffer;
use std::collections::VecDeque;

impl AbstractSyntaxTree {
    pub fn balance(self) -> Result<Self, AstError> {
        let peek = Self::balance_tree(self.peek)?;

        Ok(Self::from_node(peek))
    }

    pub fn balance_tree(node: AstNode) -> Result<AstNode, AstError> {
        match node {
            // Base cases, already balanced.
            AstNode::Number(_) | AstNode::Identifier(_) | AstNode::StringLiteral(_) => {
                Ok(node)
            },

            // Recursive cases for other node types.
            AstNode::UnaryOperation {
                operation,
                expression,
            } => Ok(AstNode::UnaryOperation {
                operation,
                expression: Box::new(Self::balance_tree(*expression)?),
            }),

            AstNode::FunctionCall { name, arguments } => {
                let mut balanced_arguments: Vec<AstNode> = vec![];
                for arg in arguments {
                    balanced_arguments.push(Self::balance_tree(arg)?);
                }

                Ok(AstNode::FunctionCall {
                    name,
                    arguments: balanced_arguments,
                })
            },

            AstNode::ArrayAccess {
                identifier,
                indices,
            } => {
                let mut balanced_indices: Vec<AstNode> = vec![];
                for index in indices {
                    balanced_indices.push(Self::balance_tree(index)?);
                }

                Ok(AstNode::ArrayAccess {
                    identifier,
                    indices: balanced_indices,
                })
            },

            // Main logic: Binary operations
            AstNode::BinaryOperation {
                operation,
                left,
                right,
            } => {
                match operation {
                    BinaryOperationKind::Plus | BinaryOperationKind::Multiply => {
                        let mut operands = Vec::new();
                        Self::collect_operands(
                            AstNode::BinaryOperation {
                                operation: operation.clone(),
                                left,
                                right,
                            },
                            operation.clone(),
                            &mut operands,
                        );

                        let mut balanced_operands = Vec::new();
                        for operand in operands {
                            balanced_operands.push(Self::balance_tree(operand)?);
                        }

                        Self::build_balanced_tree(balanced_operands, operation)
                    },

                    // Other operations (And, Or, etc.) are not associative
                    // in the arithmetic context. Just return them
                    // with already balanced children.
                    _ => {
                        let balanced_left = Self::balance_tree(*left)?;
                        let balanced_right = Self::balance_tree(*right)?;
                        Ok(AstNode::BinaryOperation {
                            operation,
                            left: Box::new(balanced_left),
                            right: Box::new(balanced_right),
                        })
                    },
                }
            },
        }
    }

    /// Making flatten tree.
    /// Recursively "unfolds" a chain of associative operations
    /// into a flat list. For example, the tree `(a + (b + c)) + d`
    /// with `op_kind = Plus` will be "flattened" into the list `[a, b, c, d]`.
    pub fn collect_operands(
        node: AstNode, op_kind: BinaryOperationKind, operands: &mut Vec<AstNode>,
    ) {
        match node {
            // If operation node is the same as we are looking for...
            AstNode::BinaryOperation {
                operation,
                left,
                right,
            } if operation == op_kind => {
                // ... we recursively collect operands from both sides.
                Self::collect_operands(*left, op_kind.clone(), operands);
                Self::collect_operands(*right, op_kind.clone(), operands);
            },
            // If operation node is different, or it's a leaf node...
            // or just operand (a, b, c...),
            // then it is a "leaf" for *this* chain.
            // We add it to the list.
            _ => {
                operands.push(node);
            },
        }
    }

    /// Building balanced tree
    /// Taking a flat list of operands and constructing
    /// a binary tree of minimal height using a queue-based algorithm.
    /// For example, `[a, b, c, d, e]` becomes `((a + b) + (c + d)) + e`
    /// (or a similar balanced structure).
    pub fn build_balanced_tree(
        operands: Vec<AstNode>, op_kind: BinaryOperationKind,
    ) -> Result<AstNode, AstError> {
        if operands.is_empty() {
            return Err(AstError::CannotBuildEmptyTree);
        }

        // Making a queue from the list of operands
        let mut queue: VecDeque<AstNode> = operands.into();

        // While more than one node remains in the queue...
        while queue.len() > 1 {
            let level_size = queue.len();

            // Process the current level of the tree:
            for _ in 0..(level_size / 2) {
                // Take two nodes from the front of the queue...
                let left = queue.pop_front().ok_or(AstError::FailedPopFromQueue)?;
                let right = queue.pop_front().ok_or(AstError::FailedPopFromQueue)?;

                // ...create a new binary operation node combining them...
                let new_node = AstNode::BinaryOperation {
                    operation: op_kind.clone(),
                    left: Box::new(left),
                    right: Box::new(right),
                };

                // .. and put the new node at the back of the queue
                // (it will be an operand for the next, higher level)
                queue.push_back(new_node);
            }

            // if level_size is odd...
            if !level_size.is_multiple_of(2) {
                // ...one node remains at the front of the queue.
                // We simply move it to the back,
                // so it can participate in the next iteration (next level).
                let odd_one_out =
                    queue.pop_front().ok_or(AstError::FailedPopFromQueue)?;
                queue.push_back(odd_one_out);
            }
        }

        // When only one node remains in the queue,
        // it is the root of the balanced tree.
        queue.pop_front().ok_or(AstError::FailedPopFromQueue)
    }
}

impl Reporter {
    pub fn balancing(&self, result: &Result<AbstractSyntaxTree, AstError>) -> String {
        let mut buffer = StringBuffer::default();

        match result {
            Ok(tree) => {
                buffer.add_line(
                    "Balanced Abstract-Syntax Tree generation succeed!\n".to_string(),
                );
                buffer.add_line(tree.pretty_print());
            },
            Err(error) => buffer.add_line(format!("Balancing AST error: {}", error)),
        }

        buffer.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::ast::tree::AstNode::{BinaryOperation, Identifier, Number};
    use crate::compiler::ast::tree::AstParser;
    use crate::compiler::lexer::Lexer;
    use crate::compiler::syntax::SyntaxAnalyzer;
    use crate::compiler::tokenizer::Tokenizer;

    fn process(code: &str) -> Option<AbstractSyntaxTree> {
        let tokens = Tokenizer::process(code);
        // Syntax Analysis
        let syntax_errors = SyntaxAnalyzer::new(&tokens).analyze();
        let is_syntax_analysis_successful = syntax_errors.is_empty();
        if !is_syntax_analysis_successful {
            return None;
        }
        // Making lexemes
        let lexemes_result = Lexer::new(tokens).run();
        let lexemes = match lexemes_result {
            Ok(lexemes) => lexemes,
            Err(error) => {
                return None;
            },
        };

        // AST Generation
        let ast_result = AstParser::new(lexemes).parse();
        let ast = match ast_result {
            Ok(ast) => ast,
            Err(error) => {
                return None;
            },
        };
        // AST Computing, Run #1
        let ast = compute_run(ast, 1)?;
        // AST Parallelization
        let ast_result = ast.transform();
        let ast = match ast_result {
            Ok(ast) => ast,
            Err(error) => {
                return None;
            },
        };
        // AST Computing, Run #2
        let ast = compute_run(ast, 2)?;
        // AST Balancing
        let ast_result = ast.balance();
        let ast = match ast_result {
            Ok(ast) => ast,
            Err(error) => {
                return None;
            },
        };
        Some(ast)
    }

    fn compute_run(tree: AbstractSyntaxTree, number: u8) -> Option<AbstractSyntaxTree> {
        // AST Math Optimization
        tree.compute().ok()
    }

    #[test]
    fn test_00() {
        let code = "a+b*c + k - x - d - e - f/g/h/q";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(BinaryOperation {
            operation: BinaryOperationKind::Minus,
            left: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(Identifier("a".to_string())),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Multiply,
                        left: Box::new(Identifier("b".to_string())),
                        right: Box::new(Identifier("c".to_string())),
                    }),
                }),
                right: Box::new(Identifier("k".to_string())),
            }),
            right: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(Identifier("x".to_string())),
                    right: Box::new(Identifier("d".to_string())),
                }),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(Identifier("e".to_string())),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Divide,
                        left: Box::new(Identifier("f".to_string())),
                        right: Box::new(BinaryOperation {
                            operation: BinaryOperationKind::Multiply,
                            left: Box::new(BinaryOperation {
                                operation: BinaryOperationKind::Multiply,
                                left: Box::new(Identifier("g".to_string())),
                                right: Box::new(Identifier("h".to_string())),
                            }),
                            right: Box::new(Identifier("q".to_string())),
                        }),
                    }),
                }),
            }),
        });

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_01() {
        let code = "a+b+c+d+e+f+g+h";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(AstNode::BinaryOperation {
            operation: BinaryOperationKind::Plus,
            left: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(Identifier("a".to_string())),
                    right: Box::new(Identifier("b".to_string())),
                }),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(Identifier("c".to_string())),
                    right: Box::new(Identifier("d".to_string())),
                }),
            }),
            right: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(Identifier("e".to_string())),
                    right: Box::new(Identifier("f".to_string())),
                }),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(Identifier("g".to_string())),
                    right: Box::new(Identifier("h".to_string())),
                }),
            }),
        });

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_01_2() {
        let code = "a+b+c+d+e+f+g+h+i";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(BinaryOperation {
            operation: BinaryOperationKind::Plus,
            left: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(Identifier("a".to_string())),
                        right: Box::new(Identifier("b".to_string())),
                    }),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(Identifier("c".to_string())),
                        right: Box::new(Identifier("d".to_string())),
                    }),
                }),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(Identifier("e".to_string())),
                        right: Box::new(Identifier("f".to_string())),
                    }),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(Identifier("g".to_string())),
                        right: Box::new(Identifier("h".to_string())),
                    }),
                }),
            }),
            right: Box::new(Identifier("i".to_string())),
        });

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_02() {
        let code = "a-b-c-d-e-f-g-h-i";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(BinaryOperation {
            operation: BinaryOperationKind::Minus,
            left: Box::new(Identifier("a".to_string())),
            right: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(Identifier("b".to_string())),
                        right: Box::new(Identifier("c".to_string())),
                    }),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(Identifier("d".to_string())),
                        right: Box::new(Identifier("e".to_string())),
                    }),
                }),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(Identifier("f".to_string())),
                        right: Box::new(Identifier("g".to_string())),
                    }),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(Identifier("h".to_string())),
                        right: Box::new(Identifier("i".to_string())),
                    }),
                }),
            }),
        });

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_03() {
        let code = "a/b/c/d/e/f/g/h/i";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(BinaryOperation {
            operation: BinaryOperationKind::Divide,
            left: Box::new(Identifier("a".to_string())),
            right: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Multiply,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Multiply,
                        left: Box::new(Identifier("b".to_string())),
                        right: Box::new(Identifier("c".to_string())),
                    }),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Multiply,
                        left: Box::new(Identifier("d".to_string())),
                        right: Box::new(Identifier("e".to_string())),
                    }),
                }),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Multiply,
                        left: Box::new(Identifier("f".to_string())),
                        right: Box::new(Identifier("g".to_string())),
                    }),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Multiply,
                        left: Box::new(Identifier("h".to_string())),
                        right: Box::new(Identifier("i".to_string())),
                    }),
                }),
            }),
        });

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_04() {
        let code = "a*(b-4) - 2*b*c - c*d - a*c*d/e/f/g - g-h-i-j";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(BinaryOperation {
            operation: BinaryOperationKind::Minus,
            left: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Multiply,
                left: Box::new(Identifier("a".to_string())),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Minus,
                    left: Box::new(Identifier("b".to_string())),
                    right: Box::new(Number(4.0)),
                }),
            }),
            right: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(BinaryOperation {
                            operation: BinaryOperationKind::Multiply,
                            left: Box::new(BinaryOperation {
                                operation: BinaryOperationKind::Multiply,
                                left: Box::new(Number(2.0)),
                                right: Box::new(Identifier("b".to_string())),
                            }),
                            right: Box::new(Identifier("c".to_string())),
                        }),
                        right: Box::new(BinaryOperation {
                            operation: BinaryOperationKind::Multiply,
                            left: Box::new(Identifier("c".to_string())),
                            right: Box::new(Identifier("d".to_string())),
                        }),
                    }),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(BinaryOperation {
                            operation: BinaryOperationKind::Divide,
                            left: Box::new(BinaryOperation {
                                operation: BinaryOperationKind::Multiply,
                                left: Box::new(BinaryOperation {
                                    operation: BinaryOperationKind::Multiply,
                                    left: Box::new(Identifier("a".to_string())),
                                    right: Box::new(Identifier("c".to_string())),
                                }),
                                right: Box::new(Identifier("d".to_string())),
                            }),
                            right: Box::new(BinaryOperation {
                                operation: BinaryOperationKind::Multiply,
                                left: Box::new(BinaryOperation {
                                    operation: BinaryOperationKind::Multiply,
                                    left: Box::new(Identifier("e".to_string())),
                                    right: Box::new(Identifier("f".to_string())),
                                }),
                                right: Box::new(Identifier("g".to_string())),
                            }),
                        }),
                        right: Box::new(Identifier("g".to_string())),
                    }),
                }),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(Identifier("h".to_string())),
                        right: Box::new(Identifier("i".to_string())),
                    }),
                    right: Box::new(Identifier("j".to_string())),
                }),
            }),
        });

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_05() {
        let code = "a+(b+c+d+(e+f)+g)+h";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(AstNode::BinaryOperation {
            operation: BinaryOperationKind::Plus,
            left: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(Identifier("a".to_string())),
                    right: Box::new(Identifier("b".to_string())),
                }),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(Identifier("c".to_string())),
                    right: Box::new(Identifier("d".to_string())),
                }),
            }),
            right: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(Identifier("e".to_string())),
                    right: Box::new(Identifier("f".to_string())),
                }),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Plus,
                    left: Box::new(Identifier("g".to_string())),
                    right: Box::new(Identifier("h".to_string())),
                }),
            }),
        });

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_06() {
        let code = "a-((b-c-d)-(e-f)-g)-h";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(BinaryOperation {
            operation: BinaryOperationKind::Minus,
            left: Box::new(Identifier("a".to_string())),
            right: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Minus,
                    left: Box::new(Identifier("b".to_string())),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(BinaryOperation {
                            operation: BinaryOperationKind::Plus,
                            left: Box::new(Identifier("c".to_string())),
                            right: Box::new(Identifier("d".to_string())),
                        }),
                        right: Box::new(BinaryOperation {
                            operation: BinaryOperationKind::Plus,
                            left: Box::new(BinaryOperation {
                                operation: BinaryOperationKind::Minus,
                                left: Box::new(Identifier("e".to_string())),
                                right: Box::new(Identifier("f".to_string())),
                            }),
                            right: Box::new(Identifier("g".to_string())),
                        }),
                    }),
                }),
                right: Box::new(Identifier("h".to_string())),
            }),
        });

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_07() {
        let code = "5040/8/7/6/5/4/3/2";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(Number(0.125));

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_08() {
        let code = "10-9-8-7-6-5-4-3-2-1";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(Number(-35.0));

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_09() {
        let code = "64-(32-16)-8-(4-2-1)";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(Number(39.0));

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_10() {
        let code = "-(-i)/1.0 + 0 - 0*k*h + 2 - 4.8/2 + 1*e/2";
        let balanced_ast = process(code);
        assert!(balanced_ast.is_some());

        let actual_ast = balanced_ast.unwrap();
        let expected_ast = AbstractSyntaxTree::from_node(BinaryOperation {
            operation: BinaryOperationKind::Plus,
            left: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left: Box::new(Identifier("i".to_string())),
                right: Box::new(Number(-0.3999999999999999)),
            }),
            right: Box::new(BinaryOperation {
                operation: BinaryOperationKind::Divide,
                left: Box::new(Identifier("e".to_string())),
                right: Box::new(Number(2.0)),
            }),
        });

        assert_eq!(actual_ast, expected_ast);
    }

    #[test]
    fn test_11() {
        let code = "a*2/0 + b/(b+b*0-1*b) - 1/(c*2*4.76*(1-2+1))";

        let tokens = Tokenizer::process(code);
        // Syntax Analysis
        let syntax_errors = SyntaxAnalyzer::new(&tokens).analyze();
        let is_syntax_analysis_successful = syntax_errors.is_empty();
        assert!(is_syntax_analysis_successful);
        // Making lexemes
        let lexemes = Lexer::new(tokens).run().unwrap();
        // AST Generation
        let ast = AstParser::new(lexemes).parse().unwrap();
        // AST Computing, Run #1
        let ast = ast.compute();

        assert_eq!(
            ast,
            Err(AstError::DivisionByZero(BinaryOperation {
                operation: BinaryOperationKind::Divide,
                left: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left: Box::new(Identifier("a".to_string())),
                    right: Box::new(Number(2.0)),
                }),
                right: Box::new(Number(0.0)),
            }))
        );
    }

    #[test]
    fn test_11_1() {
        let code = "1/(c*2*4.76*(1-2+1))";

        let tokens = Tokenizer::process(code);
        // Syntax Analysis
        let syntax_errors = SyntaxAnalyzer::new(&tokens).analyze();
        let is_syntax_analysis_successful = syntax_errors.is_empty();
        assert!(is_syntax_analysis_successful);
        // Making lexemes
        let lexemes = Lexer::new(tokens).run().unwrap();
        // AST Generation
        let ast = AstParser::new(lexemes).parse().unwrap();
        // AST Computing, Run #1
        let ast = ast.compute();

        assert_eq!(
            ast,
            Err(AstError::DivisionByZero(BinaryOperation {
                operation: BinaryOperationKind::Divide,
                left: Box::new(Number(1.0,)),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Multiply,
                        left: Box::new(BinaryOperation {
                            operation: BinaryOperationKind::Multiply,
                            left: Box::new(Identifier("c".to_string(),)),
                            right: Box::new(Number(2.0,)),
                        }),
                        right: Box::new(Number(4.76,)),
                    }),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(BinaryOperation {
                            operation: BinaryOperationKind::Minus,
                            left: Box::new(Number(1.0,)),
                            right: Box::new(Number(2.0,)),
                        }),
                        right: Box::new(Number(1.0,)),
                    }),
                }),
            }))
        );
    }

    #[test]
    fn test_11_2() {
        let code = "b/(b+b*0-1*b)";

        let tokens = Tokenizer::process(code);
        // Syntax Analysis
        let syntax_errors = SyntaxAnalyzer::new(&tokens).analyze();
        let is_syntax_analysis_successful = syntax_errors.is_empty();
        assert!(is_syntax_analysis_successful);
        // Making lexemes
        let lexemes = Lexer::new(tokens).run().unwrap();
        // AST Generation
        let ast = AstParser::new(lexemes).parse().unwrap();
        // AST Computing, Run #1
        let ast = ast.compute();

        assert_eq!(
            ast,
            Err(AstError::DivisionByZero(BinaryOperation {
                operation: BinaryOperationKind::Divide,
                left: Box::new(Identifier("b".to_string(),)),
                right: Box::new(BinaryOperation {
                    operation: BinaryOperationKind::Minus,
                    left: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Plus,
                        left: Box::new(Identifier("b".to_string(),)),
                        right: Box::new(BinaryOperation {
                            operation: BinaryOperationKind::Multiply,
                            left: Box::new(Identifier("b".to_string(),)),
                            right: Box::new(Number(0.0,)),
                        }),
                    }),
                    right: Box::new(BinaryOperation {
                        operation: BinaryOperationKind::Multiply,
                        left: Box::new(Number(1.0,)),
                        right: Box::new(Identifier("b".to_string(),)),
                    }),
                }),
            }))
        );
    }
}
