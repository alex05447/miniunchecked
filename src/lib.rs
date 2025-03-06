//! # miniunchecked
//!
//! A crate with some utility methods for debug unchecked operations on [`Option`], [`Result`]
//! and [`slice`](https://doc.rust-lang.org/std/primitive.slice.html) / [`str`](https://doc.rust-lang.org/std/primitive.str.html),
//! as well as a debug [`unreachable!`] alternative.
//!
//! A middle ground between someting like
//! - calling [`Option::unwrap()`] / [`Result::unwrap()`] / slice/string square brackets indexing operator, or using [`unreachable!`],
//! which always panic if [`None`] / [`Err`] / out of bounds / reached, and
//! - unsafe [`Option::unwrap_unchecked()`] [`Result::unwrap_unchecked()`] / [`[T]::get_unchecked()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked),
//! or using [`unreachable_unchecked()`](std::hint::unreachable_unchecked),
//! which never panic and lead to UB when [`None`] / [`Err`] / out of bounds,
//!
//! an operation which does unsafe access in release configuration, for optimal codegen if the invariants are maintained by other means,
//! but also panics on [`None`] / [`Err`] / out of bounds index in debug configuration / when running tests.

mod option;
mod result;
mod slice;
mod str;

pub use {crate::str::StrExt, option::*, result::*, slice::*};

/// An alternative function to the [`unreachable!`] macro which panics in debug configuration (like [`unreachable!`] does),
/// but doesn't in release configuration (like [`unreachable_unchecked()`](std::hint::unreachable_unchecked)).
///
/// Also see [`unreachable_dbg!`].
///
/// # Safety
///
/// See [`unreachable_unchecked()`](std::hint::unreachable_unchecked) documentation.
#[inline]
pub unsafe fn unreachable_dbg() -> ! {
    if cfg!(debug_assertions) {
        unreachable!()
    } else {
        unsafe { std::hint::unreachable_unchecked() }
    }
}

/// An alternative function to the [`unreachable!`] macro which panics in debug configuration (like [`unreachable!`] does),
/// but doesn't in release configuration (like [`unreachable_unchecked()`](std::hint::unreachable_unchecked)).
///
/// Accepts an error message as a string literal.
///
/// Also see [`unreachable_dbg!`].
///
/// # Safety
///
/// See [`unreachable_unchecked()`](std::hint::unreachable_unchecked) documentation.
#[inline]
pub unsafe fn unreachable_dbg_msg(msg: &'static str) -> ! {
    if cfg!(debug_assertions) {
        unreachable!("{}", msg)
    } else {
        unsafe { std::hint::unreachable_unchecked() }
    }
}

/// An alternative function to the [`unreachable!`] macro which panics in debug configuration (like [`unreachable!`] does),
/// but doesn't in release configuration (like [`unreachable_unchecked()`](std::hint::unreachable_unchecked)).
///
/// Accepts an error message as format arguments.
///
/// Also see [`unreachable_dbg!`].
///
/// # Safety
///
/// See [`unreachable_unchecked()`](std::hint::unreachable_unchecked) documentation.
#[inline]
pub unsafe fn unreachable_dbg_fmt(fmt: std::fmt::Arguments<'_>) -> ! {
    if cfg!(debug_assertions) {
        unreachable!("{}", fmt)
    } else {
        unsafe { std::hint::unreachable_unchecked() }
    }
}

/// An alternative to [`unreachable!`] macro which panics in debug configuration (like [`unreachable!`] does),
/// but doesn't in release configuration (like [`unreachable_unchecked()`](std::hint::unreachable_unchecked)).
///
/// Variants accept
/// - no arguments,
/// - an error message as a string literal, or
/// - an error message as format arguments.
///
/// Implemented with [`unreachable_dbg()`], [`unreachable_dbg_msg()`] and [`unreachable_dbg_fmt()`].
///
/// # Safety
///
/// See [`unreachable_unchecked()`](std::hint::unreachable_unchecked) documentation.
///
/// # Examples
///
/// ```
/// use miniunchecked::unreachable_dbg;
///
/// let x = Some(7);
///
/// match x {
///     Some(7) => println!("everything OK"),
///     _ => unsafe { unreachable_dbg!("something went wrong") },
/// }
/// ```
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

/// A utility macro which calls [`OptionExt::unwrap_unchecked_dbg()`](OptionExt::unwrap_unchecked_dbg) / [`ResultExt::unwrap_unchecked_dbg()`](ResultExt::unwrap_unchecked_dbg)
/// on a passed option or result, with or without a custom error message.
///
/// Variants accept
/// - no arguments,
/// - an error message as a string literal, or
/// - an error message as format arguments.
///
/// # Safety
///
/// See [`OptionExt::unwrap_unchecked_dbg()`](OptionExt::unwrap_unchecked_dbg) / [`ResultExt::unwrap_unchecked_dbg()`](ResultExt::unwrap_unchecked_dbg) documentation.
///
/// # Examples
///
/// ```
/// use miniunchecked::{ unwrap_unchecked_dbg, OptionExt };
///
/// let x = Some(7);
///
/// let val = unsafe { unwrap_unchecked_dbg!(x) };
/// assert_eq!(val, 7);
///
/// let val = unsafe { unwrap_unchecked_dbg!(x, "something went wrong") };
/// assert_eq!(val, 7);
///
/// let val = unsafe { unwrap_unchecked_dbg!(x, "something went wrong, expected {}", 7) };
/// assert_eq!(val, 7);
///
/// let val = unsafe { unwrap_unchecked_dbg!(Some(-7)) };
/// assert_eq!(val, -7);
///
/// let val = unsafe { unwrap_unchecked_dbg!(Some(-7), "something went wrong") };
/// assert_eq!(val, -7);
///
/// let val = unsafe { unwrap_unchecked_dbg!(Some(-7), "something went wrong, expected {}", -7) };
/// assert_eq!(val, -7);
/// ```
#[macro_export]
macro_rules! unwrap_unchecked_dbg {
    ($option_or_result:expr) => {
        ($option_or_result).unwrap_unchecked_dbg()
    };
    ($option_or_result:expr, $msg:literal) => {
        ($option_or_result).unwrap_unchecked_dbg_msg($msg)
    };
    ($option_or_result:expr, $fmt:expr, $($args:tt)*) => {
        ($option_or_result).unwrap_unchecked_dbg_fmt(format_args!($fmt, $($args)*))
    };
}
