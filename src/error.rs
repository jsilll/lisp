/// Human-readable location in a source file.
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    /// Create a new `Location` from a source string and an index into
    /// the string.
    /// 
    /// # Arguments
    /// - `source`: The source string.
    /// - `idx`: The index into the source string.
    /// 
    /// # Returns
    /// A new `Location` struct.
    pub fn new(source: &str, idx: usize) -> Self {
        let line = source[..idx].lines().count();
        let column = match source[..idx].lines().last() {
            None => 0,
            Some(l) => l.len() + 1,
        };

        Self { line, column }
    }
}