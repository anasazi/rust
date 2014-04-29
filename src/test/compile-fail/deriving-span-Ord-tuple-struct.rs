// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This file was auto-generated using 'src/etc/generate-keyword-span-tests.py'

#![feature(struct_variant)]
extern crate rand;

#[deriving(Eq)]
struct Error;

#[deriving(Eq, Ord)]
struct Struct(
    Error //~ ERROR
//~^ ERROR
//~^^ ERROR
//~^^^ ERROR
//~^^^^ ERROR
//~^^^^^ ERROR
//~^^^^^^ ERROR
//~^^^^^^^ ERROR
);

fn main() {}
