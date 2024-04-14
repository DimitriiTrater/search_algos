

use crate::{knuth_morris_pratt::kmp, simple_find::{naive_string_matcher, Printer}};

pub mod simple_find;
pub mod knuth_morris_pratt;
pub mod aho_corasik;
pub mod boyer_moore;
pub mod rabin_karp;


fn main() {
    let t = rabin_karp::rk("e".to_string(), "zmeeed".to_string());
    println!("{}", Printer(t));
}
