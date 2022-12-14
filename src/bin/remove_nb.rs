fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    println!("{}", m);
    let sum: i64 = (i64::from(m) * (i64::from(m + 1))) / 2;
    println!("{}", sum);

    let mut ans = vec![];

    for i in 1..=m {
        for j in 1..=m {
            let prod = i64::from(i) * i64::from(j);
            if prod == (sum - (i64::from(i) + i64::from(j))) {
                ans.push((i, j));
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::remove_nb;

    fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
        assert_eq!(remove_nb(n), exp)
    }

    #[test]
    fn basics_remove_nb() {
        // testing(26, vec![(15, 21), (21, 15)]);
        // testing(100, vec![]);
        // testing(101, vec![(55, 91), (91, 55)]);
        // testing(102, vec![(70, 73), (73, 70)]);
        testing(1000003, vec![(70, 73), (73, 70)]);
    }
}
