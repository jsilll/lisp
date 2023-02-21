/// The `env` module contains the code for the environment.
pub mod env;

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::Object;
use crate::repl::env::Env;

/// The `Evaluator` struct is used to evaluate an AST.
/// It contains the AST and the environment.
pub struct Evaluator<'i> {
    /// The AST to evaluate.   
    #[allow(dead_code)]
    ast: Object<'i>,
    /// The environment.
    #[allow(dead_code)]
    env: Rc<RefCell<Env<'i>>>,
}

impl<'i> Evaluator<'i> {
    /// Creates a new evaluator.
    /// 
    /// # Arguments
    /// * `ast` - The AST to evaluate.
    /// 
    /// # Returns
    /// A new evaluator.
    /// 
    /// # Examples
    /// ```
    /// use lisp::repl::Evaluator;
    /// use lisp::ast::Object;
    /// 
    /// let evaluator = Evaluator::new(Object::Void);
    /// ```
    #[allow(dead_code)]
    pub fn new(ast: Object<'i>) -> Self {
        Evaluator {
            ast: ast,
            env: Rc::new(RefCell::new(Env::new())),
        }
    }

    /// Evaluates the AST.
    /// 
    /// # Returns
    /// The result of the evaluation.
    /// 
    /// # Examples
    /// ```
    /// use lisp::repl::Evaluator;
    /// use lisp::ast::Object;
    /// 
    /// let evaluator = Evaluator::new(Object::Void);
    /// let result = evaluator.eval();
    /// ```
    #[allow(dead_code)]
    pub fn eval(&self) -> Object<'i> {
        Object::Void
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `Evaluator::new` function.
    #[test]
    fn test_new() {
        let evaluator = Evaluator::new(Object::Void);
        let result = evaluator.eval();
        assert_eq!(result, Object::Void);
    }
}