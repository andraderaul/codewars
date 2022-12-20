pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min = prices[0];
    let mut max = 0;

    for price in prices {
        min = min.min(price);
        max = max.max(price - min);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn sample_tests() {
        assert_eq!(max_profit([7, 1, 5, 3, 6, 4].to_vec()), 5);
        assert_eq!(max_profit([7, 6, 4, 3, 1].to_vec()), 0);
    }
}
