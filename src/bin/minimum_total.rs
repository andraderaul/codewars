pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut memo = triangle[triangle.len() - 1].clone();

    let mut layers = (0..triangle.len() - 1).collect::<Vec<usize>>();
    layers.reverse();

    for i in layers {
        for j in 0..triangle[i].len() {
            memo[j] = triangle[i][j] + std::cmp::min(memo[j], memo[j + 1]);
        }
    }

    memo[0]
}

fn main() {
    minimum_total(
        [
            [2].to_vec(),
            [3, 4].to_vec(),
            [6, 5, 7].to_vec(),
            [4, 1, 8, 3].to_vec(),
        ]
        .to_vec(),
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_total;

    #[test]
    fn sample_tests() {
        assert_eq!(
            minimum_total(
                [
                    [2].to_vec(),
                    [3, 4].to_vec(),
                    [6, 5, 7].to_vec(),
                    [4, 1, 8, 3].to_vec()
                ]
                .to_vec()
            ),
            11
        );
        assert_eq!(minimum_total([[-10].to_vec()].to_vec()), -10);
        assert_eq!(
            minimum_total([[-1].to_vec(), [2, 3].to_vec(), [1, -1, -3].to_vec()].to_vec()),
            -1
        );
    }
}
