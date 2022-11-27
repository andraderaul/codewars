pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort_by(|a, b| b.cmp(a));
    s.sort_by(|a, b| b.cmp(a));

    let mut greedy = 0;
    let mut i = 0;

    while i < g.len() && greedy < s.len() {
        if s[greedy] >= g[i] {
            greedy += 1
        }
        i += 1;
    }

    let x: i32 = greedy.try_into().unwrap();

    x
}

fn main() {
    println!("{}", find_content_children([1, 2, 3].to_vec(), [].to_vec()));

    println!(
        "{}",
        find_content_children([1, 2, 3].to_vec(), [1, 1].to_vec())
    );
}

#[cfg(test)]
mod tests {
    use super::find_content_children;

    #[test]
    fn returns_expected() {
        assert_eq!(
            find_content_children([1, 2, 3].to_vec(), [1, 1].to_vec()),
            1
        );
        assert_eq!(
            find_content_children([1, 2].to_vec(), [1, 2, 3].to_vec()),
            2
        );
        assert_eq!(
            find_content_children([10, 9, 8, 7].to_vec(), [5, 6, 7, 8].to_vec()),
            2
        );
    }
}
