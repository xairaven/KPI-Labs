use crate::compiler::ast::tree::AbstractSyntaxTree;
use crate::compiler::reports::Reporter;
use crate::utils::StringBuffer;
use std::collections::{HashSet, VecDeque};

impl AbstractSyntaxTree {
    pub fn find_equivalent_forms(&self) -> Vec<AbstractSyntaxTree> {
        let mut all_forms: Vec<AbstractSyntaxTree> = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();
        let mut expansion_queue: VecDeque<AbstractSyntaxTree> = VecDeque::new();

        // --- Stage 1: Expansion (Distributive Law, Nodes 0-7 from memo) ---
        // This stage finds all forms *only* by expanding parentheses.
        // It performs a Breadth-First Search (BFS) starting from the original expression.

        let initial_key = self.to_canonical_string();
        expansion_queue.push_back(self.clone());
        visited.insert(initial_key);
        all_forms.push(self.clone());

        let mut fully_expanded_node: Option<AbstractSyntaxTree> = None;

        while let Some(current_ast) = expansion_queue.pop_front() {
            // Get all possible next forms by applying *one step* of expansion
            let expansion_steps = current_ast.get_all_single_step_expansions();

            // If a node has no possible expansions, it's a "leaf" in this stage.
            // We assume the first one we find is the fully expanded form (Node 7).
            if expansion_steps.is_empty() && fully_expanded_node.is_none() {
                fully_expanded_node = Some(current_ast.clone());
            }

            for expanded_ast in expansion_steps {
                let key = expanded_ast.to_canonical_string();
                if !visited.contains(&key) {
                    visited.insert(key.clone());
                    all_forms.push(expanded_ast.clone());
                    expansion_queue.push_back(expanded_ast);
                }
            }
        }

        // --- Stage 1.5: Flattening ---
        // We take the "fully expanded" node (Node 7) and apply unary minus
        // rules like `-(A-B) -> -A+B` to get the *truly* flat form.
        // This is the form you're looking for, which has no parentheses.

        let Some(node_to_flatten) = fully_expanded_node else {
            log::warn!(
                "No fully expanded form (Node 7) found. Skipping Stage 1.5 and 2."
            );
            return all_forms;
        };

        let node_to_flatten_copy = node_to_flatten.clone();
        let start_node_for_factoring =
            match Self::transform_recursive(node_to_flatten.peek)
                .and_then(Self::fold_recursive)
            {
                Ok(flattened_node_peek) => {
                    let flattened_ast =
                        AbstractSyntaxTree::from_node(flattened_node_peek);
                    let key = flattened_ast.to_canonical_string();
                    if !visited.contains(&key) {
                        // Add this new, truly flat form if it's unique
                        visited.insert(key);
                        all_forms.push(flattened_ast.clone());
                    }
                    flattened_ast // This is the new starting point for factoring
                },
                Err(e) => {
                    log::error!("Failed to flatten Node 7: {:?}. Using un-flattened.", e);
                    node_to_flatten_copy // Fallback to the un-flattened node
                },
            };

        // --- Stage 2: Factoring (Associative Law, Nodes 7-13) ---
        // This stage now starts from the *truly flat* form.

        let mut factoring_queue: VecDeque<AbstractSyntaxTree> = VecDeque::new();
        factoring_queue.push_back(start_node_for_factoring);
        // We don't need to re-add to `visited` or `all_forms`,
        // as it was handled in Stage 1.5

        while let Some(current_ast) = factoring_queue.pop_front() {
            let factoring_steps = current_ast.get_all_single_step_factorings();

            for factored_ast in factoring_steps {
                let key = factored_ast.to_canonical_string();
                if !visited.contains(&key) {
                    visited.insert(key.clone());
                    all_forms.push(factored_ast.clone());
                    factoring_queue.push_back(factored_ast);
                }
            }
        }

        all_forms
    }
}

impl Reporter {
    pub fn finding_equivalent_form(&self, forms: &[String]) -> String {
        let mut buffer = StringBuffer::default();

        buffer.add_line(format!("Found {} equivalent forms!\n", forms.len() - 1));

        for (index, form) in forms.iter().enumerate() {
            buffer.add_line(format!("{}) {}", index, form));
            if index == 0 {
                buffer.add_line("\n".to_string());
            }
        }

        buffer.get()
    }
}
