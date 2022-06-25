use crate::*;

/// An extension trait for [`Result`](std::result::Result) which provides an alternative to [`unwrap_unchecked`](std::result::Result#method.unwrap_unchecked)
/// which panics in debug configuration.
pub trait ResultExt<T> {
    unsafe fn unwrap_unchecked_dbg(self) -> T;
    unsafe fn unwrap_unchecked_dbg_msg(self, msg: &'static str) -> T;
    unsafe fn unwrap_unchecked_dbg_fmt(self, fmt: std::fmt::Arguments<'_>) -> T;
}

impl<T, E> ResultExt<T> for Result<T, E> {
    #[inline]
    unsafe fn unwrap_unchecked_dbg(self) -> T {
        match self {
            Ok(val) => val,
            Err(_) => debug_unreachable_msg(None),
        }
    }
    #[inline]
    unsafe fn unwrap_unchecked_dbg_msg(self, msg: &'static str) -> T {
        match self {
            Ok(val) => val,
            Err(_) => debug_unreachable_msg(Some(msg)),
        }
    }
    #[inline]
    unsafe fn unwrap_unchecked_dbg_fmt(self, fmt: std::fmt::Arguments<'_>) -> T {
        match self {
            Ok(val) => val,
            Err(_) => debug_unreachable_fmt(fmt),
        }
    }
}

const ERR_STR: &'static str = "called `Result::unwrap()` on an `Err` value";

#[inline]
fn debug_unreachable_msg(msg: Option<&'static str>) -> ! {
    // TODO: deduplicate the format string somehow
    if let Some(msg) = msg {
        debug_unreachable_fmt(format_args!("{}", msg))
    } else {
        debug_unreachable(format_args!("{}", ERR_STR))
    }
}

#[inline]
fn debug_unreachable_fmt(fmt: std::fmt::Arguments<'_>) -> ! {
    debug_unreachable(format_args!("{}: {}", ERR_STR, fmt))
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
