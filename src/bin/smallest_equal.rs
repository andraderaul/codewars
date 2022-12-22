/* https://leetcode.com/problems/smallest-index-with-equal-value/description/ */
pub fn smallest_equal(nums: Vec<i32>) -> i32 {
    let mut out: i32 = -1;

    for i in 0..nums.len() {
        if (i % 10) as i32 == nums[i] {
            out = i as i32;
            break;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::smallest_equal;

    #[test]
    fn sample_tests() {
        assert_eq!(smallest_equal([0, 1, 2].to_vec()), 0);
        assert_eq!(smallest_equal([4, 3, 2, 1].to_vec()), 2);
        assert_eq!(smallest_equal([1, 2, 3, 4, 5, 6, 7, 8, 9, 0].to_vec()), -1);
    }
}
