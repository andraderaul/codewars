/* https://leetcode.com/problems/subsets/description/ */

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn convert_int_to_sub_set(x: i32, set: &Vec<i32>) -> Vec<i32> {
        let mut subset: Vec<i32> = vec![];
        let mut index = 0;

        let mut k = x;
        while k > 0 {
            if (k & 1) == 1 {
                subset.push(set[index]);
            }

            k >>= 1;
            index += 1;
        }

        subset
    }

    fn get_subset(set: Vec<i32>) -> Vec<Vec<i32>> {
        let mut all_subsets: Vec<Vec<i32>> = vec![];
        let max = 1 << set.len();
        for k in 0..max {
            let subset = convert_int_to_sub_set(k, &set);
            all_subsets.push(subset);
        }

        all_subsets
    }

    get_subset(nums)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::subsets;

    #[test]
    fn sample_tests() {
        assert_eq!(
            subsets([1, 2, 3].to_vec()),
            [
                [].to_vec(),
                [1].to_vec(),
                [2].to_vec(),
                [1, 2].to_vec(),
                [3].to_vec(),
                [1, 3].to_vec(),
                [2, 3].to_vec(),
                [1, 2, 3].to_vec()
            ]
            .to_vec()
        );
        assert_eq!(subsets([0].to_vec()), [[].to_vec(), [0].to_vec()].to_vec());
    }
}
