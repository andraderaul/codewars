/* https://www.codewars.com/kata/5a7893ef0025e9eb50000013/train/rust */

fn max_gap(xs: &[i32]) -> i32 {
    let mut v = Vec::from(xs);
    v.sort_by(|a, b| b.cmp(a));
    v.windows(2).map(|x| x[0] - x[1]).max().unwrap_or(0)
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_positive_values() {
        assert_eq!(max_gap(&[13, 10, 2, 9, 5]), 4);
        assert_eq!(max_gap(&[13, 3, 5]), 8);
        assert_eq!(max_gap(&[24, 299, 131, 14, 26, 25]), 168);
    }

    #[test]
    fn check_negative_values() {
        assert_eq!(max_gap(&[-3, -27, -4, -2]), 23);
        assert_eq!(max_gap(&[-7, -42, -809, -14, -12]), 767);
    }

    #[test]
    fn check_mixed_values() {
        assert_eq!(max_gap(&[12, -5, -7, 0, 290]), 278);
        assert_eq!(max_gap(&[-54, 37, 0, 64, -15, 640, 0]), 576);
    }

    #[test]
    fn random_values() {
        assert_eq!(
            max_gap(&[
                71, 67, 38, -31, -15, -75, 32, 70, -16, -54, 36, -74, 7, -40, -93, -30, -75, 69,
                19, 35, -81, -18, -23, -37, 73, 53, -3, 17, -90, 80, 25, 95, -76, -6, 65, 30, 28,
                -22, -23, -72, -84, 48, 89, 6, -80, 86, 31, 16, -20, -57, 61, -35, -62, 51, -87,
                -56, 77, -55, 7, 35, -91, -70, -39, 89, 13, -42, 79, -98, -12, -64, 35, -59, -91,
                -68, -23, -45, -11, 90, 2, 0, -54, 47, 55, 61, -65, 41, -18, -94, -97, -37, 15,
                -64, 20, 23, -6, -45, -46, -51, -30, -18, -6, 89, 9
            ]),
            7
        )
    }
}
