pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let i = nums.partition_point(|n| n < &target);

    if i == nums.len() || nums[i] != target {
        return vec![-1, -1];
    }

    let j = nums.partition_point(|n| n <= &target);

    vec![i as i32, (j - 1) as i32]
}

fn main() {
    println!("{:?}", search_range(vec![], 0));
}

#[cfg(test)]
mod tests {
    use super::search_range;

    #[test]
    fn basic_tests() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(search_range(vec![], 0), vec![-1, -1]);
        assert_eq!(search_range(vec![1], 0), vec![-1, -1]);
    }
}
