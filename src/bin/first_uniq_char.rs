/* https://leetcode.com/problems/first-unique-character-in-a-string/description/ */

use std::collections::HashMap;

pub fn first_uniq_char(s: String) -> i32 {
    let mut hash_map = HashMap::new();

    for key in s.chars() {
        hash_map
            .entry(key)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut out: i32 = -1;

    for (idx, ch) in s.chars().enumerate() {
        if hash_map[&ch] == 1 {
            out = idx as i32;
            break;
        }
    }

    out
}

pub fn first_uniq_char_2(s: String) -> i32 {
    let mut unique = [-1i32; 26];

    for (i, b) in s.bytes().enumerate() {
        let b_i = (b - b'a') as usize;
        let val = unique[b_i];

        unique[b_i] = if val == -1 { i as i32 } else { -2 };
    }

    match unique.iter().filter(|i| **i >= 0).min() {
        Some(p) => *p,
        None => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::{first_uniq_char, first_uniq_char_2};

    #[test]
    fn returns_expected() {
        assert_eq!(first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
        assert_eq!(first_uniq_char("aabb".to_string()), -1);
    }

    #[test]
    fn returns_expected_2() {
        assert_eq!(first_uniq_char_2("leetcode".to_string()), 0);
        assert_eq!(first_uniq_char_2("loveleetcode".to_string()), 2);
        assert_eq!(first_uniq_char_2("aabb".to_string()), -1);
    }
}
