mod index_range;
mod index_range_from;
mod index_range_inclusive;
mod index_range_to;
mod index_range_to_inclusive;
mod index_usize;

use crate::{unreachable_dbg_fmt, unreachable_dbg_msg};

/// An extension trait for [`SliceIndex`](std::slice::SliceIndex) which provides an alternative to [`get_unchecked`](std::slice::SliceIndex#tymethod.get_unchecked)
/// which panics in debug configuration in case the index is invalid.
pub trait SliceIndexExt<T>
where
    T: ?Sized,
{
    type Output: ?Sized;

    unsafe fn get_unchecked_dbg<'a>(
        self,
        slice: &'a T,
        msg: Option<&'static str>,
    ) -> &'a Self::Output;
    unsafe fn get_unchecked_mut_dbg<'a>(
        self,
        slice: &'a mut T,
        msg: Option<&'static str>,
    ) -> &'a mut Self::Output;
}

/// An extension trait for [`slice`](https://doc.rust-lang.org/std/primitive.slice.html)
/// which provides an alternative to [`get_unchecked`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked)
/// which panics in debug configuration in case the index is invalid.
///
/// Implemented for [`usize`], [`std::ops::Range<usize>`], [`std::ops::RangeInclusive<usize>`],
/// [`std::ops::RangeFrom<usize>`], [`std::ops::RangeTo<usize>`], [`std::ops::RangeToInclusive<usize>`].
pub trait SliceExt<T> {
    unsafe fn get_unchecked_dbg<I>(&self, index: I) -> &<I as SliceIndexExt<[T]>>::Output
    where
        I: SliceIndexExt<[T]>;

    unsafe fn get_unchecked_mut_dbg<I>(
        &mut self,
        index: I,
    ) -> &mut <I as SliceIndexExt<[T]>>::Output
    where
        I: SliceIndexExt<[T]>;

    unsafe fn get_unchecked_dbg_msg<I>(
        &self,
        index: I,
        msg: &'static str,
    ) -> &<I as SliceIndexExt<[T]>>::Output
    where
        I: SliceIndexExt<[T]>;

    unsafe fn get_unchecked_mut_dbg_msg<I>(
        &mut self,
        index: I,
        msg: &'static str,
    ) -> &mut <I as SliceIndexExt<[T]>>::Output
    where
        I: SliceIndexExt<[T]>;
}

impl<T> SliceExt<T> for [T] {
    #[inline]
    unsafe fn get_unchecked_dbg<I>(&self, index: I) -> &<I as SliceIndexExt<[T]>>::Output
    where
        I: SliceIndexExt<[T]>,
    {
        // See `[T]::get_unchecked()`
        &*index.get_unchecked_dbg(self, None)
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg<I>(
        &mut self,
        index: I,
    ) -> &mut <I as SliceIndexExt<[T]>>::Output
    where
        I: SliceIndexExt<[T]>,
    {
        // See `[T]::get_unchecked_mut()`
        &mut *index.get_unchecked_mut_dbg(self, None)
    }

    #[inline]
    unsafe fn get_unchecked_dbg_msg<I>(
        &self,
        index: I,
        msg: &'static str,
    ) -> &<I as SliceIndexExt<[T]>>::Output
    where
        I: SliceIndexExt<[T]>,
    {
        // See `[T]::get_unchecked()`
        &*index.get_unchecked_dbg(self, Some(msg))
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg_msg<I>(
        &mut self,
        index: I,
        msg: &'static str,
    ) -> &mut <I as SliceIndexExt<[T]>>::Output
    where
        I: SliceIndexExt<[T]>,
    {
        // See `[T]::get_unchecked_mut()`
        &mut *index.get_unchecked_mut_dbg(self, Some(msg))
    }
}

#[inline]
pub(super) unsafe fn unreachable_dbg_range(
    range: std::ops::Range<usize>,
    len: usize,
    msg: Option<&'static str>,
) -> ! {
    if range.end > len {
        unreachable_dbg_fmt(format_args!(
            "range end index {} out of range for slice of length {len}{}{}",
            range.end,
            if msg.is_some() { ": " } else { "" },
            if let Some(msg) = msg { msg } else { "" }
        ))
    } else {
        debug_assert!(range.start > len);

        unreachable_dbg_fmt(format_args!(
            "range start index {} out of range for slice of length {len}{}{}",
            range.start,
            if msg.is_some() { ": " } else { "" },
            if let Some(msg) = msg { msg } else { "" }
        ))
    }
}

/// Based on `slice::index::slice_end_index_overflow_fail`.
fn slice_end_index_overflow_fail() -> ! {
    unsafe { unreachable_dbg_msg("attempted to index slice up to maximum usize") };
}

/// Based on `std::ops::RangeInclusive<usize>::into_slice_range`, but does not handle exhausted ranges.
/// TODO: revisit exhausted handling.
pub(super) fn range_inclusive_into_range(
    range: std::ops::RangeInclusive<usize>,
) -> std::ops::Range<usize> {
    if *range.end() == usize::MAX {
        slice_end_index_overflow_fail();
    }
    *range.start()..*range.end() + 1
}
