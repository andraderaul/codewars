/* https://leetcode.com/problems/search-insert-position/description/  */

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low: i32 = 0;
        let mut high: i32 = (nums.len() - 1) as i32;

        while low <= high {
            let mid = (low + high) / 2;
            let guess = nums[mid as usize];

            if guess == target {
                return mid;
            }
            if guess > target {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        low
    }

    binary_search(nums, target)
}

fn main() {
    println!("{}", search_insert(vec![1, 3, 5, 6], 5))
}

#[cfg(test)]
mod tests {
    use super::search_insert;

    #[test]
    fn basic_tests() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
