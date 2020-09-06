//! # Info

/// Trait defining the ability to get info about a type as [`String`][0]s.
///
/// [0]: std::string::String
pub trait Info {
    /// Returns the name of the implementing struct as a [`String`][0].
    ///
    /// [0]: std::string::String
    fn name(&self) -> String;

    /// Returns some details about `self` and returns it as a [`String`][0].
    ///
    /// [0]: std::string::String
    fn details(&self) -> String;

    /// Uses the other methods of this trait combined together into a pretty
    /// print-ready string with all of the info about the implementing type.
    ///
    /// # Parameters
    ///
    /// - `margin` - The size of the margin for the detailed info to be shifted
    /// over by.
    /// - `max_line` - The maximum line length for the detailed info.
    fn info(&self, margin: usize, max_line: usize) -> String {
        use super::max_line::MaxLine;
        let out = format!("{1:<0$}", margin, self.name());
        let lines = self.details().max_line(max_line);
        let out = format!("{}{}\n", out, lines[0]);

        let mut out = out;
        for s in lines.iter().skip(1) {
            out = format!("{0:}{2:<1$}{3:}\n", out, margin, " ", s);
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::Info;

    #[derive(Default)]
    struct InfoTest();

    impl InfoTest {
        pub fn new() -> Self {
            Default::default()
        }
    }

    impl Info for InfoTest {
        fn name(&self) -> String {
            "InfoTest".to_owned()
        }
        fn details(&self) -> String {
            "This is a really long line that will certainly need to be split into multiple lines when this is called through the test function to verify that the trait is functioning properly.".to_owned()
        }
    }

    #[test]
    fn test_info() {
        let out = InfoTest::new().info(16, 60);
        eprintln!("{}", out);
    }
}
