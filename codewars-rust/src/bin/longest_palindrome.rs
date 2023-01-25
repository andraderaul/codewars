/* https://leetcode.com/problems/longest-palindrome/description/ */

use std::collections::HashSet;

pub fn longest_palindrome(s: String) -> i32 {
    let mut set = HashSet::new();
    let mut count = 0;

    for c in s.chars() {
        if set.contains(&c) {
            set.remove(&c);
            count += 1;
        } else {
            set.insert(c);
        }
    }

    let ans = if set.len() > 0 {
        count * 2 + 1
    } else {
        count * 2
    };

    ans
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::longest_palindrome;

    fn testing(s: String, exp: i32) -> () {
        assert_eq!(longest_palindrome(s), exp)
    }

    #[test]
    fn basic_tests() {
        testing("abccccdd".to_string(), 7);
        testing("a".to_string(), 1);
    }
}
