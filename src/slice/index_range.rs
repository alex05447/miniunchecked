use crate::*;

impl<T> SliceIndexExt<[T]> for std::ops::Range<usize> {
    type Output = [T];

    #[inline]
    unsafe fn get_unchecked_dbg<'a>(
        self,
        slice: &'a [T],
        msg: Option<&'static str>,
    ) -> &'a Self::Output {
        let _range = self.clone();
        let len = slice.len();
        match slice.get(self) {
            Some(val) => val,
            None => unreachable_dbg_range(_range, len, msg),
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg<'a>(
        self,
        slice: &'a mut [T],
        msg: Option<&'static str>,
    ) -> &'a mut Self::Output {
        let _range = self.clone();
        let len = slice.len();
        match slice.get_mut(self) {
            Some(val) => val,
            None => unreachable_dbg_range(_range, len, msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_unchecked_dbg_success() {
        let slice = [2, 3, 4];
        assert_eq!(unsafe { slice.get_unchecked_dbg(0..1) }, &[2]);
        assert_eq!(unsafe { slice.get_unchecked_dbg(0..2) }, &[2, 3]);
        assert_eq!(unsafe { slice.get_unchecked_dbg(1..2) }, &[3]);
        assert_eq!(unsafe { slice.get_unchecked_dbg(1..3) }, &[3, 4]);
        assert_eq!(unsafe { slice.get_unchecked_dbg(2..3) }, &[4]);
        assert_eq!(unsafe { slice.get_unchecked_dbg(0..3) }, &[2, 3, 4]);
        // NOTE: this succeeds somehow, because `3 == slice.len()`, even though index `3` is out of bounds.
        assert_eq!(unsafe { slice.get_unchecked_dbg(3..3) }, &[]);
        assert_eq!(&slice[3..3], &[]);
        // But this fails.
        assert!(slice.get(4..4).is_none());
        assert!(slice.get(3..4).is_none());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 5 out of range for slice of length 3"]
    fn get_unchecked_dbg_failure_start() {
        let slice = [2, 3, 4];
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
        let _ = unsafe { slice.get_unchecked_dbg_msg(3..5, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3: invalid range"]
    fn get_unchecked_dbg_msg_failure_end() {
        let slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_dbg_msg(1..4, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 5 out of range for slice of length 3"]
    fn get_unchecked_mut_dbg_failure_start() {
        let mut slice = [2, 3, 4];
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
        let _ = unsafe { slice.get_unchecked_mut_dbg_msg(3..5, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure_end() {
        let mut slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_mut_dbg_msg(1..4, "invalid range") };
    }
}
