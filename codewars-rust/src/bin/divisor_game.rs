// https://leetcode.com/problems/divisor-game/description/

pub fn divisor_game(n: i32) -> bool {
    n % 2 == 0
}

fn divisor_game_rec(n: i32) -> bool {
    let n = n as usize;
    let mut memo: Vec<bool> = vec![false; n + 2];
    memo[0] = true;
    memo[1] = false;
    memo[2] = true;

    fn rec(dp: &mut Vec<bool>, i: usize) -> bool {
        if dp[i] != false {
            return dp[i];
        }

        dp[i] = !rec(dp, i - 1);

        return dp[i];
    }

    rec(&mut memo, n);

    memo[n]
}

fn main() {
    println!("{}", divisor_game_rec(20));
    println!("{}", divisor_game_rec(1))
}

#[cfg(test)]
mod tests {
    use super::{divisor_game, divisor_game_rec};

    #[test]
    fn basic_tests() {
        assert_eq!(divisor_game(2), true);
        assert_eq!(divisor_game(3), false);
        assert_eq!(divisor_game(30), true);
    }

    #[test]
    fn basic_tests_rec() {
        assert_eq!(divisor_game_rec(2), true);
        assert_eq!(divisor_game_rec(3), false);
        assert_eq!(divisor_game_rec(30), true);
    }
}
