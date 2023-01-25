/* https://leetcode.com/problems/powx-n/description/ */

pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n % 2 == 0 {
        my_pow(x, n / 2) * my_pow(x, n / 2)
    } else {
        if n > 0 {
            x * my_pow(x, n / 2) * my_pow(x, n / 2)
        } else {
            (my_pow(x, n / 2) * my_pow(x, n / 2)) / x
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_tests() {
        assert_eq!(my_pow(2.0, 10), 1024.0);
        assert_eq!(my_pow(2.1, 3), 9.261000000000001);
        assert_eq!(my_pow(2.0, -2), 0.25);
        assert_eq!(my_pow(0.44528, 0), 1_f64);
        assert_eq!(my_pow(0.00001, 2147483647), 0_f64);
        assert_eq!(my_pow(1.00000, -2147483648), 1_f64);
    }
}
