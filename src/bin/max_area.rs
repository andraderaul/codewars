/* https://leetcode.com/problems/container-with-most-water/description/ */

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut low = 0;
    let mut high = height.len() - 1;
    let mut max = i32::MIN;

    while low <= high {
        let current = (high - low) as i32 * i32::min(height[low], height[high]);

        if current > max {
            max = current;
        }

        if height[low] > height[high] {
            high -= 1;
        } else {
            low += 1;
        }
    }

    max
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::max_area;

    #[test]
    fn basic_tests() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
