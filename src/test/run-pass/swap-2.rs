// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem::swap;

pub fn main() {
    let mut a: Vec<int> = vec!(0, 1, 2, 3, 4, 5, 6);
    a.as_mut_slice().swap(2, 4);
    assert_eq!(*a.get(2), 4);
    assert_eq!(*a.get(4), 2);
    let mut n = 42;
    swap(&mut n, a.get_mut(0));
    assert_eq!(*a.get(0), 42);
    assert_eq!(n, 0);
}
