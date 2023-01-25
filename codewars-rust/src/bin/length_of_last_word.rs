/* https://leetcode.com/problems/length-of-last-word/description/ */

pub fn length_of_last_word(s: String) -> i32 {
    if let Some(y) = s
        .split_whitespace()
        .into_iter()
        .filter(|x| &&x.len() > &&0)
        .last()
    {
        y.len() as i32
    } else {
        0
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::length_of_last_word;

    #[test]
    fn sample_tests() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
        assert_eq!(
            length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }
}
