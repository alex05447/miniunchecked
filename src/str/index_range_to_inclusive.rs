use super::*;

impl SliceIndexExt<str> for std::ops::RangeToInclusive<usize> {
    type Output = str;

    #[inline]
    unsafe fn get_unchecked_dbg<'a>(
        self,
        s: &'a str,
        msg: Option<&'static str>,
    ) -> &'a Self::Output {
        let range = range_inclusive_into_range(0..=self.end);
        match s.get(self) {
            Some(val) => val,
            None => unreachable_dbg_range(s, range, msg),
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg<'a>(
        self,
        s: &'a mut str,
        msg: Option<&'static str>,
    ) -> &'a mut Self::Output {
        let range = range_inclusive_into_range(0..=self.end);
        let _s: *const str = &*s;
        match s.get_mut(self) {
            Some(val) => val,
            None => unreachable_dbg_range(&*_s, range, msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_unchecked_dbg_success() {
        let string = "föo";

        let test_range = |range: std::ops::RangeToInclusive<usize>, expected: &str| {
            let substring = unsafe { string.get_unchecked_dbg(range.clone()) };
            assert_eq!(substring, expected);
            assert_eq!(substring, &string[range]);
        };

        test_range(..=0, "f");
        test_range(..=2, "fö");
        test_range(..=3, "föo");
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "attempted to index str up to maximum usize"]
    fn get_unchecked_dbg_usize_overflow() {
        let string = "föo";
        let _ = unsafe { string.get_unchecked_dbg(..=usize::MAX) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "attempted to index str up to maximum usize"]
    fn get_unchecked_dbg_usize_overflow_matches_std() {
        let string = "föo";
        let _ = &string[..=usize::MAX];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_dbg_failure_oob() {
        let string = "föo";
        assert!(string.get(..=4).is_none());
        let _ = unsafe { string.get_unchecked_dbg(..=4) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_dbg_failure_oob_matches_std() {
        let string = "föo";
        let _ = &string[..=4];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_dbg_failure_character_boundary() {
        let string = "föo";
        let _ = unsafe { string.get_unchecked_dbg(..=1) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_dbg_failure_character_boundary_matches_std() {
        let string = "föo";
        let _ = &string[..=1];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`: invalid range"]
    fn get_unchecked_dbg_msg_failure_oob() {
        let string = "föo";
        assert!(string.get(..=4).is_none());
        let _ = unsafe { string.get_unchecked_dbg_msg(..=4, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`: invalid range"]
    fn get_unchecked_dbg_msg_failure_character_boundary() {
        let string = "föo";
        let _ = unsafe { string.get_unchecked_dbg_msg(..=1, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_mut_dbg_failure_oob() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        assert!(string.get_mut(..=4).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg(..=4) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_mut_dbg_failure_oob_start_matches_std() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        let _ = &mut string[..=4];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_mut_dbg_failure_character_boundary() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        let _ = unsafe { string.get_unchecked_mut_dbg(..=1) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_mut_dbg_failure_character_boundary_matches_std() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        let _ = &mut string[..=1];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure_oob() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        assert!(string.get_mut(..=4).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg_msg(..=4, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure_character_boundary() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        let _ = unsafe { string.get_unchecked_mut_dbg_msg(..=1, "invalid range") };
    }
}
