//! # miniunchecked
//!
//! A crate with some utility methods for debug unchecked operations on options, results and slices.
//!
//! A middle ground between someting like calling `Option::unwrap()` / `Result::unwrap()` / slice square brackets indexing operator
//! (which always panic if `None` / `Err` / out of bounds)
//! and `unsafe { Option::unwrap_unchecked() }` / `unsafe { Result::unwrap_unchecked() }` / `unsafe { [T]::get_unchecked(...) }`)
//! (which never panic and lead to UB when `None` / `Err` / out of bounds) - an operation which does unsafe access in release configuration,
//! but also panics on `None` / `Err` / out of bounds index in debug configuration.

mod option;
mod result;
mod slice;
mod str;

pub use {crate::str::StrExt, option::*, result::*, slice::*};

/// An alternative function to the [`std::unreachable`] macro which panics in debug configuration (like [`std::unreachable`] does),
/// but doesn't in release configuration (like [`std::hint::unreachable_unchecked`]).
///
/// Also see [`macro@unreachable_dbg`].
#[inline]
pub unsafe fn unreachable_dbg() -> ! {
    if cfg!(debug_assertions) {
        unreachable!()
    } else {
        std::hint::unreachable_unchecked()
    }
}

/// An alternative function to the [`std::unreachable`] macro which panics in debug configuration (like [`std::unreachable`] does),
/// but doesn't in release configuration (like [`std::hint::unreachable_unchecked`]).
///
/// Takes a string literal message argument.
///
/// Also see [`macro@unreachable_dbg`].
#[inline]
pub unsafe fn unreachable_dbg_msg(msg: &'static str) -> ! {
    if cfg!(debug_assertions) {
        unreachable!("{}", msg)
    } else {
        std::hint::unreachable_unchecked()
    }
}

/// An alternative function to the [`std::unreachable`] macro which panics in debug configuration (like [`std::unreachable`] does),
/// but doesn't in release configuration (like [`std::hint::unreachable_unchecked`]).
///
/// Accepts format arguments.
///
/// Also see [`macro@unreachable_dbg`].
#[inline]
pub unsafe fn unreachable_dbg_fmt(fmt: std::fmt::Arguments<'_>) -> ! {
    if cfg!(debug_assertions) {
        unreachable!("{}", fmt)
    } else {
        std::hint::unreachable_unchecked()
    }
}

/// An alternative to [`std::unreachable`] which panics in debug configuration (like [`std::unreachable`] does),
/// but doesn't in release configuration (like [`std::hint::unreachable_unchecked`]).
///
/// Accepts no arguments, string literals, or format strings with format arguments.
///
/// Implemented with [`unreachable_dbg`](unreachable_dbg()), [`unreachable_dbg_msg()`] and [`unreachable_dbg_fmt()`].
#[macro_export]
macro_rules! unreachable_dbg {
    () => {
        miniunchecked::unreachable_dbg()
    };
    ($msg:literal) => {
        miniunchecked::unreachable_dbg_msg($msg)
    };
    ($fmt:expr, $($args:tt)*) => {
        miniunchecked::unreachable_dbg_fmt(format_args!($fmt, $($args)*))
    };
}
