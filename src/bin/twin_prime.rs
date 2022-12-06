fn is_prime(x: usize) -> bool {
    if x == 1 {
        return false;
    }

    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }

    true
}

fn twin_prime(n: i32) -> u32 {
    if n < 0 {
        return 0;
    }

    let max = (n + 2) as usize;
    let mut twin = 0;
    let mut primes = vec![false; max];

    for i in 1..(n + 1) as usize {
        let next = i + 1;
        let prev = i - 1;
        primes[next] = is_prime(next);

        if primes[prev] && primes[next] {
            twin += 1;
        }
    }

    twin
}

fn main() {
    println!("{}", twin_prime(0));
}

#[cfg(test)]
mod tests {
    use super::twin_prime;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(n: i32, expected: u32) {
        assert_eq!(twin_prime(n), expected, "{ERR_MSG} with n = {n}")
    }

    #[test]
    fn returns_expected() {
        dotest(-25, 0);
        dotest(0, 0);
        dotest(2, 0);
        dotest(10, 2);
        dotest(12, 3);
        dotest(25, 4);
    }
}
