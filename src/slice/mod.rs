mod index_range;
mod index_range_from;
mod index_range_inclusize;
mod index_range_to;
mod index_range_to_inclusive;
mod index_usize;

use crate::*;

/// An extension trait for [`SliceIndex`](std::slice::SliceIndex) which provides an alternative to [`get_unchecked`](std::slice::SliceIndex#tymethod.get_unchecked)
/// which panics in debug configuration.
///
/// Implemented for [`usize`], [`std::ops::Range<usize>`], [`std::ops::RangeInclusive<usize>`],
/// [`std::ops::RangeFrom<usize>`], [`std::ops::RangeTo<usize>`], [`std::ops::RangeToInclusive<usize>`].
pub unsafe trait SliceIndexExt<T>
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
/// which panics in debug configuration.
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
pub(crate) fn debug_unreachable_range(
    range: std::ops::Range<usize>,
    len: usize,
    msg: Option<&'static str>,
) -> ! {
    debug_unreachable(format_args!(
        "range [{}..{}] out of range for slice of length {len}{}{}",
        range.start,
        range.end,
        if msg.is_some() { ": " } else { "" },
        if let Some(msg) = msg { msg } else { "" }
    ))
}
