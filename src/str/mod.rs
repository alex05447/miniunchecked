mod index_range;
mod index_range_from;
mod index_range_inclusive;
mod index_range_to;
mod index_range_to_inclusive;

use crate::*;

/// An extension trait for [`str`](https://doc.rust-lang.org/std/primitive.str.html)
/// which provides an alternative to [`get_unchecked`](https://doc.rust-lang.org/std/primitive.str.html#method.get_unchecked)
/// which panics in debug configuration in case the index is invalid.
///
/// Implemented for [`std::ops::Range<usize>`], [`std::ops::RangeInclusive<usize>`],
/// [`std::ops::RangeFrom<usize>`], [`std::ops::RangeTo<usize>`], [`std::ops::RangeToInclusive<usize>`].
pub trait StrExt {
    unsafe fn get_unchecked_dbg<I>(&self, index: I) -> &<I as SliceIndexExt<str>>::Output
    where
        I: SliceIndexExt<str>;

    unsafe fn get_unchecked_mut_dbg<I>(
        &mut self,
        index: I,
    ) -> &mut <I as SliceIndexExt<str>>::Output
    where
        I: SliceIndexExt<str>;

    unsafe fn get_unchecked_dbg_msg<I>(
        &self,
        index: I,
        msg: &'static str,
    ) -> &<I as SliceIndexExt<str>>::Output
    where
        I: SliceIndexExt<str>;

    unsafe fn get_unchecked_mut_dbg_msg<I>(
        &mut self,
        index: I,
        msg: &'static str,
    ) -> &mut <I as SliceIndexExt<str>>::Output
    where
        I: SliceIndexExt<str>;
}

impl StrExt for str {
    #[inline]
    unsafe fn get_unchecked_dbg<I>(&self, index: I) -> &<I as SliceIndexExt<str>>::Output
    where
        I: SliceIndexExt<str>,
    {
        // See `str::get_unchecked()`
        &*index.get_unchecked_dbg(self, None)
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg<I>(
        &mut self,
        index: I,
    ) -> &mut <I as SliceIndexExt<str>>::Output
    where
        I: SliceIndexExt<str>,
    {
        // See `str::get_unchecked_mut()`
        &mut *index.get_unchecked_mut_dbg(self, None)
    }

    #[inline]
    unsafe fn get_unchecked_dbg_msg<I>(
        &self,
        index: I,
        msg: &'static str,
    ) -> &<I as SliceIndexExt<str>>::Output
    where
        I: SliceIndexExt<str>,
    {
        // See `str::get_unchecked()`
        &*index.get_unchecked_dbg(self, Some(msg))
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg_msg<I>(
        &mut self,
        index: I,
        msg: &'static str,
    ) -> &mut <I as SliceIndexExt<str>>::Output
    where
        I: SliceIndexExt<str>,
    {
        // See `str::get_unchecked_mut()`
        &mut *index.get_unchecked_mut_dbg(self, Some(msg))
    }
}

/// Copied from `u8::is_utf8_char_boundary`. Private.
#[inline]
const fn is_utf8_char_boundary(b: u8) -> bool {
    // This is bit magic equivalent to: b < 128 || b >= 192
    (b as i8) >= -0x40
}

/// Copied from `str::floor_char_boundary`. Unstable, gated by "round_char_boundary".
/// TODO: get rid of when `str::floor_char_boundary` is stabilized.
fn floor_char_boundary(s: &str, index: usize) -> usize {
    if index >= s.len() {
        s.len()
    } else {
        let lower_bound = index.saturating_sub(3);
        let new_index = s.as_bytes()[lower_bound..=index]
            .iter()
            .rposition(|&b| is_utf8_char_boundary(b));

        // SAFETY: we know that the character boundary will be within four bytes
        unsafe { lower_bound + new_index.unwrap_unchecked() }
    }
}

pub(super) unsafe fn unreachable_dbg_range(
    s: &str,
    range: std::ops::Range<usize>,
    msg: Option<&'static str>,
) -> ! {
    // Logic copied from `str::slice_error_fail_rt` with minor changes. Private.
    //
    // NOTE: when `end` is out of bounds, this code reports an off-by-one out of bounds index.
    // E.g. if a string is "foo", 3 bytes long, and `range` is `0..4`
    // (i.e. it includes bytes `0` ("f"), `1` ("o"), `2` ("o"), and `3` (actual out-of-bounds index)),
    // this will report that "byte index 4 is out of bounds of `foo`", which is wrong, but this is the default std behaviour,
    // and I want this code to mimic it.
    //
    // NOTE: when `begin > end`, this seemingly reports an "inverted" error message (namely that `begin <= end`) for some reason.
    // As in this reports the assert condition, instead of the error which happened, which is counterintuitive and weird.
    // Keeping the default behaviour, as above.

    let begin = range.start;
    let end = range.end;

    const MAX_DISPLAY_LENGTH: usize = 256;
    let trunc_len = floor_char_boundary(s, MAX_DISPLAY_LENGTH);
    let s_trunc = &s[..trunc_len];
    let ellipsis = if trunc_len < s.len() { "[...]" } else { "" };

    // 1. out of bounds
    if begin > s.len() || end > s.len() {
        let oob_index = if begin > s.len() { begin } else { end };
        //panic!("byte index {oob_index} is out of bounds of `{s_trunc}`{ellipsis}");
        unreachable_dbg_fmt(format_args!(
            "byte index {oob_index} is out of bounds of `{s_trunc}`{ellipsis}{}{}",
            if msg.is_some() { ": " } else { "" },
            if let Some(msg) = msg { msg } else { "" }
        ))
    }

    // 2. begin <= end
    // assert!(
    //     begin <= end,
    //     "begin <= end ({} <= {}) when slicing `{}`{}",
    //     begin,
    //     end,
    //     s_trunc,
    //     ellipsis
    // );
    if begin > end {
        unreachable_dbg_fmt(format_args!(
            "begin <= end ({} <= {}) when slicing `{s_trunc}`{ellipsis}{}{}",
            begin,
            end,
            if msg.is_some() { ": " } else { "" },
            if let Some(msg) = msg { msg } else { "" }
        ))
    }

    // 3. character boundary
    let index = if !s.is_char_boundary(begin) {
        begin
    } else {
        end
    };
    // find the character
    let char_start = floor_char_boundary(s, index);
    // `char_start` must be less than len and a char boundary
    let ch = s[char_start..].chars().next().unwrap();
    let char_range = char_start..char_start + ch.len_utf8();
    // panic!(
    //     "byte index {} is not a char boundary; it is inside {:?} (bytes {:?}) of `{}`{}",
    //     index, ch, char_range, s_trunc, ellipsis
    // );
    unreachable_dbg_fmt(format_args!(
        "byte index {} is not a char boundary; it is inside {:?} (bytes {:?}) of `{s_trunc}`{ellipsis}{}{}",
        index,
        ch,
        char_range,
        if msg.is_some() { ": " } else { "" },
        if let Some(msg) = msg { msg } else { "" }
    ))
}

/// Based on `str::traits::str_index_overflow_fail`.
fn str_index_overflow_fail() -> ! {
    unsafe { unreachable_dbg_msg("attempted to index str up to maximum usize") };
}

/// Based on `std::ops::RangeInclusive<usize>::into_slice_range`, but does not handle exhausted ranges.
/// TODO: revisit exhausted handling.
pub(super) fn range_inclusive_into_range(
    range: std::ops::RangeInclusive<usize>,
) -> std::ops::Range<usize> {
    if *range.end() == usize::MAX {
        str_index_overflow_fail();
    }
    *range.start()..*range.end() + 1
}
