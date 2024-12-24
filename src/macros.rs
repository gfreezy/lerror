/// Return early with an error.
///
/// This macro is equivalent to `return Err(`[`lerror!($args...)`][lerror!]`)`.
///
/// The surrounding function's or closure's return value is required to be
/// `Result<_,`[`lerror::Error`][crate::Error]`>`.
///
/// [lerror!]: crate::lerror
///
/// # Example
///
/// ```
/// # use lerror::{bail, Result};
/// #
/// # fn has_permission(user: usize, resource: usize) -> bool {
/// #     true
/// # }
/// #
/// # fn main() -> Result<()> {
/// #     let user = 0;
/// #     let resource = 0;
/// #
/// if !has_permission(user, resource) {
///     bail!("permission denied for accessing {}", resource);
/// }
/// #     Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return $crate::__private::Err($crate::__lerror!($msg))
    };
    ($err:expr $(,)?) => {
        return $crate::__private::Err($crate::__lerror!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return $crate::__private::Err($crate::__lerror!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! du {
    ($err:expr $(,)?) => {
        match $err {
            Ok(v) => v,
            Err(e) => return Err(e).context(""),
        }
    };
    ($err:expr, $fmt:expr $(,)?) => {
        match $err {
            Ok(v) => v,
            Err(e) => return Err(e).context($fmt),
        }
    };
    ($err:expr, $fmt:expr, $($arg:tt)*) => {
        match $err {
            Ok(v) => v,
            Err(e) => return Err(e).context($crate::__private::format!($fmt, $($arg)*)),
        }
    };
}

macro_rules! __ensure {
    ($ensure:item) => {
        /// Return early with an error if a condition is not satisfied.
        ///
        /// This macro is equivalent to `if !$cond { return
        /// Err(`[`lerror!($args...)`][lerror!]`); }`.
        ///
        /// The surrounding function's or closure's return value is required to be
        /// `Result<_,`[`lerror::Error`][crate::Error]`>`.
        ///
        /// Analogously to `assert!`, `ensure!` takes a condition and exits the function
        /// if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`
        /// rather than panicking.
        ///
        /// [lerror!]: crate::lerror
        ///
        /// # Example
        ///
        /// ```
        /// # use lerror::{ensure, Result};
        /// #
        /// # fn main() -> Result<()> {
        /// #     let user = 0;
        /// #
        /// ensure!(user == 0, "only user 0 is allowed");
        /// #     Ok(())
        /// # }
        /// ```
        $ensure
    };
}

#[cfg(doc)]
__ensure![
    #[macro_export]
    macro_rules! ensure {
        ($cond:expr $(,)?) => {
            if !$cond {
                return $crate::__private::Err($crate::Error::msg(
                    $crate::__private::concat!("Condition failed: `", $crate::__private::stringify!($cond), "`")
                ));
            }
        };
        ($cond:expr, $msg:literal $(,)?) => {
            if !$cond {
                return $crate::__private::Err($crate::__lerror!($msg));
            }
        };
        ($cond:expr, $err:expr $(,)?) => {
            if !$cond {
                return $crate::__private::Err($crate::__lerror!($err));
            }
        };
        ($cond:expr, $fmt:expr, $($arg:tt)*) => {
            if !$cond {
                return $crate::__private::Err($crate::__lerror!($fmt, $($arg)*));
            }
        };
    }
];

#[cfg(not(doc))]
__ensure![
    #[macro_export]
    macro_rules! ensure {
        ($($tt:tt)*) => {
            $crate::__parse_ensure!(
                /* state */ 0
                /* stack */ ()
                /* bail */ ($($tt)*)
                /* fuel */ (~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~)
                /* parse */ {()}
                /* dup */ ($($tt)*)
                /* rest */ $($tt)*
            )
        };
    }
];

/// Construct an ad-hoc error from a string or existing non-`lerror` error
/// value.
///
/// This evaluates to an [`Error`][crate::Error]. It can take either just a
/// string, or a format string with arguments. It also can take any custom type
/// which implements `Debug` and `Display`.
///
/// If called with a single argument whose type implements `std::error::Error`
/// (in addition to `Debug` and `Display`, which are always required), then that
/// Error impl's `source` is preserved as the `source` of the resulting
/// `lerror::Error`.
///
/// # Example
///
/// ```
/// # type V = ();
/// #
/// use lerror::{lerror, Result};
///
/// fn lookup(key: &str) -> Result<V> {
///     if key.len() != 16 {
///         return Err(lerror!("key length must be 16 characters, got {:?}", key));
///     }
///
///     // ...
///     # Ok(())
/// }
/// ```
#[macro_export]
macro_rules! lerror {
    ($msg:literal $(,)?) => {
        $crate::__private::must_use({
            let error = $crate::__private::format_err($crate::__private::format_args!($msg));
            error
        })
    };
    ($err:expr $(,)?) => {
        $crate::__private::must_use({
            use $crate::__private::kind::*;
            let error = match $err {
                error => (&error).lerror_kind().new(error),
            };
            error
        })
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::msg($crate::__private::format!($fmt, $($arg)*))
    };
}

// Not public API. This is used in the implementation of some of the other
// macros, in which the must_use call is not needed because the value is known
// to be used.
#[doc(hidden)]
#[macro_export]
macro_rules! __lerror {
    ($msg:literal $(,)?) => ({
        let error = $crate::__private::format_err($crate::__private::format_args!($msg));
        error
    });
    ($err:expr $(,)?) => ({
        use $crate::__private::kind::*;
        let error = match $err {
            error => (&error).lerror_kind().new(error),
        };
        error
    });
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::msg($crate::__private::format!($fmt, $($arg)*))
    };
}
