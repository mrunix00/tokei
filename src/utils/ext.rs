//! Various extensions to Rust std types.

pub(crate) trait AsciiExt {
    fn is_whitespace(self) -> bool;
    fn is_not_line_ending_whitespace(self) -> bool;
    fn is_line_ending_whitespace(self) -> bool;
}

impl AsciiExt for u8 {
    fn is_whitespace(self) -> bool {
        self == b' ' || (b'\x09'..=b'\x0d').contains(&self)
    }

    fn is_not_line_ending_whitespace(self) -> bool {
        self.is_whitespace() && !self.is_line_ending_whitespace()
    }

    fn is_line_ending_whitespace(self) -> bool {
        self == b'\n'
    }
}

pub(crate) trait SliceExt {
    fn trim_first_and_last_line_of_whitespace(&self) -> &Self;
    fn trim_start(&self) -> &Self;
    fn trim(&self) -> &Self;
    fn contains_slice(&self, needle: &Self) -> bool;
}

impl SliceExt for [u8] {
    fn trim_first_and_last_line_of_whitespace(&self) -> &Self {
        let start = self
            .iter()
            .position(|c| c.is_line_ending_whitespace() || !c.is_whitespace())
            .map_or(0, |i| (i + 1).min(self.len()));

        let end = self
            .iter()
            .rposition(|c| c.is_line_ending_whitespace() || !c.is_whitespace())
            .map_or_else(
                || self.len(),
                |i| {
                    if self[i.saturating_sub(1)] == b'\r' {
                        i - 1
                    } else {
                        i
                    }
                },
            );

        if self[start..].is_empty() {
            return &[]
        }

        &self[start..=end]
    }

    fn trim_start(&self) -> &Self {
        let length = self.len();

        if length == 0 {
            return &self;
        }

        let start = match self.iter().position(|c| !c.is_whitespace()) {
            Some(start) => start,
            None => return &[],
        };

        &self[start..]
    }

    fn trim(&self) -> &Self {
        let length = self.len();

        if length == 0 {
            return &self;
        }

        let start = match self.iter().position(|c| !c.is_whitespace()) {
            Some(start) => start,
            None => return &[],
        };

        let end = match self.iter().rposition(|c| !c.is_whitespace()) {
            Some(end) => end.max(start),
            _ => length,
        };

        &self[start..=end]
    }

    fn contains_slice(&self, needle: &Self) -> bool {
        let self_length = self.len();
        let needle_length = needle.len();

        if needle_length == 0 || needle_length > self_length {
            return false;
        } else if needle_length == self_length {
            return self == needle;
        }

        for window in self.windows(needle_length) {
            if needle == window {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_whitespace() {
        assert!(b' '.is_whitespace());
        assert!(b'\r'.is_whitespace());
        assert!(b'\n'.is_whitespace());
    }

    #[test]
    fn trim() {
        assert!([b' ', b' ', b' '].trim().is_empty());
        assert!([b' ', b'\r', b'\n'].trim().is_empty());
        assert!([b'\n'].trim().is_empty());
        assert!([].trim().is_empty());

        assert_eq!([b'a', b'b'], [b'a', b'b'].trim());
        assert_eq!([b'h', b'i'], [b' ', b'h', b'i'].trim());
        assert_eq!([b'h', b'i'], [b'h', b'i', b' '].trim());
        assert_eq!([b'h', b'i'], [b' ', b'h', b'i', b' '].trim());
    }

    #[test]
    fn contains() {
        assert!([1, 2, 3, 4, 5].contains_slice(&[1, 2, 3, 4, 5]));
        assert!([1, 2, 3, 4, 5].contains_slice(&[1, 2, 3]));
        assert!([1, 2, 3, 4, 5].contains_slice(&[3, 4, 5]));
        assert!([1, 2, 3, 4, 5].contains_slice(&[2, 3, 4]));
        assert!(![1, 2, 3, 4, 5].contains_slice(&[]));
    }
}
