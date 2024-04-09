use core::fmt;

/// Working time
/// O(m⋅(n−m))
/// in bad m = n/2
/// O(n^2/4) = O(n^2)

pub fn naive_string_matcher(str: String, sub_str: String) -> Vec<i32> {
    let str_len = str.len();
    let sub_str_len = sub_str.len();
    let mut ans = Vec::new();
    for i in 0..str_len-sub_str_len {
        if str[i..i + sub_str_len] == sub_str {
            ans.push(i as i32);
        }
    }
    ans
}


pub struct Printer(pub Vec<i32>);

impl fmt::Display for Printer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Values:\n")?;
        for v in &self.0 {
            write!(f, "\t{}", v)?;
        }
        Ok(())
    }
}


