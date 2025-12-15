use crate::compiler::ast::tree::{
    AbstractSyntaxTree, AstError, AstNode, BinaryOperationKind, UnaryOperationKind,
};
use crate::compiler::reports::Reporter;
use crate::utils::StringBuffer;

impl AbstractSyntaxTree {
    pub fn fold(self) -> Result<AbstractSyntaxTree, AstError> {
        let folded = Self::fold_recursive(self.peek)?;

        Ok(Self::from_node(folded))
    }

    pub fn fold_recursive(node: AstNode) -> Result<AstNode, AstError> {
        match &node {
            AstNode::Number(_) | AstNode::Identifier(_) | AstNode::StringLiteral(_) => {
                Ok(node)
            },
            AstNode::UnaryOperation {
                operation,
                expression,
            } => {
                let folded_child = Self::fold_recursive(*expression.clone())?;
                Ok(AstNode::UnaryOperation {
                    operation: operation.clone(),
                    expression: Box::new(folded_child),
                })
            },
            AstNode::BinaryOperation {
                operation,
                left,
                right,
            } => {
                let folded_left = Self::fold_recursive(*left.clone())?;
                let folded_right = Self::fold_recursive(*right.clone())?;

                match operation {
                    BinaryOperationKind::Plus => {
                        if let AstNode::UnaryOperation {
                            operation,
                            expression,
                        } = &folded_right
                            && operation.eq(&UnaryOperationKind::Minus)
                        {
                            return Ok(AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Minus,
                                left: Box::new(folded_left),
                                right: expression.clone(),
                            });
                        }

                        if let AstNode::Number(number) = &folded_right
                            && number.is_sign_negative()
                        {
                            return Ok(AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Minus,
                                left: Box::new(folded_left),
                                right: Box::new(AstNode::Number(-number)),
                            });
                        }
                    },
                    BinaryOperationKind::Multiply => {
                        if let AstNode::BinaryOperation {
                            operation,
                            left,
                            right,
                        } = &folded_right
                            && operation.eq(&BinaryOperationKind::Divide)
                            && let AstNode::Number(number) = **left
                            && [1.0, -1.0].contains(&number)
                        {
                            return Ok(AstNode::BinaryOperation {
                                operation: BinaryOperationKind::Divide,
                                left: Box::new(folded_left),
                                right: right.clone(),
                            });
                        }
                    },
                    _ => {},
                }

                Ok(AstNode::BinaryOperation {
                    operation: operation.clone(),
                    left: Box::new(folded_left),
                    right: Box::new(folded_right),
                })
            },
            AstNode::FunctionCall { name, arguments } => {
                let folded_arguments: Result<Vec<AstNode>, AstError> = arguments
                    .iter()
                    .map(|arg| Self::fold_recursive(arg.clone()))
                    .collect();

                Ok(AstNode::FunctionCall {
                    name: name.clone(),
                    arguments: folded_arguments?,
                })
            },
            AstNode::ArrayAccess {
                identifier,
                indices,
            } => {
                let folded_indices: Result<Vec<AstNode>, AstError> = indices
                    .iter()
                    .map(|index| Self::fold_recursive(index.clone()))
                    .collect();

                Ok(AstNode::ArrayAccess {
                    identifier: identifier.clone(),
                    indices: folded_indices?,
                })
            },
        }
    }
}

impl Reporter {
    pub fn folding(&self, result: &Result<AbstractSyntaxTree, AstError>) -> String {
        let mut buffer = StringBuffer::default();

        match result {
            Ok(tree) => {
                buffer.add_line("Folding Abstract-Syntax Tree success!\n".to_string());
                buffer.add_line(tree.pretty_print());
            },
            Err(error) => buffer.add_line(format!("Folding AST error: {}", error)),
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
        // AST Computing, Run #3
        let ast = compute_run(ast, 3)?;
        // AST Folding
        let ast_result = ast.fold();
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
                operation: BinaryOperationKind::Minus,
                left: Box::new(Identifier("i".to_string())),
                right: Box::new(Number(0.3999999999999999)),
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
