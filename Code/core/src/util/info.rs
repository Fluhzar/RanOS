//! # Info

/// Trait defining the ability to get info about a type as [`String`][0]s.
///
/// [0]: std::string::String
pub trait Info {
    /// Creates a new `Info`-implementing object, boxed for ease of working with
    /// multiple different implementing types.
    fn new() -> Box<dyn Info>
    where
        Self: Sized;

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
    ///
    /// It is important to note that the margin size is not taken into account
    /// when splitting the `Info::detail()` string into lines. Instead that
    /// string is split by the `max_line` length, and then shifted over by
    /// `" "` chars inserted `margin` times at the beginning of each line, so
    /// the theoretical maximum line length that may be present is
    /// `margin + max_line`.
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

/// Returns a string containing info about the given slice of `Info` objects in
/// a pretty-formatted `String`.
///
/// # Parameters
///
/// - `info_objects`
///
/// # Example
///
/// ```
/// # use base::util::Info;
/// # use base::animation::*;
/// # use base::util::info::*;
/// # fn get_infos() -> Vec<Box<dyn Info>> { vec![RainbowInfo::new()] }
/// let info_objects = get_infos();
/// println!("Options:\n{}", format_info(&info_objects, 80));
/// ```
pub fn format_info(info_objects: &[Box<dyn Info>], max_line: usize) -> String {
    let name_max_len = info_objects.iter().fold(0, |a, b| {
        if b.name().len() > a {
            b.name().len()
        } else {
            a
        }
    }) + 4;

    let mut out = String::new();
    for i in info_objects.iter() {
        out = format!("{}\n{}", out, i.info(name_max_len, max_line));
    }

    out
}

#[cfg(test)]
mod test {
    use super::Info;

    #[derive(Default)]
    struct InfoTest();

    impl Info for InfoTest {
        fn new() -> Box<dyn Info>
        where
            Self: Sized,
        {
            Box::new(InfoTest::default())
        }

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
