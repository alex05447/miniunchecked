use super::*;

impl SliceIndexExt<str> for std::ops::RangeFrom<usize> {
    type Output = str;

    #[inline]
    unsafe fn get_unchecked_dbg<'a>(
        self,
        s: &'a str,
        msg: Option<&'static str>,
    ) -> &'a Self::Output {
        let range = self.start..s.len();
        s.get(self)
            .unwrap_or_else(|| unsafe { unreachable_dbg_range(s, range, msg) })
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg<'a>(
        self,
        s: &'a mut str,
        msg: Option<&'static str>,
    ) -> &'a mut Self::Output {
        let range = self.start..s.len();
        let _s: *const str = &*s;
        s.get_mut(self)
            .unwrap_or_else(|| unsafe { unreachable_dbg_range(&*_s, range, msg) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_unchecked_dbg_success() {
        let string = "föo";

        let do_test = |idx: std::ops::RangeFrom<usize>, res: &str| {
            assert_eq!(unsafe { string.get_unchecked_dbg(idx.clone()) }, res);
            assert_eq!(unsafe { string.get_unchecked(idx.clone()) }, res);
            assert_eq!(string.get(idx.clone()), Some(res));
            assert_eq!(&string[idx.clone()], res);
        };

        do_test(0.., "föo");
        do_test(1.., "öo");
        do_test(3.., "o");
        do_test(4.., "");
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_dbg_failure_oob() {
        let string = "föo";
        assert!(string.get(5..).is_none());
        let _ = unsafe { string.get_unchecked_dbg(5..) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_dbg_failure_oob_matches_std() {
        let string = "föo";
        let _ = &string[5..];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_dbg_failure_character_boundary() {
        let string = "föo";
        assert!(string.get(2..).is_none());
        let _ = unsafe { string.get_unchecked_dbg(2..) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_dbg_failure_character_boundary_matches_std() {
        let string = "föo";
        let _ = &string[2..];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`: invalid range"]
    fn get_unchecked_dbg_msg_failure_oob() {
        let string = "föo";
        assert!(string.get(5..).is_none());
        let _ = unsafe { string.get_unchecked_dbg_msg(5.., "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`: invalid range"]
    fn get_unchecked_dbg_msg_failure_character_boundary() {
        let string = "föo";
        assert!(string.get(2..).is_none());
        let _ = unsafe { string.get_unchecked_dbg_msg(2.., "invalid range") };
    }

    #[test]
    fn get_unchecked_mut_dbg_success() {
        let mut string = "föo".to_string();
        let string = string.as_mut_str();

        let mut do_test = |idx: std::ops::RangeFrom<usize>, res: &mut str| {
            assert_eq!(unsafe { string.get_unchecked_mut_dbg(idx.clone()) }, res);
            assert_eq!(unsafe { string.get_unchecked_mut(idx.clone()) }, res);
            assert_eq!(string.get_mut(idx.clone()), Some(&mut *res));
            assert_eq!(&mut string[idx.clone()], res);
        };

        do_test(0.., "föo".to_string().as_mut_str());
        do_test(1.., "öo".to_string().as_mut_str());
        do_test(3.., "o".to_string().as_mut_str());
        do_test(4.., "".to_string().as_mut_str());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_mut_dbg_failure_oob() {
        let mut string = "föo".to_string();
        let string = string.as_mut_str();
        assert!(string.get_mut(5..).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg(5..) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_mut_dbg_failure_oob_start_matches_std() {
        let mut string = "föo".to_string();
        let string = string.as_mut_str();
        let _ = &mut string[5..];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_mut_dbg_failure_character_boundary() {
        let mut string = "föo".to_string();
        let string = string.as_mut_str();
        assert!(string.get_mut(2..).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg(2..) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_mut_dbg_failure_character_boundary_matches_std() {
        let mut string = "föo".to_string();
        let string = string.as_mut_str();
        let _ = &mut string[2..];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure_oob() {
        let mut string = "föo".to_string();
        let string = string.as_mut_str();
        assert!(string.get_mut(5..).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg_msg(5.., "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure_character_boundary() {
        let mut string = "föo".to_string();
        let string = string.as_mut_str();
        assert!(string.get_mut(2..).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg_msg(2.., "invalid range") };
    }
}
