use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::Object;

/// The environment.
pub struct Env<'i> {
    /// The parent environment.
    #[allow(dead_code)]
    parent: Option<Rc<RefCell<Env<'i>>>>,
    /// The variables in the environment.
    #[allow(dead_code)]
    vars: HashMap<String, Object<'i>>,
}

impl<'i> Env<'i> {
    /// Creates a new environment.
    /// 
    /// # Returns
    /// A new environment.
    /// 
    /// # Examples
    /// ```
    /// use lisp::repl::env::Env;
    /// 
    /// let env = Env::new();
    /// ```
    pub fn new() -> Self {
        Env {
            parent: None,
            vars: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `Env::new` function.
    #[test]
    fn test_env() {
        let env = Env::new();
        assert!(env.vars.is_empty());
        assert!(env.parent.is_none());
    }
}
