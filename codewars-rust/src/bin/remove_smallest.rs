/* https://www.codewars.com/kata/563cf89eb4747c5fb100001b/train/rust */

fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let min = numbers.iter().min().unwrap_or(&0);
    let mut arr = vec![];
    let mut flag = false;

    for &x in numbers {
        if !flag && x.eq(&min) {
            flag = true;
            continue;
        }
        arr.push(x);
    }

    arr
}

fn main() {
    println!("{:?}", remove_smallest(&[1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::remove_smallest;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[u32], expected: &[u32]) {
        assert_eq!(
            remove_smallest(a),
            expected,
            "{ERR_MSG} with numbers = {a:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5], &[2, 3, 4, 5]);
        dotest(&[1, 2, 3, 4], &[2, 3, 4]);
        dotest(&[5, 3, 2, 1, 4], &[5, 3, 2, 4]);
        dotest(&[1, 2, 3, 1, 1], &[2, 3, 1, 1]);
    }
}
