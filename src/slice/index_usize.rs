use crate::*;

unsafe impl<T> SliceIndexExt<[T]> for usize {
    type Output = T;

    #[inline]
    unsafe fn get_unchecked_dbg<'a>(
        self,
        slice: &'a [T],
        msg: Option<&'static str>,
    ) -> &'a Self::Output {
        let len = slice.len();
        match slice.get(self) {
            Some(val) => val,
            None => unreachable_dbg_index(self, len, msg),
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg<'a>(
        self,
        slice: &'a mut [T],
        msg: Option<&'static str>,
    ) -> &'a mut Self::Output {
        let len = slice.len();
        match slice.get_mut(self) {
            Some(val) => val,
            None => unreachable_dbg_index(self, len, msg),
        }
    }
}

fn unreachable_dbg_index(index: usize, len: usize, msg: Option<&'static str>) -> ! {
    unreachable_dbg_fmt(format_args!(
        "index out of bounds: the len is {len} but the index is {index}{}{}",
        if msg.is_some() { ": " } else { "" },
        if let Some(msg) = msg { msg } else { "" }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_unchecked_dbg_success() {
        let slice = [2, 3, 4];
        assert_eq!(*unsafe { slice.get_unchecked_dbg(1) }, 3);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 3"]
    fn get_unchecked_dbg_failure() {
        let slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_dbg(3) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 3: invalid index"]
    fn get_unchecked_dbg_msg_failure() {
        let slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_dbg_msg(3, "invalid index") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 3"]
    fn get_unchecked_mut_dbg_failure() {
        let mut slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_mut_dbg(3) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 3: invalid index"]
    fn slice_get_unchecked_mut_dbg_msg_failure() {
        let mut slice = [2, 3, 4];
        let _ = unsafe { slice.get_unchecked_mut_dbg_msg(3, "invalid index") };
    }
}
