/* https://leetcode.com/problems/intersection-of-two-arrays/description/ */

use std::collections::HashMap;

fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut hash_map = HashMap::new();
    let mut ans = vec![];

    for num in nums1 {
        hash_map
            .entry(num)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for num in nums2 {
        if let Some(x) = hash_map.get(&num) {
            if *x > 0 {
                if !ans.contains(&num) {
                    ans.push(num);
                }
                hash_map.entry(num).and_modify(|counter| *counter -= 1);
            }
        }
    }

    ans
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::intersection;

    #[test]
    fn sample_tests() {
        assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
        assert_eq!(intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![9, 4]);
    }
}
