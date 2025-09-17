use std::str::Split;

use itertools::Itertools;

use crate::tern;

pub trait StringTools {
    fn split_lines_once(&self) -> Option<(&str, &str)>;

    fn paragraphs(&self) -> Split<'_, &str>;

    fn split_paragraphs_once(&self) -> Option<(&str, &str)>;
}

impl StringTools for &str {
    /// Splits a string once by lines. Returns None if there are not exactly two
    /// lines.
    fn split_lines_once(&self) -> Option<(&str, &str)> {
        self.lines().collect_tuple()
    }

    /// Splits a string on paragraphs (two newlines in a row). Handles both
    /// `\r\n\r\n` and `\n\n`.
    fn paragraphs(&self) -> Split<'_, &str> {
        tern!(
            self.contains("\r\n\r\n"),
            self.split("\r\n\r\n"),
            self.split("\n\n")
        )
    }

    /// Splits a string once by paragraphs (two newlines in a row). Returns None
    /// if there are not exactly two lines.
    fn split_paragraphs_once(&self) -> Option<(&str, &str)> {
        self.paragraphs().collect_tuple()
    }
}
