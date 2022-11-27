fn fib_memo(n: i32) -> i32 {
    let mut memo: Vec<i32> = vec![0; n as usize + 2];
    memo[0] = 0;
    memo[1] = 1;

    for i in 2..=n as usize {
        memo[i] = memo[i - 1] + memo[i - 2];
    }

    memo[n as usize]
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut prev_a = 0;
    let mut prev_b = 1;

    for _ in 2..n {
        let sum = prev_a + prev_b;
        prev_a = prev_b;
        prev_b = sum;
    }

    prev_a + prev_b
}

fn main() {
    println!("{}", fib(2));
    println!("{}", fib(3));
    println!("{}", fib(4));
    println!("{}", fib(5));
}

#[cfg(test)]
mod tests {
    use super::{fib, fib_memo};

    #[test]
    fn basic() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(20), 6765);
        assert_eq!(fib(30), 832040);
        assert_eq!(fib(40), 102334155);
        assert_eq!(fib(45), 1134903170);
        assert_eq!(fib(46), 1836311903);
    }

    #[test]
    fn basic_memo() {
        assert_eq!(fib_memo(0), 0);
        assert_eq!(fib_memo(1), 1);
        assert_eq!(fib_memo(2), 1);
        assert_eq!(fib_memo(3), 2);
        assert_eq!(fib_memo(4), 3);
        assert_eq!(fib_memo(5), 5);
        assert_eq!(fib_memo(10), 55);
        assert_eq!(fib_memo(20), 6765);
        assert_eq!(fib_memo(30), 832040);
        assert_eq!(fib_memo(40), 102334155);
        assert_eq!(fib_memo(45), 1134903170);
        assert_eq!(fib_memo(46), 1836311903);
    }
}
