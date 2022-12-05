fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool {
    (0..strng.len())
        .map(|index| strng[index..].to_owned() + &strng[..index])
        .all(|x| arr.contains(&x.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(strng: &str, arr: Vec<&str>, exp: bool) -> () {
        println!("strng: {}", strng);
        println!("arr: {:?}", arr);
        let ans = contain_all_rots(strng, arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basis_tests() {
        dotest("", vec![], true);
        dotest(
            "bsjq",
            vec!["bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"],
            true,
        );
        dotest(
            "XjYABhR",
            vec![
                "TzYxlgfnhf",
                "yqVAuoLjMLy",
                "BhRXjYA",
                "YABhRXj",
                "hRXjYAB",
                "jYABhRX",
                "XjYABhR",
                "ABhRXjY",
            ],
            false,
        );
    }
}
