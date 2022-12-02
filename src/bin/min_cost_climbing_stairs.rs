/* https://leetcode.com/problems/min-cost-climbing-stairs/description/ */

pub fn min_cost_climbing_stairs_2(cost: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut memo: Vec<i32> = vec![0; n];

    for i in 0..n {
        if i < 2 {
            memo[i] = cost[i];
        } else {
            memo[i] = cost[i] + memo[i - 1].min(memo[i - 2]);
        }
    }

    memo[n - 1].min(memo[n - 2])
}

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut steps = [0, 0];
    cost.into_iter().enumerate().for_each(|(ind, x)| {
        steps[ind % 2] = steps[0].min(steps[1]) + x;
    });
    *steps.iter().min().unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{min_cost_climbing_stairs, min_cost_climbing_stairs_2};

    #[test]
    fn basic_tests() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }

    #[test]
    fn basic_tests_2() {
        assert_eq!(min_cost_climbing_stairs_2(vec![10, 15, 20]), 15);
        assert_eq!(
            min_cost_climbing_stairs_2(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
