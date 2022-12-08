/* https://leetcode.com/problems/rotate-string/description/ */

fn rotate_string(s: String, goal: String) -> bool {
    s.len() == goal.len() && (s.to_owned() + &s).contains(&goal)
}

fn main() {
    println!(
        "{}",
        rotate_string("abcde".to_string(), "cdeab".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::rotate_string;

    #[test]
    fn sample_tests() {
        assert_eq!(
            rotate_string("abcde".to_string(), "cdeab".to_string()),
            true
        );
        assert_eq!(
            rotate_string("abcde".to_string(), "abced".to_string()),
            false
        );
    }
}
