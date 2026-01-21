//! https://leetcode.com/problems/isomorphic-strings
//!
//! Given two strings s and t, determine if they are isomorphic.
//! Two strings s and t are isomorphic if the characters in s can be replaced to get t.
//! All occurrences of a character must be replaced with another character while preserving the order of characters.
//! No two characters may map to the same character, but a character may map to itself.

// first attempt.
// beats 100%, but failed on memory.
pub fn is_isomorphic_1(s: String, t: String) -> bool {
    fn index(value: &str) -> Vec<Vec<u16>> {
        const NONE: u16 = u16::MAX;
        let mut pos_s = [NONE; 256]; // first positions for each character
        let mut ind_s: Vec<Vec<u16>> = vec![Vec::new(); 256]; // relative index

        for (i, l) in value.bytes().map(|l| l as usize).enumerate() {
            if pos_s[l] == NONE {
                pos_s[l] = i as u16
            }

            let pos = pos_s[l] as usize;
            ind_s[pos].push(i as u16);
        }

        ind_s
    }

    if s.len() != t.len() {
        return false;
    }

    let ind_s = index(&s);
    let ind_t = index(&t);

    ind_s.eq(&ind_t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        is_isomorphic_1,
    );

    fn run_test(target: fn(String, String) -> bool) {
        vec![
            ("paper", "title", true),
            ("egg", "add", true),
            ("foo", "bar", false),
            ("13", "42", true),
            ("!!#", "%%!", true),
        ]
        .into_iter()
        .for_each(|(s, t, expected)| {
            let name = format!("{s} {t}");
            let actual = target(s.to_string(), t.to_string());
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
