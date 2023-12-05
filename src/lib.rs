use std::{borrow::Cow, iter::Peekable};

pub fn lpad<'a>(base: &'a str, pad_string: &str) -> Cow<'a, str> {
    if pad_string.is_empty() {
        Cow::Borrowed(base)
    } else {
        let intersperse = Intersperse {
            base: base.trim_end_matches('\n').split('\n').peekable(),
            pad: pad_string,
            stage: Stage::Pad,
            end_with_lb: base.ends_with('\n'),
        };
        Cow::Owned(intersperse.collect())
    }
}

enum Stage {
    Pad,
    Next,
    LineBreak,
}

struct Intersperse<'a> {
    base: Peekable<std::str::Split<'a, char>>,
    pad: &'a str,
    stage: Stage,
    end_with_lb: bool,
}

impl<'a> Iterator for Intersperse<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stage {
            Stage::Pad => {
                if self.base.peek().is_some() {
                    self.stage = Stage::Next;
                    Some(self.pad)
                } else if self.end_with_lb {
                    self.end_with_lb = false;
                    Some("\n")
                } else {
                    None
                }
            }
            Stage::Next => {
                self.stage = Stage::LineBreak;
                self.base.next()
            }
            Stage::LineBreak => {
                if self.base.peek().is_some() {
                    self.stage = Stage::Pad;
                    Some("\n")
                } else if self.end_with_lb {
                    self.end_with_lb = false;
                    Some("\n")
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(lpad("hello\nworld", " "), " hello\n world");
        assert_eq!(lpad("hello\nworld\n", " "), " hello\n world\n");
        assert_eq!(lpad("hello\nworld\n", " "), " hello\n world\n");
        assert_eq!(lpad("hello\r\nworld", " "), " hello\r\n world");
        assert_eq!(lpad("hello\r\nworld\r\n", " "), " hello\r\n world\r\n");
        assert_eq!(
            lpad("hello\r\nworld\r\n", " aaa "),
            " aaa hello\r\n aaa world\r\n"
        );
    }
}
