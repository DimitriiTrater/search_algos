use core::hash;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::boyer_moore::get_true_len;


fn hash<T: Hash>(obj: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}


pub fn rk(pattern: String, str: String) -> Vec<i32> {
    let s_len = get_true_len(str.clone());
    let p_len = get_true_len(pattern.clone());


    let p_hash = hash(pattern);
    let mut s_hash = 0;

    let mut ans = Vec::new();
    for i in 0..s_len {
        if s_hash == p_hash {
            ans.push(i as i32);
        }
        println!("{}", &str[i..i+p_len].to_string());
        s_hash = hash(&str[i..i+p_len].to_string());
    }
    ans
}
