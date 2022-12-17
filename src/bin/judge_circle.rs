/* https://leetcode.com/problems/robot-return-to-origin/description/ */

fn judge_circle(moves: String) -> bool {
    let (x, y) = moves.chars().fold((0, 0), |(x, y), c| match c {
        'U' => (x, y + 1),
        'D' => (x, y - 1),
        'R' => (x + 1, y),
        'L' => (x - 1, y),
        _ => panic!(),
    });

    x == 0 && y == 0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::judge_circle;

    #[test]
    fn sample_tests() {
        assert_eq!(judge_circle("UD".to_string()), true);
        assert_eq!(judge_circle("LL".to_string()), false);
    }
}
