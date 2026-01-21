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

/// Simple two-pass approach: count all magazine letters, then verify ransom note.
///
/// Time: O(n + m) where n = magazine.len(), m = ransom_note.len()
/// Space: O(1) - fixed 26-element array for lowercase English letters
///
/// Benchmarked: ~60ns (small), ~82ns (medium), ~86ns (worst case)
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

/// Unsafe optimized two-pass with early length check.
/// Uses unchecked array access and signed counter for single comparison on underflow.
///
/// Time: O(n + m) where n = magazine.len(), m = ransom_note.len()
/// Space: O(1) - fixed 26-element array
///
/// Benchmarked: ~60ns (small), ~79ns (medium), ~79ns (worst case)
/// Performance gain: 5-7% over safe version, not worth the unsafe trade-off for most cases
pub fn can_construct_6(ransom_note: String, magazine: String) -> bool {
    // Early exit: ransom note can't be longer than magazine
    if ransom_note.len() > magazine.len() {
        return false;
    }

    let mut letters = [0_i16; 26];

    // Count magazine letters
    for b in magazine.bytes() {
        unsafe {
            *letters.get_unchecked_mut((b - b'a') as usize) += 1;
        }
    }

    // Check ransom note and decrement
    for b in ransom_note.bytes() {
        let idx = (b - b'a') as usize;
        unsafe {
            let count = letters.get_unchecked_mut(idx);
            *count -= 1;
            if *count < 0 {
                return false;
            }
        }
    }

    true
}

/// Bitflag tracking with early termination when all required letters are found.
/// Sets a bit for each unique letter needed, clears it when satisfied, exits when flags == 0.
/// Optimal for cases where magazine is much longer than ransom_note.
///
/// Time: O(n + m) best case (early exit), O(n + m) worst case where n = magazine.len(), m = ransom_note.len()
/// Space: O(1) - fixed 26-element array + 32-bit flag
///
/// Benchmarked: ~61ns (small), ~70ns (medium), ~60ns (long magazine - 35% faster!)
/// Best overall choice: fastest when magazine >> ransom_note, competitive elsewhere
pub fn can_construct_7(ransom_note: String, magazine: String) -> bool {
    if ransom_note.len() > magazine.len() {
        return false;
    }

    let mut map = [0_i8; 26];
    let mut flags = 0_u32;

    // Count required letters and set flags
    for b in ransom_note.bytes() {
        let i = (b - b'a') as usize;
        unsafe {
            *map.get_unchecked_mut(i) += 1;
        }
        flags |= 1 << i;
    }

    // Scan magazine and clear flags when satisfied
    for b in magazine.bytes() {
        let i = (b - b'a') as usize;
        unsafe {
            let count = map.get_unchecked_mut(i);
            *count -= 1;
            if *count == 0 {
                flags ^= 1 << i; // Clear flag
                if flags == 0 {
                    return true; // Early exit!
                }
            }
        }
    }

    false
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

    #[test]
    fn test_5() {
        run_test(can_construct_5);
    }

    #[test]
    fn test_6() {
        run_test(can_construct_6);
    }

    #[test]
    fn test_7() {
        run_test(can_construct_7);
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
