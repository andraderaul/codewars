/* https://www.codewars.com/kata/515f51d438015969f7000013 */

fn pyramid(n: usize) -> Vec<Vec<u32>> {
    (1..=n).map(|i| vec![1; i]).collect()
}

fn main() {
    println!("{:?}", pyramid(3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(pyramid(0), vec![] as Vec<Vec<u32>>);
        assert_eq!(pyramid(1), vec![vec![1]]);
        assert_eq!(pyramid(2), vec![vec![1], vec![1, 1]]);
        assert_eq!(pyramid(3), vec![vec![1], vec![1, 1], vec![1, 1, 1]]);
    }
}
