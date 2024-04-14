/// An extension trait for [`Result`] which provides alternatives to [`unwrap_unchecked()`](Result::unwrap_unchecked)
/// which panic in debug configuration with an optional custom message.
pub trait ResultExt<T> {
    /// Alternative to [`unwrap_unchecked()`](Result::unwrap_unchecked) which panics in debug configuration if the [`Result`] is [`Err`].
    ///
    /// # Safety
    ///
    /// See [`unwrap_unchecked()`](Result::unwrap_unchecked) documentation.
    unsafe fn unwrap_unchecked_dbg(self) -> T;

    /// Alternative to [`unwrap_unchecked()`](Result::unwrap_unchecked) which panics in debug configuration if the [`Result`] is [`Err`], with a custom error message.
    ///
    /// # Safety
    ///
    /// See [`unwrap_unchecked()`](Result::unwrap_unchecked) documentation.
    unsafe fn unwrap_unchecked_dbg_msg(self, msg: &'static str) -> T;

    /// Alternative to [`unwrap_unchecked()`](Result::unwrap_unchecked) which panics in debug configuration if the [`Result`] is [`Err`], with a custom formatted error message.
    ///
    /// # Safety
    ///
    /// See [`unwrap_unchecked()`](Result::unwrap_unchecked) documentation.
    unsafe fn unwrap_unchecked_dbg_fmt(self, fmt: std::fmt::Arguments<'_>) -> T;
}

impl<T, E> ResultExt<T> for Result<T, E> {
    #[inline]
    unsafe fn unwrap_unchecked_dbg(self) -> T {
        self.unwrap_or_else(|_| unreachable_dbg_msg(None))
    }

    #[inline]
    unsafe fn unwrap_unchecked_dbg_msg(self, msg: &'static str) -> T {
        self.unwrap_or_else(|_| unreachable_dbg_msg(Some(msg)))
    }

    #[inline]
    unsafe fn unwrap_unchecked_dbg_fmt(self, fmt: std::fmt::Arguments<'_>) -> T {
        self.unwrap_or_else(|_| unreachable_dbg_fmt(fmt))
    }
}

const ERR_STR: &'static str = "called `Result::unwrap()` on an `Err` value";

#[inline]
fn unreachable_dbg_msg(msg: Option<&'static str>) -> ! {
    if let Some(msg) = msg {
        unreachable_dbg_fmt(format_args!("{}", msg))
    } else {
        unsafe { crate::unreachable_dbg_fmt(format_args!("{}", ERR_STR)) }
    }
}

#[inline]
fn unreachable_dbg_fmt(fmt: std::fmt::Arguments<'_>) -> ! {
    unsafe { crate::unreachable_dbg_fmt(format_args!("{}: {}", ERR_STR, fmt)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwrap_unchecked_dbg_success() {
        let x: Result<i32, ()> = Ok(7);
        assert_eq!(unsafe { x.unwrap_unchecked_dbg() }, 7)
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "called `Result::unwrap()` on an `Err` value"]
    fn unwrap_unchecked_dbg_failure() {
        let x: Result<i32, ()> = Err(());
        let _ = unsafe { x.unwrap_unchecked_dbg() };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "called `Result::unwrap()` on an `Err` value: missing value"]
    fn unwrap_unchecked_dbg_msg_failure() {
        let x: Result<i32, ()> = Err(());
        let _ = unsafe { x.unwrap_unchecked_dbg_msg("missing value") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "called `Result::unwrap()` on an `Err` value: missing value (expected 7)"]
    fn unwrap_unchecked_dbg_fmt_failure() {
        let x: Result<i32, ()> = Err(());
        let _ =
            unsafe { x.unwrap_unchecked_dbg_fmt(format_args!("missing value (expected {})", 7)) };
    }
}
