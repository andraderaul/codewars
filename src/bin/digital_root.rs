fn digital_root(n: i64) -> i64 {
    (n - 1) % 9 + 1
}

fn main() {
    let x = digital_root(16);

    println!("{}", x)
}

#[cfg(test)]
mod tests {
    use super::digital_root;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
    }
}
