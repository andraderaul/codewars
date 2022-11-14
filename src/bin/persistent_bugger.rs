fn prod(n: u64) -> u64 {
    let mut n = n;
    let mut prod = 1;
    while n > 0 {
        prod *= n % 10;
        n /= 10;
    }
    prod
}

pub fn persistence(num: u64) -> u64 {
    let mut n = num;
    let mut count = 0;
    while n > 9 {
        n = prod(n);
        count += 1;
    }
    count
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::persistence;

    #[test]
    fn sample_tests() {
        assert_eq!(persistence(39), 3);
        assert_eq!(persistence(4), 0);
        assert_eq!(persistence(25), 2);
        assert_eq!(persistence(999), 4);
    }
}
