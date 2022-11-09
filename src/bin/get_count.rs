fn get_count(string: &str) -> usize {
    string
        .matches(|x| match x {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .count()
}

fn main() {
    let x = get_count("abracadabra");

    println!("{}", x)
}

#[cfg(test)]
mod tests {
    use super::get_count;
    #[test]
    fn my_tests() {
        assert_eq!(get_count("abracadabra"), 5);
    }
}
