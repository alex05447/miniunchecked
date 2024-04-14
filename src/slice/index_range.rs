use crate::*;

impl<T> SliceIndexExt<[T]> for std::ops::Range<usize> {
    type Output = [T];

    #[inline]
    unsafe fn get_unchecked_dbg<'a>(
        self,
        slice: &'a [T],
        msg: Option<&'static str>,
    ) -> &'a Self::Output {
        let range = self.clone();
        slice
            .get(self)
            .unwrap_or_else(|| unreachable_dbg_range(range, slice.len(), msg))
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg<'a>(
        self,
        slice: &'a mut [T],
        msg: Option<&'static str>,
    ) -> &'a mut Self::Output {
        let range = self.clone();
        let len = slice.len();
        slice
            .get_mut(self)
            .unwrap_or_else(|| unreachable_dbg_range(range, len, msg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_unchecked_dbg_success() {
        let slice = [2, 3, 4];

        let do_test = |idx: std::ops::Range<usize>, res: &[i32]| {
            assert_eq!(unsafe { slice.get_unchecked_dbg(idx.clone()) }, res);
            assert_eq!(unsafe { slice.get_unchecked(idx.clone()) }, res);
            assert_eq!(slice.get(idx.clone()), Some(res));
            assert_eq!(&slice[idx.clone()], res);
        };

        do_test(0..1, &[2]);
        do_test(0..2, &[2, 3]);
        do_test(1..2, &[3]);
        do_test(1..3, &[3, 4]);
        do_test(2..3, &[4]);
        do_test(0..3, &[2, 3, 4]);

        // NOTE: this succeeds somehow, because `3 == slice.len()`, even though index `3` is out of bounds.
        do_test(3..3, &[]);
        // But this fails.
        assert!(slice.get(3..4).is_none());
        assert!(slice.get(4..4).is_none());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 5 out of range for slice of length 3"]
    fn get_unchecked_dbg_failure_start() {
        let slice = [2, 3, 4];
        assert!(slice.get(3..5).is_none());
        let _ = unsafe { slice.get_unchecked_dbg(3..5) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 5 out of range for slice of length 3"]
    fn get_unchecked_dbg_failure_start_matches_std() {
        let slice = [2, 3, 4];
        let _ = &slice[3..5];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_dbg_failure_end() {
        let slice = [2, 3, 4];
        assert!(slice.get(1..4).is_none());
        let _ = unsafe { slice.get_unchecked_dbg(1..4) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_dbg_failure_end_matches_std() {
        let slice = [2, 3, 4];
        let _ = &slice[1..4];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 5 out of range for slice of length 3: invalid range"]
    fn get_unchecked_dbg_msg_failure_start() {
        let slice = [2, 3, 4];
        assert!(slice.get(3..5).is_none());
        let _ = unsafe { slice.get_unchecked_dbg_msg(3..5, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3: invalid range"]
    fn get_unchecked_dbg_msg_failure_end() {
        let slice = [2, 3, 4];
        assert!(slice.get(1..4).is_none());
        let _ = unsafe { slice.get_unchecked_dbg_msg(1..4, "invalid range") };
    }

    #[test]
    fn get_unchecked_mut_dbg_success() {
        let mut slice = [2, 3, 4];

        let mut do_test = |idx: std::ops::Range<usize>, res: &mut [i32]| {
            assert_eq!(unsafe { slice.get_unchecked_mut_dbg(idx.clone()) }, res);
            assert_eq!(unsafe { slice.get_unchecked_mut(idx.clone()) }, res);
            assert_eq!(slice.get_mut(idx.clone()), Some(res));
        };

        do_test(0..1, &mut [2]);
        do_test(0..2, &mut [2, 3]);
        do_test(1..2, &mut [3]);
        do_test(1..3, &mut [3, 4]);
        do_test(2..3, &mut [4]);
        do_test(0..3, &mut [2, 3, 4]);

        // NOTE: this succeeds somehow, because `3 == slice.len()`, even though index `3` is out of bounds.
        do_test(3..3, &mut []);
        // But this fails.
        assert!(slice.get_mut(3..4).is_none());
        assert!(slice.get_mut(4..4).is_none());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 5 out of range for slice of length 3"]
    fn get_unchecked_mut_dbg_failure_start() {
        let mut slice = [2, 3, 4];
        assert!(slice.get_mut(3..5).is_none());
        let _ = unsafe { slice.get_unchecked_mut_dbg(3..5) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 5 out of range for slice of length 3"]
    fn get_unchecked_mut_dbg_failure_start_matches_std() {
        let mut slice = [2, 3, 4];
        let _ = &mut slice[3..5];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_mut_dbg_failure_end() {
        let mut slice = [2, 3, 4];
        assert!(slice.get_mut(1..4).is_none());
        let _ = unsafe { slice.get_unchecked_mut_dbg(1..4) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_mut_dbg_failure_end_matches_std() {
        let mut slice = [2, 3, 4];
        let _ = &mut slice[1..4];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 5 out of range for slice of length 3: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure_start() {
        let mut slice = [2, 3, 4];
        assert!(slice.get_mut(3..5).is_none());
        let _ = unsafe { slice.get_unchecked_mut_dbg_msg(3..5, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure_end() {
        let mut slice = [2, 3, 4];
        assert!(slice.get_mut(1..4).is_none());
        let _ = unsafe { slice.get_unchecked_mut_dbg_msg(1..4, "invalid range") };
    }
}
