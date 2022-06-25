use crate::*;

/// An extension trait for [`Option`](std::option::Option) which provides an alternative to [`unwrap_unchecked`](std::option::Option#method.unwrap_unchecked)
/// which panics in debug configuration.
pub trait OptionExt<T> {
    unsafe fn unwrap_unchecked_dbg(self) -> T;
    unsafe fn unwrap_unchecked_dbg_msg(self, msg: &'static str) -> T;
    unsafe fn unwrap_unchecked_dbg_fmt(self, fmt: std::fmt::Arguments<'_>) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    unsafe fn unwrap_unchecked_dbg(self) -> T {
        match self {
            Some(val) => val,
            None => debug_unreachable_msg(None),
        }
    }
    #[inline]
    unsafe fn unwrap_unchecked_dbg_msg(self, msg: &'static str) -> T {
        match self {
            Some(val) => val,
            None => debug_unreachable_msg(Some(msg)),
        }
    }
    #[inline]
    unsafe fn unwrap_unchecked_dbg_fmt(self, fmt: std::fmt::Arguments<'_>) -> T {
        match self {
            Some(val) => val,
            None => debug_unreachable_fmt(fmt),
        }
    }
}

const ERR_STR: &'static str = "called `Option::unwrap()` on a `None` value";

#[inline]
fn debug_unreachable_msg(msg: Option<&'static str>) -> ! {
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
    #[should_panic = "called `Option::unwrap()` on a `None` value: missing value"]
    fn unwrap_unchecked_dbg_msg_failure() {
        let x: Option<i32> = None;
        let _ = unsafe { x.unwrap_unchecked_dbg_msg("missing value") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "called `Option::unwrap()` on a `None` value: missing value (expected 7)"]
    fn unwrap_unchecked_dbg_fmt_failure() {
        let x: Option<i32> = None;
        let _ =
            unsafe { x.unwrap_unchecked_dbg_fmt(format_args!("missing value (expected {})", 7)) };
    }
}
