/* https://www.codewars.com/kata/54da539698b8a2ad76000228 */

fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    }
    let mut x = 0;
    let mut y = 0;

    for step in walk {
        match step {
            &'n' => x += 1,
            &'s' => x -= 1,
            &'e' => y += 1,
            &'w' => y -= 1,
            _ => panic!("Expected n, s, w, e"),
        }
    }

    x == 0 && y == 0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::is_valid_walk;

    #[test]
    fn sample_tests() {
        assert!(is_valid_walk(&[
            'n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e'
        ]));
        assert!(!is_valid_walk(&['w']));
        assert!(!is_valid_walk(&[
            'n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's'
        ]))
    }
}
