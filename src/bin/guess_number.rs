/* https://leetcode.com/problems/guess-number-higher-or-lower/description/ */
/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

pub struct Solution {}

impl Solution {
    pub unsafe fn guess_number(n: i32) -> i32 {
        let (mut low, mut high) = (1, n);
        loop {
            let mid = low + (high - low) / 2;
            match guess(mid) {
                0 => break mid,
                -1 => high = mid - 1,
                _ => low = mid + 1,
            }
        }
    }
}

fn guess(mid: i32) -> i32 {
    println!("{}", mid);
    todo!()
}

fn main() {}
