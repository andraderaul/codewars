/* https://www.codewars.com/kata/5a63948acadebff56f000018/train/rust */

fn max_product(mut lst: Vec<i32>, n_largest_elements: i32) -> i32 {
    lst.sort_by(|a, b| b.cmp(a));
    lst.iter().take(n_largest_elements as usize).product()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_tests() {
        assert_eq!(max_product(vec![0; 10], 5), 0);
        assert_eq!(max_product(vec![4, 3, 5], 2), 20);
        assert_eq!(max_product(vec![10, 8, 7, 9], 3), 720);
        assert_eq!(max_product(vec![8, 6, 4, 6], 3), 288);
    }
}
