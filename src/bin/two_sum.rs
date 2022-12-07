/* https://leetcode.com/problems/two-sum/description/ */

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;

        match hash_map.get(&complement) {
            Some(x) => return vec![i as i32, *x],
            None => {}
        }

        hash_map.insert(*num, i as i32);
    }

    Vec::new()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn returns_expected() {
        let l1 = vec![2, 7, 11, 15];
        let l2 = vec![3, 2, 4];
        let l3 = vec![3, 3];
        let l4 = vec![];

        assert_eq!(two_sum(l1, 9), vec![1, 0]);
        assert_eq!(two_sum(l2, 6), vec![2, 1]);
        assert_eq!(two_sum(l3, 6), vec![1, 0]);
        assert_eq!(two_sum(l4, 2), vec![]);
    }
}
