use crate::compiler::ast::tree::{
    AbstractSyntaxTree, AstError, AstNode, BinaryOperationKind,
};
use std::collections::{HashSet, VecDeque};

impl AbstractSyntaxTree {
    /// Returns a VECTOR of trees, where each tree is ONE step of the factoring.
    pub fn get_all_single_step_factorings(&self) -> Vec<AbstractSyntaxTree> {
        let mut all_new_forms = Vec::new();
        // We start factoring from the root node.
        Self::get_all_possible_factorings(self.peek.clone(), &mut all_new_forms);
        all_new_forms
    }

    /// Finds *all* possible *single* factoring steps from the current node.
    fn get_all_possible_factorings(
        node: AstNode, all_forms: &mut Vec<AbstractSyntaxTree>,
    ) {
        // We can only factor terms from an addition or subtraction chain.
        match &node {
            AstNode::BinaryOperation { operation, .. }
                if *operation == BinaryOperationKind::Plus
                    || *operation == BinaryOperationKind::Minus =>
            {
                // This is a node we can start collecting from.
            },
            _ => return, // Not an additive/subtractive node, nothing to factor.
        };

        // --- STEP 1: Flatten the expression ---
        // This is the CRITICAL FIX. We flatten the tree into a list of terms.
        // `ak - ck - ax` will become `[ (ak), (-ck), (-ax) ]`
        let mut summands = Vec::new();
        Self::local_collect_operands(node, &mut summands);

        if summands.len() < 2 {
            return; // Not enough terms to factor.
        }

        // --- STEP 2: Find all possible factor groupings ---
        let mut unique_factors: HashSet<String> = HashSet::new();

        // Iterate over every term to use as the "base" for a group
        for i in 0..summands.len() {
            let factors_i = Self::get_factors(&summands[i]);
            if factors_i.is_empty() {
                continue; // This term (e.g., 'a') has no factors.
            }

            // Iterate over each factor of the base term (e.g., 'a' and 'k' for 'ak')
            for factor in factors_i {
                let tree = AbstractSyntaxTree::from_node(factor);
                let factor_key = tree.to_canonical_string();
                let factor = tree.peek;

                // We use a HashSet to avoid generating duplicates.
                if !unique_factors.insert(factor_key) {
                    continue;
                }

                // Get the remainder for the base term (e.g., 'k' from 'ak' with factor 'a')
                if let Some(remainder_i) = Self::get_remainder(&summands[i], &factor) {
                    let mut current_group_terms = vec![remainder_i];
                    let mut current_group_indices = vec![i];

                    // --- STEP 3: Find other terms with the same factor ---
                    for (j, summand) in summands.iter().enumerate().skip(i + 1) {
                        // e.g., check '-ax' for factor 'a'
                        if let Some(remainder_j) = Self::get_remainder(summand, &factor) {
                            // Remainder will be '(-x)'. This is correct.
                            current_group_terms.push(remainder_j);
                            current_group_indices.push(j);
                        }
                    }

                    // --- STEP 4: Build the new factored form ---
                    if current_group_terms.len() > 1 {
                        // We found a group! (e.g., factor 'a' with remainders [k, (-x)])
                        let mut new_summands = Vec::new();

                        // 1. Create the new factored node: a * (k + (-x))
                        // We *always* sum the remainders with `Plus`.
                        let sum_of_terms = Self::local_build_balanced_tree(
                            current_group_terms,
                            BinaryOperationKind::Plus,
                        )
                        .unwrap_or(AstNode::Number(0.0)); // Should not happen

                        let new_node = AstNode::BinaryOperation {
                            operation: BinaryOperationKind::Multiply,
                            left: Box::new(factor.clone()),
                            right: Box::new(sum_of_terms),
                        };
                        new_summands.push(new_node);

                        // 2. Add all the terms that were *not* part of this group
                        for (idx, term) in summands.iter().enumerate() {
                            if !current_group_indices.contains(&idx) {
                                new_summands.push(term.clone());
                            }
                        }

                        // 3. Re-assemble the final tree from all terms
                        // We *always* build the final tree with `Plus` as well.
                        if let Ok(final_node) = Self::local_build_balanced_tree(
                            new_summands,
                            BinaryOperationKind::Plus,
                        ) {
                            all_forms.push(AbstractSyntaxTree::from_node(final_node));
                        }
                    }
                }
            }
        }
    }

    /// Flattens a chain of `Plus`/`Minus` nodes into a flat Vec of operands.
    /// `A - B` is treated as `A` and `(-B)`.
    /// `ak - ck - ax` -> `[ (ak), (-ck), (-ax) ]`
    fn local_collect_operands(node: AstNode, operands: &mut Vec<AstNode>) {
        match node {
            // Case: A + B
            AstNode::BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left,
                right,
            } => {
                // Collect operands from both sides
                Self::local_collect_operands(*left, operands);
                Self::local_collect_operands(*right, operands);
            },
            // Case: A - B
            AstNode::BinaryOperation {
                operation: BinaryOperationKind::Minus,
                left,
                right,
            } => {
                // Collect operands from the left side
                Self::local_collect_operands(*left, operands);
                // Collect operands from the right side, but wrap them in UnaryMinus
                Self::local_collect_operands_with_minus(*right, operands);
            },
            // Base case: A standalone term (like 'ak' or '-ck')
            _ => {
                operands.push(node);
            },
        }
    }

    /// Helper for `local_collect_operands` to correctly apply UnaryMinus.
    /// This handles `-(A+B) -> -A + -B` and `-(-A) -> A`.
    fn local_collect_operands_with_minus(node: AstNode, operands: &mut Vec<AstNode>) {
        match node {
            // Case: -(-A) => A
            AstNode::UnaryOperation {
                operation: crate::compiler::ast::tree::UnaryOperationKind::Minus,
                expression,
            } => {
                Self::local_collect_operands(*expression, operands);
            },
            // Case: -(A + B) => -A, -B
            AstNode::BinaryOperation {
                operation: BinaryOperationKind::Plus,
                left,
                right,
            } => {
                Self::local_collect_operands_with_minus(*left, operands);
                Self::local_collect_operands_with_minus(*right, operands);
            },
            // Case: -(A - B) => -A, B
            AstNode::BinaryOperation {
                operation: BinaryOperationKind::Minus,
                left,
                right,
            } => {
                Self::local_collect_operands_with_minus(*left, operands);
                Self::local_collect_operands(*right, operands); // Note: right side becomes positive
            },
            // Base case: -(term) => push(-term)
            _ => {
                operands.push(AstNode::UnaryOperation {
                    operation: crate::compiler::ast::tree::UnaryOperationKind::Minus,
                    expression: Box::new(node),
                });
            },
        }
    }

    /// Builds a balanced tree from a list of operands.
    /// (This is a standard helper function, same as in balancer.rs)
    fn local_build_balanced_tree(
        operands: Vec<AstNode>, op_kind: BinaryOperationKind,
    ) -> Result<AstNode, AstError> {
        if operands.is_empty() {
            return Err(AstError::CannotBuildEmptyTree);
        }
        let mut queue: VecDeque<AstNode> = operands.into();
        while queue.len() > 1 {
            let level_size = queue.len();
            for _ in 0..(level_size / 2) {
                let left = queue.pop_front().ok_or(AstError::FailedPopFromQueue)?;
                let right = queue.pop_front().ok_or(AstError::FailedPopFromQueue)?;
                let new_node = AstNode::BinaryOperation {
                    operation: op_kind.clone(),
                    left: Box::new(left),
                    right: Box::new(right),
                };
                queue.push_back(new_node);
            }
            if !level_size.is_multiple_of(2) {
                let odd_one_out =
                    queue.pop_front().ok_or(AstError::FailedPopFromQueue)?;
                queue.push_back(odd_one_out);
            }
        }
        queue.pop_front().ok_or(AstError::FailedPopFromQueue)
    }

    /// Helper function: finds the factors for a term.
    /// `A * B` -> `[A, B]`
    /// `-(A * B)` -> `[A, B]` (sign is handled by get_remainder)
    fn get_factors(node: &AstNode) -> Vec<AstNode> {
        match node {
            // Case: A * B
            AstNode::BinaryOperation {
                operation: BinaryOperationKind::Multiply,
                left,
                right,
            } => {
                vec![*left.clone(), *right.clone()]
            },
            // Case: -(A * B)
            AstNode::UnaryOperation {
                operation: crate::compiler::ast::tree::UnaryOperationKind::Minus,
                expression,
            } => {
                if let AstNode::BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left,
                    right,
                } = expression.as_ref()
                {
                    // Factors are A and B. The minus sign is part of the "remainder".
                    vec![*left.clone(), *right.clone()]
                } else {
                    vec![]
                }
            },
            _ => vec![], // Not a multiplication, no factors
        }
    }

    /// Helper function: for `node` (term) and `factor`, returns the remainder.
    /// `(A * B, A)` -> `B`
    /// `(-(A * B), A)` -> `(-B)`
    fn get_remainder(node: &AstNode, factor: &AstNode) -> Option<AstNode> {
        match node {
            // Case: A * B
            AstNode::BinaryOperation {
                operation: BinaryOperationKind::Multiply,
                left,
                right,
            } => {
                if left.as_ref() == factor {
                    return Some(*right.clone()); // (A * B) / A = B
                }
                if right.as_ref() == factor {
                    return Some(*left.clone()); // (A * B) / B = A
                }
                None
            },
            // Case: -(A * B)
            AstNode::UnaryOperation {
                operation: op @ crate::compiler::ast::tree::UnaryOperationKind::Minus,
                expression,
            } => {
                if let AstNode::BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left,
                    right,
                } = expression.as_ref()
                {
                    if left.as_ref() == factor {
                        // -(A * B) / A = -B
                        return Some(AstNode::UnaryOperation {
                            operation: op.clone(),
                            expression: right.clone(),
                        });
                    }
                    if right.as_ref() == factor {
                        // -(A * B) / B = -A
                        return Some(AstNode::UnaryOperation {
                            operation: op.clone(),
                            expression: left.clone(),
                        });
                    }
                }
                None
            },
            _ => None,
        }
    }
}
