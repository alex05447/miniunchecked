/// An extension trait for [`Option`] which provides alternatives to [`unwrap_unchecked()`](Option::unwrap_unchecked)
/// which panic in debug configuration with an optional custom message.
pub trait OptionExt<T> {
    /// Alternative to [`unwrap_unchecked()`](Option::unwrap_unchecked) which panics in debug configuration if the [`Option`] is [`None`].
    ///
    /// # Safety
    ///
    /// See [`unwrap_unchecked()`](Option::unwrap_unchecked) documentation.
    unsafe fn unwrap_unchecked_dbg(self) -> T;

    /// Alternative to [`unwrap_unchecked()`](Option::unwrap_unchecked) which panics in debug configuration if the [`Option`] is [`None`], with a custom error message.
    ///
    /// # Safety
    ///
    /// See [`unwrap_unchecked()`](Option::unwrap_unchecked) documentation.
    unsafe fn unwrap_unchecked_dbg_msg(self, msg: &'static str) -> T;

    /// Alternative to [`unwrap_unchecked()`](Option::unwrap_unchecked) which panics in debug configuration if the [`Option`] is [`None`], with a custom formatted error message.
    ///
    /// # Safety
    ///
    /// See [`unwrap_unchecked()`](Option::unwrap_unchecked) documentation.
    unsafe fn unwrap_unchecked_dbg_fmt(self, fmt: std::fmt::Arguments<'_>) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    unsafe fn unwrap_unchecked_dbg(self) -> T {
        self.unwrap_or_else(|| unreachable_dbg_msg(None))
    }

    #[inline]
    unsafe fn unwrap_unchecked_dbg_msg(self, msg: &'static str) -> T {
        self.unwrap_or_else(|| unreachable_dbg_msg(Some(msg)))
    }

    #[inline]
    unsafe fn unwrap_unchecked_dbg_fmt(self, fmt: std::fmt::Arguments<'_>) -> T {
        self.unwrap_or_else(|| unreachable_dbg_fmt(fmt))
    }
}

const ERR_STR: &'static str = "called `Option::unwrap()` on a `None` value";

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
    use {super::*, crate::unwrap_unchecked_dbg};

    fn returns_an_option(val: i32) -> Option<u8> {
        u8::try_from(val).ok()
    }

    #[test]
    fn unwrap_unchecked_dbg_success() {
        let x = Some(7);
        assert_eq!(unsafe { x.unwrap_unchecked_dbg() }, 7)
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "called `Option::unwrap()` on a `None` value"]
    fn unwrap_unchecked_dbg_failure() {
        let x: Option<i32> = None;
        let _ = unsafe { x.unwrap_unchecked_dbg() };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "called `Option::unwrap()` on a `None` value"]
    fn unwrap_unchecked_dbg_macro_failure() {
        unsafe { unwrap_unchecked_dbg!(returns_an_option(255), "this should succeed") };
        unsafe { unwrap_unchecked_dbg!(returns_an_option(-7)) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "called `Option::unwrap()` on a `None` value: missing value"]
    fn unwrap_unchecked_dbg_msg_failure() {
        let x: Option<i32> = None;
        let _ = unsafe { x.unwrap_unchecked_dbg_msg("missing value") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "called `Option::unwrap()` on a `None` value: missing value"]
    fn unwrap_unchecked_dbg_msg_macro_failure() {
        unsafe { unwrap_unchecked_dbg!(returns_an_option(255), "this should succeed") };
        unsafe { unwrap_unchecked_dbg!(returns_an_option(-7), "missing value") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "called `Option::unwrap()` on a `None` value: missing value (expected 7)"]
    fn unwrap_unchecked_dbg_fmt_failure() {
        let x: Option<i32> = None;
        let _ =
            unsafe { x.unwrap_unchecked_dbg_fmt(format_args!("missing value (expected {})", 7)) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "called `Option::unwrap()` on a `None` value: missing value (expected -7)"]
    fn unwrap_unchecked_dbg_fmt_macro_failure() {
        unsafe { unwrap_unchecked_dbg!(returns_an_option(255), "this should succeed") };
        unsafe { unwrap_unchecked_dbg!(returns_an_option(-7), "missing value (expected {})", -7) };
    }
}
