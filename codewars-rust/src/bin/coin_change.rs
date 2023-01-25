use std::cmp;
/* https://leetcode.com/problems/coin-change/description/ */

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let max = amount + 1;
    let mut memo: Vec<usize> = vec![max; max];
    memo[0] = 0;

    for i in 1..max {
        for j in 0..coins.len() {
            let current = coins[j] as usize;
            if current <= i {
                memo[i] = cmp::min(memo[i], memo[i - current] + 1)
            }
        }
    }

    if memo[amount] > amount {
        return -1;
    }

    return memo[amount] as i32;
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::coin_change;

    #[test]
    fn basic_tests() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(coin_change(vec![2], 3), -1);
        assert_eq!(coin_change(vec![1], 0), 0);
        assert_eq!(coin_change(vec![1, 2], 2), 1);
        assert_eq!(coin_change(vec![], 2), -1);
    }
}
