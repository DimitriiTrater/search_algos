/// working time complexity O(s.len)
pub fn prefix_function(s: String) -> Vec<usize> {
    let mut p: Vec<usize> = Vec::new();
    p.resize(s.len(), 0);
    p[0] = 0;
    for i in 1..s.len()-1 {
        let mut k: usize = p[i - 1];
        while k > 0 && s.chars().nth(i).unwrap() != s.chars().nth(k as usize).unwrap() {
            k = p[k - 1];
        }
        if s.chars().nth(i).unwrap() == s.chars().nth(k).unwrap() {
            k += 1;
        }
        p[i] = k;
    }
    p
}

/// working time complexity O(p.len + t.len)
pub fn kmp(p: String, t: String) -> Vec<usize> {
    let pl = p.len();
    let tl = t.len();
    let mut answer = Vec::new();
    let search_string = p.clone() + "#" + &t;
    let pik = prefix_function(search_string);
    for i in 0..tl - 1 {
        if pik[pl + i - 1] == pl {
            answer.push(i - pl);
        }
    }
    answer
}
