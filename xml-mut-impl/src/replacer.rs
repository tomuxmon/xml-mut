use std::{cmp::Ordering, ops::Range};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replacer {
    // TODO: String -> &'a str
    pub bounds: Range<usize>,
    pub replacement: String,
}

impl Replacer {
    pub fn len_diff(&self) -> i32 {
        i32::try_from(self.replacement.len()).unwrap_or(0)
            - (i32::try_from(self.bounds.end).unwrap_or(0)
                - i32::try_from(self.bounds.start).unwrap_or(0))
    }

    pub fn bounds_cmp(&self, other: &Self) -> Ordering {
        if self.bounds == other.bounds {
            Ordering::Equal
        } else if self.bounds.start == other.bounds.start {
            self.bounds.end.cmp(&other.bounds.end)
        } else {
            self.bounds.start.cmp(&other.bounds.start)
        }
    }

    /// Checks if the bounds of this replacer do not interfere with other replacer bounds.
    /// Touching other bounds is still ok.
    ///
    /// # Examples
    ///
    /// ```
    /// use xml_mut_impl::prelude::Replacer;
    ///
    /// let x = Replacer { bounds : 0..10, replacement : "".to_string() };
    /// let y = Replacer { bounds : 10..20, replacement : "".to_string() };
    ///
    /// assert!(!x.overlaps(&y));
    /// ```

    pub fn overlaps(&self, other: &Self) -> bool {
        self.bounds.start < other.bounds.end && self.bounds.end > other.bounds.start
    }
}
