mod context;
mod ensure;
mod error;
mod fmt;
mod macros;
use std::{collections::LinkedList, error::Error as StdError, fmt::Display};

extern crate alloc;

use error::ContextError;

#[repr(transparent)]
pub struct Error {
    inner: LinkedList<ContextError<String>>,
}

#[doc(no_inline)]
pub use lerror as format_err;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub trait Context<T, E>: crate::context::private::Sealed {
    /// Wrap the error value with additional context.
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

#[allow(non_snake_case)]
pub fn Ok<T>(t: T) -> Result<T> {
    Result::Ok(t)
}

// Not public API. Referenced by macro-generated code.
#[doc(hidden)]
pub mod __private {
    use crate::Error;
    use alloc::fmt;
    use core::fmt::Arguments;

    #[doc(hidden)]
    pub use crate::ensure::{BothDebug, NotBothDebug};
    #[doc(hidden)]
    pub use alloc::format;
    #[doc(hidden)]
    pub use core::result::Result::Err;
    #[doc(hidden)]
    pub use core::{concat, format_args, stringify};

    #[doc(hidden)]
    #[inline]
    #[cold]
    #[track_caller]
    pub fn format_err(args: Arguments) -> Error {
        let fmt_arguments_as_str = args.as_str();

        if let Some(message) = fmt_arguments_as_str {
            // anyhow!("literal"), can downcast to &'static str
            Error::from_display(message)
        } else {
            // anyhow!("interpolate {var}"), can downcast to String
            Error::from_display(fmt::format(args))
        }
    }

    #[doc(hidden)]
    #[inline]
    #[cold]
    #[must_use]
    pub fn must_use(error: Error) -> Error {
        error
    }
}
