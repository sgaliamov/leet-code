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
        let mut positions = [NONE; 256]; // first positions for each character
        let mut index: Vec<Vec<u16>> = vec![Vec::new(); 256]; // relative index

        for (i, l) in value.bytes().map(|l| l as usize).enumerate() {
            if positions[l] == NONE {
                positions[l] = i as u16
            }

            let pos = positions[l] as usize;
            index[pos].push(i as u16);
        }

        index
    }

    if s.len() != t.len() {
        return false;
    }

    let ind_s = index(&s);
    let ind_t = index(&t);

    ind_s.eq(&ind_t)
}

// better memory (85%).
pub fn is_isomorphic_2(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    const NONE: u16 = u16::MAX;
    let mut pos_s = [NONE; 256];
    let mut pos_t = [NONE; 256];
    let s = s.as_bytes();
    let t = t.as_bytes();

    for i in 0..s.len() {
        let cs = s[i] as usize;
        let ct = t[i] as usize;

        if pos_s[cs] == NONE {
            pos_s[cs] = i as u16
        }

        if pos_t[ct] == NONE {
            pos_t[ct] = i as u16
        }

        let ps = pos_s[cs] as usize;
        let pt = pos_t[ct] as usize;

        if ps != pt {
            return false;
        }
    }

    true
}

// beats runtime and memory
pub fn is_isomorphic_3(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();
    const NONE: u16 = u16::MAX;
    let mut map = [NONE; 256];

    for i in 0..s.len() {
        let cs = s[i] as usize;
        let ct = t[i].into();

        if map[cs] == NONE && !map.contains(&ct) {
            map[cs] = ct;
        } else if map[cs] != ct {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        is_isomorphic_1,
        is_isomorphic_2,
        is_isomorphic_3,
    );

    fn run_test(target: fn(String, String) -> bool) {
        vec![
            // ("paper", "title", true),
            // ("egg", "add", true),
            // ("foo", "bar", false),
            ("badc", "baba", false),
            // ("13", "42", true),
            // ("!!#", "%%!", true),
        ]
        .into_iter()
        .for_each(|(s, t, expected)| {
            let name = format!("{s} {t}");
            let actual = target(s.to_string(), t.to_string());
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
