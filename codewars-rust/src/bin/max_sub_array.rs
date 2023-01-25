/* https://leetcode.com/problems/maximum-subarray/description/ */

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    let mut sum = 0;
    for i in 0..nums.len() {
        sum += nums[i];
        if sum > max {
            max = sum;
        }
        if sum < 0 {
            sum = 0;
        }
    }
    max
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::max_sub_array;

    #[test]
    fn sample_tests() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![11]), 11);
        assert_eq!(max_sub_array(vec![-32]), -32);
    }
}
