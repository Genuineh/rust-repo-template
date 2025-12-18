//! Library for the project template.

/// Adds two numbers (example function with docs and a testable unit)
///
/// # Examples
///
/// ```
/// let s = rust_repo_template::add(2, 3);
/// assert_eq!(s, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
