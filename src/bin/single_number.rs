pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, num| acc ^ num)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn returns_expected() {
        let l1 = vec![2, 2, 1];
        let l2 = vec![4, 1, 2, 1, 2];
        let l3 = vec![1];

        assert_eq!(single_number(l1), 1);
        assert_eq!(single_number(l2), 4);
        assert_eq!(single_number(l3), 1);
    }
}
