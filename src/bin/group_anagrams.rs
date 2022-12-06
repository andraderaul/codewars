use std::collections::HashMap;

/* https://leetcode.com/problems/group-anagrams/description/ */

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hash_map = HashMap::new();

    for s in strs {
        let mut key: Vec<char> = s.chars().collect();
        key.sort();
        hash_map.entry(key).or_insert(vec![]).push(s);
    }

    hash_map.values().cloned().collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::group_anagrams;

    #[test]
    fn returns_expected() {
        assert_eq!(
            group_anagrams(
                [
                    "eat".to_string(),
                    "tea".to_string(),
                    "tan".to_string(),
                    "ate".to_string(),
                    "nat".to_string(),
                    "bat".to_string()
                ]
                .to_vec(),
            ),
            [
                ["eat".to_string(), "tea".to_string(), "ate".to_string()].to_vec(),
                ["tan".to_string(), "nat".to_string()].to_vec(),
                ["bat".to_string()].to_vec()
            ]
            .to_vec()
        );
        assert_eq!(
            group_anagrams(["".to_string()].to_vec()),
            [["".to_string()].to_vec()].to_vec()
        );
        assert_eq!(
            group_anagrams(["a".to_string()].to_vec()),
            [["a".to_string()].to_vec()].to_vec()
        );
    }
}
