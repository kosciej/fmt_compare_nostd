## Description

Utility functions for Display/Debug trait comparison in no_std environment.

This crate provides functions to compare output of Display and Debug traits against &str
in no_std env. No `alloc` nor `std` is used.

Broader description of the issue that this crate solves:
[StackOverflow](https://stackoverflow.com/questions/72727634/how-to-test-result-of-corefmtdisplay-trait-implementation-in-no-std-env)

## Quick Start

```rust
#![no_std]
use fmt_compare_nostd::eq_display;
use core::fmt::{Display, Formatter, Result};
 
struct D {}

impl Display for D {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
       write!(f, "Display D")
   }
}

fn main() {
    assert!(eq_display(&D {}, "Display D"));
}
```