//! Utility functions for Display/Debug trait comparison in no_std environment.
//!
//! This crate provides functions to compare output of Display and Debug traits against &str
//! in no_std env. No `alloc` nor `std` is used.
//!
//! # Quick Start
//!
//! ```
//! #![no_std]
//! use fmt_compare_nostd::eq_display;
//! use core::fmt::{Display, Formatter, Result};
//!
//! struct D {}
//!
//! impl Display for D {
//!    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//!        write!(f, "Display D")
//!    }
//! }
//!
//! # fn main() {
//! assert!(eq_display(&D {}, "Display D"));
//! # }
//! ```

use core::fmt::{Debug, Display, Result, Write};

pub fn eq_display(d: &impl Display, s: &str) -> bool {
    let mut cmp = Comparator::new(s);
    write!(&mut cmp, "{}", d).is_ok() && cmp.is_valid()
}

pub fn eq_debug(d: &impl Debug, s: &str) -> bool {
    let mut cmp = Comparator::new(s);
    write!(&mut cmp, "{:?}", d).is_ok() && cmp.is_valid()
}

struct Comparator<'a> {
    to_compare: &'a str,
}

impl<'a> Comparator<'a> {
    fn new(s: &'a str) -> Self {
        Self { to_compare: s }
    }

    fn is_valid(&self) -> bool {
        self.to_compare.is_empty()
    }
}

impl<'a> Write for Comparator<'a> {
    fn write_str(&mut self, s: &str) -> Result {
        if s.eq(self.to_compare) {
            self.to_compare = "";
            return Ok(());
        }

        if self.to_compare.starts_with(s) && self.to_compare.len() >= s.len() {
            self.to_compare = &self.to_compare[s.len()..];
        } else {
            return Err(core::fmt::Error);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use core::fmt::{Display, Formatter, Result};

    #[derive(Debug)]
    struct D {}

    impl Display for D {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Display D")
        }
    }

    mod display {
        use super::D;
        use crate::eq_display;

        #[test]
        fn it_works() {
            assert!(eq_display(&D {}, "Display D"));
        }

        #[test]
        fn too_short() {
            assert!(!eq_display(&D {}, "Display"));
        }

        #[test]
        fn too_long() {
            assert!(!eq_display(&D {}, "Display DD"));
        }

        #[test]
        fn different() {
            assert!(!eq_display(&D {}, "Totally different str"));
        }
    }

    mod debug {
        use super::D;
        use crate::eq_debug;

        #[test]
        fn it_works() {
            assert!(eq_debug(&D {}, "D"));
        }

        #[test]
        fn too_short() {
            assert!(!eq_debug(&D {}, ""));
        }

        #[test]
        fn too_long() {
            assert!(!eq_debug(&D {}, "DD"));
        }

        #[test]
        fn different() {
            assert!(!eq_debug(&D {}, "A"));
        }
    }
}
