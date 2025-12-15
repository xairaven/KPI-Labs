use crate::compiler::ast::tree::{AbstractSyntaxTree, AstNode, BinaryOperationKind};

// A "path" is a sequence of 0s and 1s (and 2s for Unary)
// 0 = left child, 1 = right child, 2 = unary expression
type NodePath = Vec<u8>;

impl AbstractSyntaxTree {
    /// Returns a vector of AbstractSyntaxTree, each representing a single-step expansion.
    pub fn get_all_single_step_expansions(&self) -> Vec<AbstractSyntaxTree> {
        let mut expandable_nodes_paths: Vec<NodePath> = Vec::new();

        // 1. Find all paths to nodes that can be expanded
        Self::find_expandable_nodes_recursive(
            &self.peek,
            &mut vec![],
            &mut expandable_nodes_paths,
        );

        let mut all_forms = Vec::new();

        // 2. For each found path...
        for path in expandable_nodes_paths {
            // Create a fresh copy of the tree for this single expansion
            let mut new_tree = self.clone();

            // 3. Get a mutable reference to the node at that path
            if let Some(target_node) =
                Self::get_node_mut_by_path(&mut new_tree.peek, &path)
            {
                // 4. Replace that node with its expanded version
                // `perform_expansion` is guaranteed to expand *this* node
                *target_node = Self::perform_expansion(target_node.clone());
            }
            all_forms.push(new_tree);
        }

        all_forms
    }

    /// Recursively finds nodes matching the pattern `A * (B +/- C)` or `(A +/- B) * C`
    /// OR `(A +/- B) / C`.
    fn find_expandable_nodes_recursive(
        node: &AstNode, path: &mut NodePath, paths: &mut Vec<NodePath>,
    ) {
        // We now check for BOTH Multiply and Divide at the root
        if let AstNode::BinaryOperation {
            operation: op @ (BinaryOperationKind::Multiply | BinaryOperationKind::Divide),
            left,
            right,
        } = node
        {
            // Pattern 1: (A +/- B) * C  OR  (A +/- B) / C
            // This is the pattern that matches `(a-b)/(c-d)`
            if let AstNode::BinaryOperation {
                operation: op_child,
                ..
            } = left.as_ref()
                && (*op_child == BinaryOperationKind::Plus
                    || *op_child == BinaryOperationKind::Minus)
            {
                // This `Multiply` or `Divide` node can be expanded. Save its path.
                paths.push(path.clone());
            }

            // Pattern 2: A * (B +/- C)
            // We only do this for Multiply.
            // We DO NOT expand `A / (B + C)`.
            if *op == BinaryOperationKind::Multiply
                && let AstNode::BinaryOperation {
                    operation: op_child,
                    ..
                } = right.as_ref()
                && (*op_child == BinaryOperationKind::Plus
                    || *op_child == BinaryOperationKind::Minus)
            {
                // This `Multiply` node can also be expanded. Save its path.
                paths.push(path.clone());
            }
        }

        // Recursive traversal to check children
        match node {
            AstNode::BinaryOperation { left, right, .. } => {
                path.push(0); // 0 = left
                Self::find_expandable_nodes_recursive(left, path, paths);
                path.pop();

                path.push(1); // 1 = right
                Self::find_expandable_nodes_recursive(right, path, paths);
                path.pop();
            },
            AstNode::UnaryOperation { expression, .. } => {
                path.push(2); // 2 = expression
                Self::find_expandable_nodes_recursive(expression, path, paths);
                path.pop();
            },
            // Base cases (Number, Identifier): nowhere else to go
            _ => {},
        }
    }

    /// Helper function to get mutable reference to a node by path
    pub fn get_node_mut_by_path<'a>(
        node: &'a mut AstNode, path: &[u8],
    ) -> Option<&'a mut AstNode> {
        let mut current = node;
        for &index in path {
            match current {
                AstNode::BinaryOperation { left, right, .. } => {
                    current = if index == 0 { left } else { right };
                },
                AstNode::UnaryOperation { expression, .. } => {
                    current = expression;
                },
                _ => return None, // Path is invalid
            }
        }
        Some(current)
    }

    /// Performs ONE unfolding on a node that is guaranteed to be `Multiply` or `Divide`
    /// and have at least one child that is `Plus` or `Minus` in the correct position.
    fn perform_expansion(node: AstNode) -> AstNode {
        // --- Block 1: Handle Multiply ---
        if let AstNode::BinaryOperation {
            operation: BinaryOperationKind::Multiply,
            left,
            right,
        } = node
        {
            // Case 1: A * (B +/- C)
            if let AstNode::BinaryOperation {
                operation: op @ (BinaryOperationKind::Plus | BinaryOperationKind::Minus),
                left: b,  // B
                right: c, // C
            } = *right
            {
                // Create (A * B)
                let new_left = AstNode::BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left: left.clone(), // A
                    right: b,
                };
                // Create (A * C)
                let new_right = AstNode::BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left, // A
                    right: c,
                };
                // Return (A*B) +/- (A*C)
                return AstNode::BinaryOperation {
                    operation: op,
                    left: Box::new(new_left),
                    right: Box::new(new_right),
                };
            }

            // Case 2: (A +/- B) * C
            if let AstNode::BinaryOperation {
                operation: op @ (BinaryOperationKind::Plus | BinaryOperationKind::Minus),
                left: a,  // A
                right: b, // B
            } = *left
            {
                // Create (A * C)
                let new_left = AstNode::BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left: a,
                    right: right.clone(), // C
                };
                // Create (B * C)
                let new_right = AstNode::BinaryOperation {
                    operation: BinaryOperationKind::Multiply,
                    left: b,
                    right, // C
                };
                // Return (A*C) +/- (B*C)
                return AstNode::BinaryOperation {
                    operation: op,
                    left: Box::new(new_left),
                    right: Box::new(new_right),
                };
            }

            // If patterns didn't match
            return AstNode::BinaryOperation {
                operation: BinaryOperationKind::Multiply,
                left,
                right,
            };
        }

        // --- Block 2: Handle Divide ---
        // This block handles the `(a - b) / (c - d)` case
        if let AstNode::BinaryOperation {
            operation: BinaryOperationKind::Divide,
            left,
            right,
        } = node
        {
            // We ONLY handle Case 2: (A +/- B) / C
            // We DO NOT handle A / (B +/- C)
            if let AstNode::BinaryOperation {
                operation: op @ (BinaryOperationKind::Plus | BinaryOperationKind::Minus),
                left: a,  // A
                right: b, // B
            } = *left
            {
                // Create (A / C)
                let new_left = AstNode::BinaryOperation {
                    operation: BinaryOperationKind::Divide, // Use Divide
                    left: a,
                    right: right.clone(), // C
                };
                // Create (B / C)
                let new_right = AstNode::BinaryOperation {
                    operation: BinaryOperationKind::Divide, // Use Divide
                    left: b,
                    right, // C
                };
                // Return (A/C) +/- (B/C)
                return AstNode::BinaryOperation {
                    operation: op, // Keep the original Plus or Minus
                    left: Box::new(new_left),
                    right: Box::new(new_right),
                };
            }

            // If pattern (A+B)/C didn't match, return the original node
            return AstNode::BinaryOperation {
                operation: BinaryOperationKind::Divide,
                left,
                right,
            };
        }

        // Return the node as is if it's not `Multiply` or `Divide`
        node
    }
}
