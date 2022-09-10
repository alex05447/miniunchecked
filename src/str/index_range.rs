use super::*;

impl SliceIndexExt<str> for std::ops::Range<usize> {
    type Output = str;

    #[inline]
    unsafe fn get_unchecked_dbg<'a>(
        self,
        s: &'a str,
        msg: Option<&'static str>,
    ) -> &'a Self::Output {
        let _range = self.clone();
        match s.get(self) {
            Some(val) => val,
            None => unreachable_dbg_range(s, _range, msg),
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut_dbg<'a>(
        self,
        s: &'a mut str,
        msg: Option<&'static str>,
    ) -> &'a mut Self::Output {
        let _range = self.clone();
        let _s: *const str = &*s;
        match s.get_mut(self) {
            Some(val) => val,
            None => unreachable_dbg_range(&*_s, _range, msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_unchecked_dbg_success() {
        let string = "föo";

        let test_range = |range: std::ops::Range<usize>, expected: &str| {
            let substring = unsafe { string.get_unchecked_dbg(range.clone()) };
            assert_eq!(substring, expected);
            assert_eq!(substring, &string[range]);
        };

        test_range(0..0, "");
        test_range(0..1, "f");
        test_range(0..3, "fö");
        test_range(0..4, "föo");
        test_range(1..1, "");
        test_range(1..3, "ö");
        test_range(1..4, "öo");
        test_range(3..3, "");
        test_range(3..4, "o");
        // NOTE: this succeeds somehow, because `4 == string.len()`, even though byte index `4` is out of bounds.
        test_range(4..4, "");
        // But this fails.
        assert!(string.get(5..5).is_none());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_dbg_failure_oob_start() {
        let string = "föo";
        assert!(string.get(5..6).is_none());
        let _ = unsafe { string.get_unchecked_dbg(5..6) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_dbg_failure_oob_start_matches_std() {
        let string = "föo";
        let _ = &string[5..6];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_dbg_failure_oob_end() {
        let string = "föo";
        assert!(string.get(0..5).is_none());
        let _ = unsafe { string.get_unchecked_dbg(0..5) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_dbg_failure_oob_end_matches_std() {
        let string = "föo";
        let _ = &string[0..5];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "begin <= end (1 <= 0) when slicing `föo`"]
    fn get_unchecked_dbg_failure_begin_greater_than_end() {
        let string = "föo";
        assert!(string.get(1..0).is_none());
        let _ = unsafe { string.get_unchecked_dbg(1..0) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "begin <= end (1 <= 0) when slicing `föo`"]
    fn get_unchecked_dbg_failure_begin_greater_than_end_matches_std() {
        let string = "föo";
        let _ = &string[1..0];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_dbg_failure_character_boundary() {
        let string = "föo";
        let _ = unsafe { string.get_unchecked_dbg(2..3) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_dbg_failure_character_boundary_matches_std() {
        let string = "föo";
        let _ = &string[2..3];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`: invalid range"]
    fn get_unchecked_dbg_msg_failure_oob_start() {
        let string = "föo";
        assert!(string.get(5..6).is_none());
        let _ = unsafe { string.get_unchecked_dbg_msg(5..6, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`: invalid range"]
    fn get_unchecked_dbg_msg_failure_oob_end() {
        let string = "föo";
        assert!(string.get(0..5).is_none());
        let _ = unsafe { string.get_unchecked_dbg_msg(0..5, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "begin <= end (1 <= 0) when slicing `föo`: invalid range"]
    fn get_unchecked_dbg_msg_failure_begin_greater_than_end() {
        let string = "föo";
        assert!(string.get(1..0).is_none());
        let _ = unsafe { string.get_unchecked_dbg_msg(1..0, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`: invalid range"]
    fn get_unchecked_dbg_msg_failure_character_boundary() {
        let string = "föo";
        let _ = unsafe { string.get_unchecked_dbg_msg(2..3, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_mut_dbg_failure_oob_start() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        assert!(string.get_mut(5..6).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg(5..6) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_mut_dbg_failure_oob_start_matches_std() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        let _ = &mut string[5..6];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_mut_dbg_failure_oob_end() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        assert!(string.get(0..5).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg(0..5) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`"]
    fn get_unchecked_mut_dbg_failure_oob_end_matches_std() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        let _ = &mut string[0..5];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "begin <= end (1 <= 0) when slicing `föo`"]
    fn get_unchecked_mut_dbg_failure_begin_greater_than_end() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        assert!(string.get_mut(1..0).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg(1..0) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "begin <= end (1 <= 0) when slicing `föo`"]
    fn get_unchecked_mut_dbg_failure_begin_greater_than_end_matches_std() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        let _ = &mut string[1..0];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_mut_dbg_failure_character_boundary() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        let _ = unsafe { string.get_unchecked_mut_dbg(2..3) };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`"]
    fn get_unchecked_mut_dbg_failure_character_boundary_matches_std() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        let _ = &mut string[2..3];
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure_oob_start() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        assert!(string.get_mut(5..6).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg_msg(5..6, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 5 is out of bounds of `föo`: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure_oob_end() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        assert!(string.get(0..5).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg_msg(0..5, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "begin <= end (1 <= 0) when slicing `föo`: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure_begin_greater_than_end() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        assert!(string.get_mut(1..0).is_none());
        let _ = unsafe { string.get_unchecked_mut_dbg_msg(1..0, "invalid range") };
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic = "byte index 2 is not a char boundary; it is inside 'ö' (bytes 1..3) of `föo`: invalid range"]
    fn get_unchecked_mut_dbg_msg_failure_character_boundary() {
        let mut string = String::from("föo");
        let string = string.as_mut_str();
        let _ = unsafe { string.get_unchecked_mut_dbg_msg(2..3, "invalid range") };
    }
}
