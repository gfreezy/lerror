use core::fmt::{self, Debug, Write};

use crate::Error;

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lerror::Error\n\n")?;

        for (i, context_error) in self.inner.iter().enumerate() {
            let mut indented = Indented {
                inner: f,
                number: Some(i),
                started: false,
            };
            if context_error.context.is_empty() {
                write!(
                    indented,
                    "{}:{}:{}",
                    context_error.file, context_error.line, context_error.column
                )?;
            } else {
                write!(
                    indented,
                    "{}:{}:{}\n{}",
                    context_error.file,
                    context_error.line,
                    context_error.column,
                    context_error.context
                )?;
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

struct Indented<'a, D> {
    inner: &'a mut D,
    number: Option<usize>,
    started: bool,
}

impl<T> Write for Indented<'_, T>
where
    T: Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (i, line) in s.split('\n').enumerate() {
            if !self.started {
                self.started = true;
                match self.number {
                    Some(number) => write!(self.inner, "{: >5}: ", number)?,
                    None => self.inner.write_str("    ")?,
                }
            } else if i > 0 {
                self.inner.write_char('\n')?;
                if self.number.is_some() {
                    self.inner.write_str("       ")?;
                } else {
                    self.inner.write_str("    ")?;
                }
            }

            self.inner.write_str(line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_digit() {
        let input = "verify\nthis";
        let expected = "    2: verify\n       this";
        let mut output = String::new();

        Indented {
            inner: &mut output,
            number: Some(2),
            started: false,
        }
        .write_str(input)
        .unwrap();

        assert_eq!(expected, output);
    }

    #[test]
    fn two_digits() {
        let input = "verify\nthis";
        let expected = "   12: verify\n       this";
        let mut output = String::new();

        Indented {
            inner: &mut output,
            number: Some(12),
            started: false,
        }
        .write_str(input)
        .unwrap();

        assert_eq!(expected, output);
    }

    #[test]
    fn no_digits() {
        let input = "verify\nthis";
        let expected = "    verify\n    this";
        let mut output = String::new();

        Indented {
            inner: &mut output,
            number: None,
            started: false,
        }
        .write_str(input)
        .unwrap();

        assert_eq!(expected, output);
    }
}
