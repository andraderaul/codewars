pub fn fizz_buzz(n: i32) -> Vec<String> {
    let answer = (1..=n)
        .map(|x| {
            if x % 3 == 0 && x % 5 == 0 {
                return "FizzBuzz".to_string();
            } else if x % 3 == 0 {
                return "Fizz".to_string();
            } else if x % 5 == 0 {
                return "Buzz".to_string();
            }
            x.to_string()
        })
        .collect();

    answer
}

#[cfg(test)]
mod tests {
    use super::fizz_buzz;

    fn testing(n: i32, exp: Vec<String>) -> () {
        assert_eq!(fizz_buzz(n), exp)
    }

    #[test]
    fn basic_tests() {
        testing(
            3,
            vec!["1".to_string(), "2".to_string(), "Fizz".to_string()],
        );
        testing(
            5,
            vec![
                "1".to_string(),
                "2".to_string(),
                "Fizz".to_string(),
                "4".to_string(),
                "Buzz".to_string(),
            ],
        );
        testing(
            15,
            vec![
                "1".to_string(),
                "2".to_string(),
                "Fizz".to_string(),
                "4".to_string(),
                "Buzz".to_string(),
                "Fizz".to_string(),
                "7".to_string(),
                "8".to_string(),
                "Fizz".to_string(),
                "Buzz".to_string(),
                "11".to_string(),
                "Fizz".to_string(),
                "13".to_string(),
                "14".to_string(),
                "FizzBuzz".to_string(),
            ],
        );
    }
}
