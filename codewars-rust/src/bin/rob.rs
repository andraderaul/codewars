/* https://leetcode.com/problems/house-robber/description/ */

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut memo = vec![0; nums.len() + 1];
    memo[1] = nums[0];

    for i in 1..nums.len() {
        memo[i + 1] = std::cmp::max(memo[i - 1] + nums[i], memo[i])
    }

    memo[nums.len()]
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::rob;

    #[test]
    fn sample_tests() {
        assert_eq!(rob([1, 2, 3, 1].to_vec()), 4);
        assert_eq!(rob([2, 7, 9, 3, 1].to_vec()), 12);
        assert_eq!(rob([2, 1, 1, 2].to_vec()), 4);
        assert_eq!(rob([2, 1, 1, 2].to_vec()), 4);
        assert_eq!(rob([1].to_vec()), 1);
    }
}
