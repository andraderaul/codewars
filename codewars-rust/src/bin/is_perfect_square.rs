/* https://leetcode.com/problems/valid-perfect-square/description/ */

pub fn is_perfect_square(num: i32) -> bool {
    fn binary_search(low: i32, high: i32, target: i32) -> f32 {
        let mid = (high as f32 + low as f32) / 2.0;
        let mid_squared = mid * mid;
        println!("mid {} low {} high {}", mid, low, high);

        if mid_squared == target as f32 {
            return mid;
        }

        if high - 1 == low {
            if high * high == target {
                return high as f32;
            } else if low * low == target {
                return low as f32;
            }

            return mid as f32;
        }

        if mid_squared > target as f32 {
            return binary_search(low, mid as i32, target);
        } else {
            return binary_search(mid as i32, high, target);
        }
    }

    let x = binary_search(0, num, num);

    x.trunc() == x
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::is_perfect_square;

    #[test]
    fn sample_tests() {
        assert_eq!(is_perfect_square(16), true);
        assert_eq!(is_perfect_square(14), false);
        assert_eq!(is_perfect_square(1), true);
        assert_eq!(is_perfect_square(5), false);
        assert_eq!(is_perfect_square(2), false);
        assert_eq!(is_perfect_square(100), true);
    }
}
