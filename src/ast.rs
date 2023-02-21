/// Object is the type of the values in the language.
#[derive(Debug, PartialEq)]
pub enum Object<'i> {
    /// The void value
    Void,
    /// A boolean value
    Bool(bool),
    /// An integer value
    Integer(i64),
    /// A symbol
    Symbol(&'i str),
    /// A list of objects
    List(Vec<Object<'i>>),
    /// A function
    Lambda(Vec<&'i str>, Vec<Object<'i>>),
}

impl<'i> std::fmt::Display for Object<'i> {
    /// Display an object.
    /// 
    /// # Examples
    /// ```
    /// use lisp::ast::Object;
    /// 
    /// let obj = Object::Void;
    /// assert_eq!(obj.to_string(), "Void");
    /// 
    /// let obj = Object::Bool(true);
    /// assert_eq!(obj.to_string(), "true");
    /// 
    /// let obj = Object::Integer(42);
    /// assert_eq!(obj.to_string(), "42");
    /// 
    /// let obj = Object::Symbol("foo");
    /// assert_eq!(obj.to_string(), "foo");
    /// 
    /// let obj = Object::List(vec![Object::Integer(1), Object::Integer(2)]);
    /// assert_eq!(obj.to_string(), "(1 2)");
    /// 
    /// let obj = Object::Lambda(vec!["x", "y"], vec![Object::Integer(1), Object::Integer(2)]);
    /// assert_eq!(obj.to_string(), "(lambda (x y) 1 2)");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Integer(i) => write!(f, "{}", i),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::List(l) => {
                write!(f, "(")?;
                for (i, o) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", o)?;
                }
                write!(f, ")")
            }
            Object::Lambda(args, body) => {
                write!(f, "(lambda (")?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")?;
                for o in body {
                    write!(f, " {}", o)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `fmt` method.
    #[test]
    fn test_fmt() {
        let obj = Object::Void;
        assert_eq!(obj.to_string(), "Void");

        let obj = Object::Bool(true);
        assert_eq!(obj.to_string(), "true");

        let obj = Object::Integer(42);
        assert_eq!(obj.to_string(), "42");

        let obj = Object::Symbol("foo");
        assert_eq!(obj.to_string(), "foo");

        let obj = Object::List(vec![Object::Integer(1), Object::Integer(2)]);
        assert_eq!(obj.to_string(), "(1 2)");

        let obj = Object::Lambda(vec!["x", "y"], vec![Object::Integer(1), Object::Integer(2)]);
        assert_eq!(obj.to_string(), "(lambda (x y) 1 2)");
    }
}