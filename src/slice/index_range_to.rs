use crate::*;

impl<T> SliceIndexExt<[T]> for std::ops::RangeTo<usize> {
    type Output = [T];

    #[inline]
    unsafe fn get_unchecked_dbg<'a>(
        self,
        slice: &'a [T],
        msg: Option<&'static str>,
    ) -> &'a Self::Output {
        let _range = 0..self.end;
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
        let _range = 0..self.end;
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
        assert_eq!(unsafe { slice.get_unchecked_dbg(..0) }, &[]);
        assert_eq!(unsafe { slice.get_unchecked_dbg(..1) }, &[2]);
        assert_eq!(unsafe { slice.get_unchecked_dbg(..2) }, &[2, 3]);
        assert_eq!(unsafe { slice.get_unchecked_dbg(..3) }, &[2, 3, 4]);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_dbg_failure() {
        let slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_dbg(..4) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_dbg_failure_matches_std() {
        let slice = [2, 3, 4];
        let _ = &slice[..4];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3: invalid range"]
    fn get_unchecked_dbg_msg_failure() {
        let slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_dbg_msg(..4, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_mut_dbg_failure() {
        let mut slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_mut_dbg(..4) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3"]
    fn get_unchecked_mut_dbg_failure_matches_std() {
        let mut vec = [2, 3, 4].into_iter().collect::<Vec<_>>();
        let slice = vec.as_mut_slice();
        let _ = &mut slice[..4];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "range end index 4 out of range for slice of length 3: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure() {
        let mut slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_mut_dbg_msg(..4, "invalid range") };
    }
}
