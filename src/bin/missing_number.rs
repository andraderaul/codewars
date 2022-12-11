/* https://leetcode.com/problems/missing-number/description/ */

fn missing_number(nums: Vec<i32>) -> i32 {
    let total = ((nums.len() + 1) * nums.len() / 2) as i32;
    let sum = nums.iter().sum::<i32>();

    total - sum
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::missing_number;

    #[test]
    fn basic_tests() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
        assert_eq!(missing_number(vec![0, 1]), 2);
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
