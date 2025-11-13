use crate::compiler::ast::tree::{AbstractSyntaxTree, AstError, AstParser};
use crate::compiler::lexer::{Lexeme, Lexer, LexerError};
use crate::compiler::reports::Reporter;
use crate::compiler::syntax::{SyntaxAnalyzer, SyntaxError};
use crate::compiler::tokenizer::{Token, Tokenizer};
use crate::config::Config;

pub struct CompilerContext {
    pub code: String,
    pub pretty_output: bool,
}

impl CompilerContext {
    pub fn new(config: &Config) -> Self {
        Self {
            code: String::new(),
            pretty_output: config.pretty_output,
        }
    }

    fn tokenize(&self) -> Vec<Token> {
        Tokenizer::process(&self.code)
    }

    pub fn tokenize_report(&self) -> String {
        Tokenizer::report(&self.tokenize())
    }

    fn check_syntax(&self) -> Vec<SyntaxError> {
        let tokens = self.tokenize();
        SyntaxAnalyzer::new(&tokens).analyze()
    }

    pub fn syntax_report(&self) -> String {
        Reporter.syntax(&self.code, self.pretty_output, &self.check_syntax())
    }

    fn create_lexemes(&self) -> Result<Result<Vec<Lexeme>, LexerError>, String> {
        let tokens = self.tokenize();
        let syntax_errors = self.check_syntax();
        if !syntax_errors.is_empty() {
            return Err(self.syntax_report());
        }
        let lexemes = Lexer::new(tokens).run();
        Ok(lexemes)
    }

    pub fn lexer_report(&self) -> String {
        match self.create_lexemes() {
            Ok(lexer_result) => Reporter.lexemes_creation(&lexer_result),
            Err(syntax_error) => syntax_error,
        }
    }

    fn create_ast(&self) -> Result<Result<AbstractSyntaxTree, AstError>, String> {
        let lexer_result = self.create_lexemes()?;
        let lexemes = match lexer_result {
            Ok(value) => value,
            Err(_) => return Err(Reporter.lexemes_creation(&lexer_result)),
        };

        Ok(AstParser::new(lexemes).parse())
    }

    pub fn ast_report(&self) -> String {
        match self.create_ast() {
            Ok(ast_result) => Reporter.tree_build(&ast_result),
            Err(error) => error,
        }
    }

    fn compute_ast_1(&self) -> Result<Result<AbstractSyntaxTree, AstError>, String> {
        let ast_creation_result = self.create_ast()?;
        let ast = match ast_creation_result {
            Ok(value) => value,
            Err(_) => return Err(Reporter.tree_build(&ast_creation_result)),
        };

        Ok(ast.compute())
    }

    pub fn compute_1_report(&self) -> String {
        match self.compute_ast_1() {
            Ok(compute_result) => Reporter.computing(&compute_result, 1),
            Err(error) => error,
        }
    }

    fn transform_ast(&self) -> Result<Result<AbstractSyntaxTree, AstError>, String> {
        let ast_compute_result = self.compute_ast_1()?;
        let ast = match ast_compute_result {
            Ok(value) => value,
            Err(_) => return Err(Reporter.computing(&ast_compute_result, 1)),
        };

        if ast.is_finalized() {
            return Err(Reporter.computing_finalization());
        }

        Ok(ast.transform())
    }

    pub fn transform_report(&self) -> String {
        match self.transform_ast() {
            Ok(transform_result) => Reporter.transforming(&transform_result),
            Err(error) => error,
        }
    }

    fn compute_ast_2(&self) -> Result<Result<AbstractSyntaxTree, AstError>, String> {
        let ast_transformation_result = self.transform_ast()?;
        let ast = match ast_transformation_result {
            Ok(value) => value,
            Err(_) => {
                return Err(Reporter.transforming(&ast_transformation_result));
            },
        };

        Ok(ast.compute())
    }

    pub fn compute_2_report(&self) -> String {
        match self.compute_ast_2() {
            Ok(compute_result) => Reporter.computing(&compute_result, 2),
            Err(error) => error,
        }
    }

    fn balance_ast(&self) -> Result<Result<AbstractSyntaxTree, AstError>, String> {
        let ast_compute_result = self.compute_ast_2()?;
        let ast = match ast_compute_result {
            Ok(value) => value,
            Err(_) => return Err(Reporter.computing(&ast_compute_result, 2)),
        };

        if ast.is_finalized() {
            return Err(Reporter.computing_finalization());
        }

        Ok(ast.balance())
    }

    pub fn balance_report(&self) -> String {
        match self.balance_ast() {
            Ok(balance_result) => Reporter.balancing(&balance_result),
            Err(error) => error,
        }
    }

    fn compute_ast_3(&self) -> Result<Result<AbstractSyntaxTree, AstError>, String> {
        let ast_balance_result = self.balance_ast()?;
        let ast = match ast_balance_result {
            Ok(value) => value,
            Err(_) => return Err(Reporter.balancing(&ast_balance_result)),
        };

        Ok(ast.compute())
    }

    pub fn compute_3_report(&self) -> String {
        match self.compute_ast_3() {
            Ok(compute_result) => Reporter.computing(&compute_result, 3),
            Err(error) => error,
        }
    }

    fn folding_ast(&self) -> Result<Result<AbstractSyntaxTree, AstError>, String> {
        let ast_compute_result = self.compute_ast_3()?;
        let ast = match ast_compute_result {
            Ok(value) => value,
            Err(_) => return Err(Reporter.computing(&ast_compute_result, 3)),
        };

        if ast.is_finalized() {
            return Err(Reporter.computing_finalization());
        }

        Ok(ast.fold())
    }

    pub fn folding_report(&self) -> String {
        match self.folding_ast() {
            Ok(folding_result) => Reporter.folding(&folding_result),
            Err(error) => error,
        }
    }

    fn compute_ast_4(&self) -> Result<Result<AbstractSyntaxTree, AstError>, String> {
        let ast_folding_result = self.folding_ast()?;
        let ast = match ast_folding_result {
            Ok(value) => value,
            Err(_) => return Err(Reporter.folding(&ast_folding_result)),
        };

        Ok(ast.compute())
    }

    pub fn compute_4_report(&self) -> String {
        match self.compute_ast_4() {
            Ok(compute_result) => Reporter.computing(&compute_result, 4),
            Err(error) => error,
        }
    }

    fn find_equivalent_forms(&self) -> Result<Vec<String>, String> {
        let ast_computing_result = self.compute_ast_4()?;
        let ast = match ast_computing_result {
            Ok(value) => value,
            Err(_) => return Err(Reporter.computing(&ast_computing_result, 4)),
        };

        let forms = ast.find_equivalent_forms();

        Ok(forms.iter().map(|form| form.to_pretty_string()).collect())
    }

    pub fn equivalent_forms_report(&self) -> String {
        match self.find_equivalent_forms() {
            Ok(forms) => Reporter.finding_equivalent_form(&forms),
            Err(error) => error,
        }
    }
}
