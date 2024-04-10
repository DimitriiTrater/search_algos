use std::ptr::null;


use crate::{knuth_morris_pratt::kmp, simple_find::{naive_string_matcher, Printer}};

pub mod simple_find;
pub mod knuth_morris_pratt;
pub mod aho_corasik;


fn main() {
    let t = kmp("k".to_string(), "lol kek lol kek lol kek".to_string());
    println!("{}", Printer(t));
}
