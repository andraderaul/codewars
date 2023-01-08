/* https://leetcode.com/problems/guess-number-higher-or-lower/description/ */
/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

struct Solution {}

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut low: i32 = 0;
        let mut high: i32 = n;

        while low <= high {
            let mid = (low + high) / 2;
            let v = guess(n);

            if v == 0 {
                return mid;
            }
            if v < 0 {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        low
    }
}

fn main() {}
