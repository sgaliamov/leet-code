//! https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/
//!
//! Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
//!
//! Example 1:
//! Input: haystack = "sadbutsad", needle = "sad"
//! Output: 0
//! Explanation: "sad" occurs at index 0 and 6. The first occurrence is at index 0, so we return 0.
//!
//! Example 2:
//! Input: haystack = "leetcode", needle = "leeto"
//! Output: -1
//! Explanation: "leeto" did not occur in "leetcode", so we return -1.
//!
//! Constraints:
//! - 1 <= haystack.length, needle.length <= 10^4
//! - haystack and needle consist of only lowercase English characters.

pub fn str_str(haystack: String, needle: String) -> i32 {
	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;
	use leet_code::solution_tests;
	use spectral::prelude::*;

	solution_tests!(
		run_test:
		str_str,
	);

	fn run_test(target: fn(String, String) -> i32) {
		let cases = vec![
			("sadbutsad".to_string(), "sad".to_string(), 0), // Example 1: "sad" at index 0
			("leetcode".to_string(), "leeto".to_string(), -1), // Example 2: "leeto" not found
		];
		for (haystack, needle, expected) in cases {
			let name = format!("haystack='{}', needle='{}'", haystack, needle);
			let actual = target(haystack, needle);
			assert_that!(actual).named(&name).is_equal_to(expected);
		}
	}
}
