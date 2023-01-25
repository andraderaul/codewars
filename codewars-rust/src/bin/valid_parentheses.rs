/* https://www.codewars.com/kata/52774a314c2333f0a7000688/train/rust */

fn valid_parentheses(s: &str) -> bool {
    let mut opened = 0usize;

    for ch in s.chars() {
        match ch {
            '(' => opened += 1,
            ')' if opened > 0 => opened -= 1,
            ')' => return false,
            _ => (), // ignore
        }
    }
    opened == 0
}

fn main() {
    println!("{}", valid_parentheses("()"));
}

#[cfg(test)]
mod tests {
    use super::valid_parentheses;

    #[test]
    fn sample_tests() {
        do_test("()", true);
        do_test("())", false);
        do_test("", true);
        do_test("(}{)", true);
        do_test("((((((((", false);
        do_test("(]", false);
        do_test(")()()()(", false);
        do_test("  ", true);
        do_test(
            "xiqq[(vof]uta]zqlaqbuqf{ibnshtpkiwmnqd(huzb[kpmiwslor[cnmkhkwe[]aaxysficf]n}cfgvm",
            false,
        );
    }

    fn do_test(s: &str, exp: bool) {
        let actual = valid_parentheses(s);
        assert_eq!(
            actual, exp,
            "\nFor the input \"{}\", your result ({}) did not match the expected output ({})",
            s, actual, exp
        );
    }
}
