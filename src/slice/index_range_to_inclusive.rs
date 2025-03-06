use crate::*;

impl<T> SliceIndexExt<[T]> for std::ops::RangeToInclusive<usize> {
    type Output = [T];

    #[inline]
    unsafe fn get_unchecked_dbg<'a>(
        self,
        slice: &'a [T],
        msg: Option<&'static str>,
    ) -> &'a Self::Output {
        let range = range_inclusive_into_range(0..=self.end);
        slice
            .get(self)
            .unwrap_or_else(|| unsafe { unreachable_dbg_range(range, slice.len(), msg) })
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg<'a>(
        self,
        slice: &'a mut [T],
        msg: Option<&'static str>,
    ) -> &'a mut Self::Output {
        let range = range_inclusive_into_range(0..=self.end);
        let len = slice.len();
        slice
            .get_mut(self)
            .unwrap_or_else(|| unsafe { unreachable_dbg_range(range, len, msg) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_unchecked_dbg_success() {
        let slice = [2, 3, 4];

        let do_test = |idx: std::ops::RangeToInclusive<usize>, res: &[i32]| {
            assert_eq!(unsafe { slice.get_unchecked_dbg(idx) }, res);
            assert_eq!(unsafe { slice.get_unchecked(idx) }, res);
            assert_eq!(slice.get(idx), Some(res));
            assert_eq!(&slice[idx], res);
        };

        do_test(..=0, &[2]);
        do_test(..=1, &[2, 3]);
        do_test(..=2, &[2, 3, 4]);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "attempted to index slice up to maximum usize"]
    fn get_unchecked_dbg_usize_overflow() {
        let slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_dbg(..=usize::MAX) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "attempted to index slice up to maximum usize"]
    fn get_unchecked_dbg_usize_overflow_matches_std() {
        let slice = [2, 3, 4];
        let _ = &slice[..=usize::MAX];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_dbg_failure() {
        let slice = [2, 3, 4];
        assert!(slice.get(..=3).is_none());
        let _ = unsafe { slice.get_unchecked_dbg(..=3) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_dbg_failure_matches_std() {
        let slice = [2, 3, 4];
        let _ = &slice[..=3];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3: invalid range"]
    fn get_unchecked_dbg_msg_failure() {
        let slice = [2, 3, 4];
        assert!(slice.get(..=3).is_none());
        let _ = unsafe { slice.get_unchecked_dbg_msg(..=3, "invalid range") };
    }

    #[test]
    fn get_unchecked_mut_dbg_success() {
        let mut slice = [2, 3, 4];

        let mut do_test = |idx: std::ops::RangeToInclusive<usize>, res: &mut [i32]| {
            assert_eq!(unsafe { slice.get_unchecked_mut_dbg(idx) }, res);
            assert_eq!(unsafe { slice.get_unchecked_mut(idx) }, res);
            assert_eq!(slice.get_mut(idx), Some(res));
        };

        do_test(..=0, &mut [2]);
        do_test(..=1, &mut [2, 3]);
        do_test(..=2, &mut [2, 3, 4]);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "attempted to index slice up to maximum usize"]
    fn get_unchecked_mut_dbg_usize_overflow() {
        let mut slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_mut_dbg(..=usize::MAX) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "attempted to index slice up to maximum usize"]
    fn get_unchecked_mut_dbg_usize_overflow_matches_std() {
        let mut slice = [2, 3, 4];
        let _ = &mut slice[..=usize::MAX];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_mut_dbg_failure() {
        let mut slice = [2, 3, 4];
        assert!(slice.get_mut(..=3).is_none());
        let _ = unsafe { slice.get_unchecked_mut_dbg(..=3) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_mut_dbg_failure_matches_std() {
        let mut slice = [2, 3, 4];
        let _ = &mut slice[..=3];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure() {
        let mut slice = [2, 3, 4];
        assert!(slice.get_mut(..=3).is_none());
        let _ = unsafe { slice.get_unchecked_mut_dbg_msg(..=3, "invalid range") };
    }
}
