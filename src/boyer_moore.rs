use std::collections::HashMap;

type Table = HashMap<char, i32>;

pub fn get_true_len(s: String) -> usize {
    let mut true_len = 0;
    for _ in s.chars() {
        true_len+=1;
    }
    true_len
}


fn make_table(s: String) -> Table {
    let mut table = Table::new();
    let true_len = get_true_len(s.clone());
    for i in (0..true_len-1).rev() {
        let symbol = s.chars().nth(i).unwrap();
        if !table.contains_key(&symbol) {
            table.insert(symbol, (true_len - i - 1) as i32);
        }
    }
    let symbol = s.chars().nth(true_len-1).unwrap();
    if !table.contains_key(&symbol) {
        table.insert(symbol, (true_len) as i32);
    }
    table
}


pub fn bm(pattern: String, str: String) -> bool {
    let table = make_table(pattern.clone());
    let true_len = get_true_len(pattern.clone());
    
    let mut i = true_len.clone() - 1;
    let pattern_len = true_len.clone();
    let str_len = get_true_len(str.clone());
    let mut succses = false;
    loop {
        if i >= str_len {
            return false;
        }
        let mut j = i.clone();
        for s in (0..pattern_len).rev() {
            let pattern_symbol = pattern.chars().nth(s).unwrap();
            let symbol = str.chars().nth(j).unwrap();
            if pattern_symbol == symbol {
                succses = true;
                j -= 1;
                continue;
            } else {
                succses = false;
                break;
            }
        }
        if succses {
            return succses;
        }
        let symbol = str.chars().nth(i).unwrap();
        if !table.contains_key(&symbol) {
            i += true_len.clone();
        } else {
            i += (table[&symbol]) as usize;
        }
    }
}
