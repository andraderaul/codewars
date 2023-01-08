/* https://leetcode.com/problems/n-th-tribonacci-number/description/ */

pub fn tribonacci(n: i32) -> i32 {
    let mut memo = vec![0; n as usize + 3];
    memo[0] = 0;
    memo[1] = 1;
    memo[2] = 1;

    for i in 3..=n as usize {
        memo[i] = memo[i - 1] + memo[i - 2] + memo[i - 3];
    }

    memo[n as usize]
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::tribonacci;

    #[test]
    fn basic() {
        assert_eq!(tribonacci(4), 4);
        assert_eq!(tribonacci(25), 1389537)
    }
}
