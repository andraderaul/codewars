pub fn bingo<S: AsRef<str>>(ticket: &[(S, u8)], win: usize) -> &'static str {
    match ticket
        .iter()
        .filter(|(s, n)| s.as_ref().as_bytes().contains(n))
        .count()
        >= win
    {
        true => "Winner!",
        _ => "Loser!",
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 2),
            "Loser!"
        );
        assert_eq!(
            bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 1),
            "Winner!"
        );
        assert_eq!(
            bingo(&[("HGTYRE", 74), ("BE", 66), ("JKTY", 74)], 3),
            "Loser!"
        );
        assert_eq!(
            bingo(
                &[
                    ("FSZMS", 89),
                    ("BSEOXB", 86),
                    ("NNPNKBC", 78),
                    ("QWFFHX", 67),
                    ("ITTC", 66)
                ],
                1
            ),
            "Winner!"
        );

        assert_eq!(
            bingo(
                &[
                    ("ZJCDPUPQ", 80),
                    ("OAGMV", 69),
                    ("CNJD", 81),
                    ("OJAHYV", 79),
                    ("DXSBAIU", 66),
                    ("WB", 69),
                    ("EGBSTKW", 84),
                    ("UHXNG", 68)
                ],
                5
            ),
            "Loser!"
        );
    }
}
