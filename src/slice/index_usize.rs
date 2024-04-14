use crate::*;

impl<T> SliceIndexExt<[T]> for usize {
    type Output = T;

    #[inline]
    unsafe fn get_unchecked_dbg<'a>(
        self,
        slice: &'a [T],
        msg: Option<&'static str>,
    ) -> &'a Self::Output {
        slice
            .get(self)
            .unwrap_or_else(|| unreachable_dbg_index(self, slice.len(), msg))
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg<'a>(
        self,
        slice: &'a mut [T],
        msg: Option<&'static str>,
    ) -> &'a mut Self::Output {
        let len = slice.len();
        slice
            .get_mut(self)
            .unwrap_or_else(|| unreachable_dbg_index(self, len, msg))
    }
}

fn unreachable_dbg_index(index: usize, len: usize, msg: Option<&'static str>) -> ! {
    unsafe {
        unreachable_dbg_fmt(format_args!(
            "index out of bounds: the len is {len} but the index is {index}{}{}",
            if msg.is_some() { ": " } else { "" },
            if let Some(msg) = msg { msg } else { "" }
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_unchecked_dbg_success() {
        let slice = [2, 3, 4];

        let do_test = |idx: usize, res: &i32| {
            assert_eq!(unsafe { slice.get_unchecked_dbg(idx) }, res);
            assert_eq!(unsafe { slice.get_unchecked(idx) }, res);
            assert_eq!(slice.get(idx), Some(res));
            assert_eq!(&slice[idx], res);
        };

        do_test(0, &2);
        do_test(1, &3);
        do_test(2, &4);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 3"]
    fn get_unchecked_dbg_failure() {
        let slice = [2, 3, 4];
        assert!(slice.get(3).is_none());
        let _ = unsafe { slice.get_unchecked_dbg(3) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 3"]
    fn get_unchecked_dbg_failure_matches_std() {
        let slice = [2, 3, 4];
        #[allow(unconditional_panic)]
        let _ = &slice[3];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 3: invalid index"]
    fn get_unchecked_dbg_msg_failure() {
        let slice = [2, 3, 4];
        assert!(slice.get(3).is_none());
        let _ = unsafe { slice.get_unchecked_dbg_msg(3, "invalid index") };
    }

    #[test]
    fn get_unchecked_mut_dbg_success() {
        let mut slice = [2, 3, 4];

        let mut do_test = |idx: usize, res: &mut i32| {
            assert_eq!(unsafe { slice.get_unchecked_mut_dbg(idx) }, res);
            assert_eq!(unsafe { slice.get_unchecked_mut(idx) }, res);
            assert_eq!(slice.get_mut(idx), Some(res));
        };

        do_test(0, &mut 2);
        do_test(1, &mut 3);
        do_test(2, &mut 4);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 3"]
    fn get_unchecked_mut_dbg_failure() {
        let mut slice = [2, 3, 4];
        assert!(slice.get_mut(3).is_none());
        let _ = unsafe { slice.get_unchecked_mut_dbg(3) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 3"]
    fn get_unchecked_mut_dbg_failure_matches_std() {
        let mut slice = [2, 3, 4];
        #[allow(unconditional_panic)]
        let _ = &mut slice[3];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 3: invalid index"]
    fn slice_get_unchecked_mut_dbg_msg_failure() {
        let mut slice = [2, 3, 4];
        assert!(slice.get_mut(3).is_none());
        let _ = unsafe { slice.get_unchecked_mut_dbg_msg(3, "invalid index") };
    }
}
