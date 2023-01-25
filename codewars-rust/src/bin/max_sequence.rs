/* https://www.codewars.com/kata/54521e9ec8e60bc4de000d6c/train/rust */

fn max_sequence(seq: &[i32]) -> i32 {
    let mut ans = 0;

    seq.iter().fold(0, |prev, &v| {
        let p = v.max(prev + v);
        ans = ans.max(p);
        p
    });

    ans
}

fn main() {
    println!("{}", max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_sequence;

    #[test]
    fn sample_tests() {
        assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sequence(&[11]), 11);
        assert_eq!(max_sequence(&[-32]), 0);
        assert_eq!(max_sequence(&[]), 0);
    }
}
