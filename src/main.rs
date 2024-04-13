use std::ptr::null;


use crate::{knuth_morris_pratt::kmp, simple_find::{naive_string_matcher, Printer}};

pub mod simple_find;
pub mod knuth_morris_pratt;
pub mod aho_corasik;
pub mod boyer_moore;


fn main() {
    let t = boyer_moore::bm("данные".to_string(), "персональные данные данные".to_string());
    println!("{}", t);
}
