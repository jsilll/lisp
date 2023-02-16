/// Human-readable location in a source file.
pub struct FileLocation {
    /// The line number.
    pub line: usize,
    /// The column number.
    pub column: usize,
}

impl FileLocation {
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
        let column = match source[..idx].rfind(|c| {
            c == '\n' || c == '\r'
        }) {
            None => 1,
            Some(i) => idx - i,
        };
        Self { line, column }
    }
}