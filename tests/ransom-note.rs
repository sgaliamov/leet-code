//! https://leetcode.com/problems/ransom-note
//!
//! Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.
//! Each letter in magazine can only be used once in ransomNote.

/// slow
pub fn can_construct_1(ransom_note: String, magazine: String) -> bool {
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

    for l in ransom_note.as_bytes() {
        let Some(e) = letters.get_mut(l) else {
            return false;
        };

        if e == &0 {
            return false;
        }

        *e -= 1;
    }

    true
}

/// still slow
pub fn can_construct_2(ransom_note: String, magazine: String) -> bool {
    use std::collections::HashMap;
    let mut letters: HashMap<_, _> = Default::default();

    for l in magazine.as_bytes() {
        letters.entry(l).and_modify(|e| *e += 1).or_insert(1);
    }

    for l in ransom_note.as_bytes() {
        let Some(e) = letters.get_mut(l) else {
            return false;
        };

        if e == &0 {
            return false;
        }

        *e -= 1;
    }

    true
}

/// beats 100%, overcomplicated
pub fn can_construct_3(ransom_note: String, magazine: String) -> bool {
    use std::collections::HashMap;
    use std::hash::*;

    fn find<S: BuildHasher>(
        letter: &u8,
        mut mi: usize,
        magazine: &[u8],
        letters: &mut HashMap<u8, i32, S>,
    ) -> Option<usize> {
        if let Some(e) = letters.get(letter)
            && e != &0
        {
            return Some(mi);
        }

        while mi < magazine.len() {
            let ml = magazine[mi];
            letters.entry(ml).and_modify(|e| *e += 1).or_insert(1);
            mi += 1;

            if &ml == letter {
                return Some(mi);
            }
        }

        None
    }

    let mut letters: HashMap<_, _, _> =
        HashMap::with_capacity_and_hasher(26, BuildHasherDefault::<DefaultHasher>::default());
    let mut mi = 0;
    let magazine = magazine.as_bytes();

    for l in ransom_note.as_bytes() {
        if let Some(fi) = find(l, mi, magazine, &mut letters) {
            mi = fi;
            letters.entry(*l).and_modify(|e| *e -= 1);
        } else {
            return false;
        };
    }

    true
}

/// overcomplicated
pub fn can_construct_4(ransom_note: String, magazine: String) -> bool {
    let mut letters = vec![0; 26];
    let mut mi = 0;
    let magazine: Vec<_> = magazine
        .as_bytes()
        .iter()
        .map(|&x| x as usize - 97_usize)
        .collect();

    fn find(li: usize, mut mi: usize, magazine: &[usize], letters: &mut [u8]) -> Option<usize> {
        if letters[li] != 0 {
            return Some(mi);
        }

        while mi < magazine.len() {
            let ml = magazine[mi];
            letters[ml] += 1;
            mi += 1;

            if ml == li {
                return Some(mi);
            }
        }

        None
    }

    for li in ransom_note
        .as_bytes()
        .iter()
        .map(|&x| x as usize - 97_usize)
    {
        if let Some(fi) = find(li, mi, &magazine, &mut letters) {
            mi = fi;
            letters[li] -= 1;
        } else {
            return false;
        };
    }

    true
}

/// Simple, straightforward: count then check
pub fn can_construct_5(ransom_note: String, magazine: String) -> bool {
    let mut letters = [0_u8; 26];

    for b in magazine.bytes() {
        letters[(b - b'a') as usize] += 1;
    }

    for b in ransom_note.bytes() {
        let idx = (b - b'a') as usize;
        if letters[idx] == 0 {
            return false;
        }
        letters[idx] -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test_1() {
        run_test(can_construct_1);
    }

    #[test]
    fn test_2() {
        run_test(can_construct_2);
    }

    #[test]
    fn test_3() {
        run_test(can_construct_3);
    }

    #[test]
    fn test_4() {
        run_test(can_construct_4);
    }

    fn run_test(target: fn(String, String) -> bool) {
        vec![
            ("a", "b", false),
            ("aa", "ab", false),
            ("aa", "aab", true),
            ("aab", "baa", true),
            ("abc", "cba", true),
        ]
        .into_iter()
        .for_each(|(ransom_note, magazine, expected)| {
            let name = format!("{ransom_note} {magazine}");
            let actual = target(ransom_note.to_string(), magazine.to_string());
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
