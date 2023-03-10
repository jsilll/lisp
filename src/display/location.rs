/// Location in a source file.
///
/// # Example
/// ```
/// use lisp::display::location::Location;
///
/// let loc = Location::new("path", "1 + 2", 3);
/// assert_eq!(loc.path, "path");
/// assert_eq!(loc.line, 1);
/// assert_eq!(loc.column, 4);
/// ```
pub struct Location<'i> {
    /// The file path.
    pub path: &'i str,
    /// The line number.
    pub line: usize,
    /// The column number.
    pub column: usize,
}

impl<'i> Location<'i> {
    /// Create a new `Location` from a source string and an index into
    /// the string.
    ///
    /// # Arguments
    /// - `path`: The path to the source file.
    /// - `source`: The source string.
    /// - `idx`: The index into the source string.
    ///
    /// # Returns
    /// A new `Location` struct.
    ///
    /// # Example
    /// ```
    /// use lisp::display::location::Location;
    ///
    /// let loc = Location::new("path", "1 + 2", 3);
    /// assert_eq!(loc.path, "path");
    /// assert_eq!(loc.line, 1);
    /// assert_eq!(loc.column, 4);
    /// ```
    pub fn new(path: &'i str, source: &str, idx: usize) -> Self {
        let idx = std::cmp::min(idx, source.len());
        let line = source[..std::cmp::min(idx + 1, source.len())].lines().count();
        let column = match source[..idx].rfind('\n') {
            None => idx + 1,
            Some(i) => idx - i,
        };
        Self { path, line, column }
    }
}

impl std::fmt::Display for Location<'_> {
    /// Display the location in a human-readable format.
    /// Additionally, the location is printed in a format that can be interpreted by an IDE.
    ///
    /// # Example
    /// ```
    /// use lisp::display::location::Location;
    ///
    /// let loc = Location::new("path", "1 + 2", 3);
    /// assert_eq!(loc.to_string(), "path:1:4");
    ///
    /// let loc = Location::new("path", "1 + 2\n3 + 4", 7);
    /// assert_eq!(loc.to_string(), "path:2:2");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.path, self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the `Location` struct.
    #[test]
    fn test_location() {
        let loc = Location::new("path", "1 + 2", 3);
        assert_eq!(loc.path, "path");
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 4);

        let loc = Location::new("path", "1 + 2", 0);
        assert_eq!(loc.path, "path");
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 1);

        let loc = Location::new("path", "1 + 2", 5);
        assert_eq!(loc.path, "path");
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 6);

        let loc = Location::new("path", "1 + 2\n3 + 4", 5);
        assert_eq!(loc.path, "path");
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 6);

        let loc = Location::new("path", "1 + 2\n3 + 4", 6);
        assert_eq!(loc.path, "path");
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 1);

        let loc = Location::new("path", "1 + 2\n3 + 4", 7);
        assert_eq!(loc.path, "path");
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 2);

        let loc = Location::new("path", "1 + 2\n3 + 4", 8);
        assert_eq!(loc.path, "path");
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 3);

        let loc = Location::new("path", "1 + 2\n3 + 4", 9);
        assert_eq!(loc.path, "path");
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 4);

        let loc = Location::new("path", "1 + 2\n3 + 4", 10);
        assert_eq!(loc.path, "path");
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 5);

        let loc = Location::new("path", "1 + 2\n3 + 4", 11);
        assert_eq!(loc.path, "path");
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 6);

        let loc = Location::new("path", "1 + 2\n3 + 4", 12);
        assert_eq!(loc.path, "path");
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 6);
    }

    /// Test the `fmt` method.
    #[test]
    fn test_fmt() {
        let loc = Location::new("path", "1 + 2", 3);
        assert_eq!(loc.to_string(), "path:1:4");

        let loc = Location::new("path", "1 + 2\n3 + 4", 6);
        assert_eq!(loc.to_string(), "path:2:1");

        let loc = Location::new("path/to/file", "1 + 2\n3 + 4", 7);
        assert_eq!(loc.to_string(), "path/to/file:2:2");
    }
}
