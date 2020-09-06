//! # Max Line

/// Trait that defines the ability to split a string-like type into an array
/// owned `String`s that have a maximum length up to a given maximum number of
/// bytes.
pub trait MaxLine: std::ops::Deref<Target = str> {
    /// Splits `self`, a string-like type, into separate lines with each line's
    /// length as close to but less than `max`, and converting any other
    /// whitespace character to a space.
    fn max_line(&self, max: usize) -> Vec<String> {
        let mut out = String::new();

        let mut curr = 0;
        self.to_owned().split_whitespace().for_each(|s| {
            curr += s.len() + " ".len();
            if curr > max {
                out.push('\n');
                curr = s.len();

                out.push_str(s);
                out.push(' ');
            } else {
                out.push_str(s);
                out.push(' ');
            }
        });

        out.lines().map(|s| s.trim().to_owned()).collect()
    }
}

impl MaxLine for &str {}
impl MaxLine for String {}
