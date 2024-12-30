use crate::error::ContextError;
use crate::{Context, Error, StdError};
use core::convert::Infallible;
use core::fmt::{self, Debug, Display, Write};

mod ext {
    use super::*;

    pub trait StdError {
        fn ext_context<C>(self, context: C) -> Error
        where
            C: Display + Send + Sync + 'static;
    }

    impl<E> StdError for E
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        #[track_caller]
        fn ext_context<C>(self, context: C) -> Error
        where
            C: Display + Send + Sync + 'static,
        {
            Error::from_display(context)
        }
    }

    impl StdError for Error {
        #[track_caller]
        fn ext_context<C>(self, context: C) -> Error
        where
            C: Display + Send + Sync + 'static,
        {
            self.context(context)
        }
    }
}

impl<T, E> Context<T, E> for Result<T, E>
where
    E: ext::StdError + Send + Sync + 'static,
{
    #[track_caller]
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        // Not using map_err to save 2 useless frames off the captured backtrace
        // in ext_context.
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.ext_context(context)),
        }
    }

    #[track_caller]
    fn with_context<C, F>(self, context: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.ext_context(context())),
        }
    }
}

/// ```
/// # type T = ();
/// #
/// use lerror::{Context, Result};
///
/// fn maybe_get() -> Option<T> {
///     # const IGNORE: &str = stringify! {
///     ...
///     # };
///     # unimplemented!()
/// }
///
/// fn demo() -> Result<()> {
///     let t = maybe_get().context("there is no T")?;
///     # const IGNORE: &str = stringify! {
///     ...
///     # };
///     # unimplemented!()
/// }
/// ```
impl<T> Context<T, Infallible> for Option<T> {
    #[track_caller]
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        // Not using ok_or_else to save 2 useless frames off the captured
        // backtrace.
        match self {
            Some(ok) => Ok(ok),
            None => Err(Error::from_display(context)),
        }
    }

    #[track_caller]
    fn with_context<C, F>(self, context: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Some(ok) => Ok(ok),
            None => Err(Error::from_display(context())),
        }
    }
}

impl<C> Debug for ContextError<C>
where
    C: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("context", &Quoted(&self.context))
            .field("file", &self.file)
            .field("line", &self.line)
            .field("column", &self.column)
            .finish()
    }
}

impl<C> Display for ContextError<C>
where
    C: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.context, f)
    }
}

impl<C> StdError for ContextError<C>
where
    C: Display,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

struct Quoted<C>(C);

impl<C> Debug for Quoted<C>
where
    C: Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_char('"')?;
        Quoted(&mut *formatter).write_fmt(format_args!("{}", self.0))?;
        formatter.write_char('"')?;
        Ok(())
    }
}

impl Write for Quoted<&mut fmt::Formatter<'_>> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Display::fmt(&s.escape_debug(), self.0)
    }
}

pub(crate) mod private {
    use super::*;

    pub trait Sealed {}

    impl<T, E> Sealed for Result<T, E> where E: ext::StdError {}
    impl<T> Sealed for Option<T> {}
}
