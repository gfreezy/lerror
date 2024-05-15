use std::panic::Location;
use std::{collections::LinkedList, fmt::Display};

use crate::Error;
use crate::StdError;

// repr C to ensure that ContextError<C, E> has the same layout as
// ContextError<ManuallyDrop<C>, E> and ContextError<C, ManuallyDrop<E>>.
#[repr(C)]
pub(crate) struct ContextError<C> {
    pub context: C,
    pub line: u32,
    pub column: u32,
    pub file: &'static str,
}

impl ContextError<String> {
    #[track_caller]
    pub fn new(c: impl Display) -> Self {
        let caller = Location::caller();
        ContextError {
            context: c.to_string(),
            line: caller.line(),
            column: caller.column(),
            file: caller.file(),
        }
    }
}

impl Error {
    /// Create a new error object from any error type.
    ///
    /// The error type must be threadsafe and `'static`, so that the `Error`
    /// will be as well.
    ///
    /// If the error type does not provide a backtrace, a backtrace will be
    /// created here to ensure that a backtrace exists.
    #[cold]
    #[must_use]
    #[track_caller]
    pub fn new<E>(error: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Error::from_display(error)
    }

    #[cold]
    #[must_use]
    #[track_caller]
    pub fn context<C>(mut self, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        self.inner.push_front(ContextError::new(context));
        self
    }

    #[cold]
    #[track_caller]
    pub(crate) fn from_display<M>(message: M) -> Self
    where
        M: Display + Send + Sync + 'static,
    {
        let mut l = LinkedList::new();
        l.push_front(ContextError::new(message));
        Error { inner: l }
    }

    #[cold]
    #[track_caller]
    pub fn msg<M>(message: M) -> Self
    where
        M: Display + Send + Sync + 'static,
    {
        Error::from_display(message)
    }
}
