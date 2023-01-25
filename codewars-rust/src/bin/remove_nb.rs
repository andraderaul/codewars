/* https://www.codewars.com/kata/5547cc7dcad755e480000004 */

pub fn remove_nb_2(m: i32) -> Vec<(i32, i32)> {
    let n: i64 = m as i64;
    let sum: i64 = n * (n + 1) / 2;
    let mut result = vec![];
    for a in 1..n {
        if (sum - a) % (a + 1) == 0 {
            let b = (sum - a) / (a + 1);
            if b < n {
                result.push((a as i32, b as i32))
            }
        }
    }
    result
}

pub fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    fn binary_search(nums: &Vec<i32>, i: i64, sum: i64) -> i64 {
        let mut low: i64 = 0;
        let mut high: i64 = (nums.len() - 1) as i64;

        while low <= high {
            let mid = (low + high) / 2;
            let guess = i64::from(nums[mid as usize]);
            let target = sum - (i + guess) - (i * guess);

            if target == 0 {
                return guess;
            }
            if target < 0 {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        return -1;
    }

    let sum: i64 = (i64::from(m) * (i64::from(m + 1))) / 2;

    let mut ans = vec![];
    let arr: Vec<i32> = (1..=m).collect();

    for i in 1..=m {
        let j = binary_search(&arr, i64::from(i), sum);
        if j != -1 {
            ans.push((i, j as i32));
        }
    }

    ans
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::remove_nb;

    fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
        assert_eq!(remove_nb(n), exp)
    }

    #[test]
    fn basics_remove_nb() {
        testing(26, vec![(15, 21), (21, 15)]);
        testing(100, vec![]);
        testing(101, vec![(55, 91), (91, 55)]);
        testing(102, vec![(70, 73), (73, 70)]);
        testing(
            1000003,
            vec![
                (550320, 908566),
                (559756, 893250),
                (893250, 559756),
                (908566, 550320),
            ],
        );
    }
}
