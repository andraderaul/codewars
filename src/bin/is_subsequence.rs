fn is_subsequence(s: String, t: String) -> bool {
    if s.len() > t.len() {
        return false;
    }

    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut subsequence = 0;

    for i in 0..t.len() {
        if s[subsequence] == t[i] {
            subsequence += 1;
        }
    }

    subsequence == s.len()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::is_subsequence;

    #[test]
    fn basic_tests() {
        assert_eq!(
            is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
        assert_eq!(
            is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }
}
