pub fn check_record(s: String) -> bool {
    s.find("A") == s.rfind("A") && s.find("LLL") == None
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::check_record;

    #[test]
    fn sample_tests() {
        assert_eq!(check_record("PPALLP".to_string()), true);
        assert_eq!(check_record("PPALLL".to_string()), false);
        assert_eq!(check_record("PPALLPL".to_string()), true);
        assert_eq!(check_record("PAPALLPL".to_string()), false);
        assert_eq!(check_record("LALL".to_string()), true);
    }
}
