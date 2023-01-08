/* https://leetcode.com/problems/climbing-stairs/description/ */

use std::collections::HashMap;

pub fn climb_stairs_vec(n: i32) -> i32 {
    let mut memo = vec![-1; (n + 1) as usize];
    memo[0] = 1;
    memo[1] = 1;
    fn climb_stairs_rec(x: i32, memo: &mut Vec<i32>) -> i32 {
        let x = x as usize;
        if x == 0 {
            return memo[x];
        }

        if x == 1 {
            return memo[x];
        }

        if memo[x] == -1 {
            memo[x] =
                climb_stairs_rec((x - 1) as i32, memo) + climb_stairs_rec((x - 2) as i32, memo);
        }

        return memo[x];
    }

    climb_stairs_rec(n, &mut memo)
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();

    fn climb_stairs_rec(x: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if let Some(&r) = memo.get(&x) {
            return r;
        }

        let r = match x {
            0 => 1,
            1 => 1,
            _ => climb_stairs_rec(x - 1, memo) + climb_stairs_rec(x - 2, memo),
        };

        memo.insert(x, r);
        r
    }

    climb_stairs_rec(n, &mut memo)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::climb_stairs;

    #[test]
    fn sample_tests() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
    }
}
