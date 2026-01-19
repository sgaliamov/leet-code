//! https://leetcode.com/problems/ransom-note
//!
//! Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.
//! Each letter in magazine can only be used once in ransomNote.

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    use itertools::Itertools;
    use std::collections::HashMap;
    let mut letters: HashMap<_, _> = magazine
        .as_bytes()
        .iter()
        .sorted()
        .chunk_by(|&x| x)
        .into_iter()
        .map(|(&k, g)| (k, g.count()))
        .collect();

    for &l in ransom_note.as_bytes() {
        let Some(e) = letters.get_mut(&l) else {
            return false;
        };

        if e == &0 {
            return false;
        }

        *e -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test() {
        run_test(can_construct);
    }

    fn run_test(target: fn(String, String) -> bool) {
        vec![("a", "b", false), ("aa", "ab", false), ("aa", "aab", true)]
            .into_iter()
            .for_each(|(ransom_note, magazine, expected)| {
                let name = format!("{ransom_note} {magazine}");
                let actual = target(ransom_note.to_string(), magazine.to_string());
                assert_that!(actual).named(&name).is_equal_to(expected);
            });
    }
}
