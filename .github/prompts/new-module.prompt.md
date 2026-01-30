---
agent: agent
---

# Create LeetCode Module from Problem Description

Given a raw LeetCode problem description, create a properly organized test module in `tests/` directory.

## Module Structure Template

```rust
//! <problem-url>
//!
//! <Original formatted problem description>
//!
//! Constraints:
//! - <constraint 1>
//! - <constraint 2>
//! ...

// Helper types/structs if needed (e.g., ListNode, TreeNode)

pub fn <function_name>(<params>) -> <return_type> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn <function_name>_test() {
        let cases = vec![
            (<input>, <expected>), // Example 1: description
            (<input>, <expected>), // Example 2: description
            // ... more test cases
        ];

        cases.into_iter().for_each(|(<params>, expected)| {
            let name = format!("<descriptive name>");
            let actual = <function_name>(<params>);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }

    // Helper functions if needed (e.g., vec_to_list, build_tree)
}
```

## Key Rules

1. **File naming**: Use kebab-case matching the problem slug (e.g., `merge-two-sorted-lists.rs`)
2. **Module doc**: Include URL, brief description, and all constraints
3. **Function stub**: Leave implementation as `todo!()`
4. **Test cases**: Extract ALL examples from problem description
5. **Test format**: Use `Vec<(input, expected)>` with descriptive comments
6. **Assertions**: Use spectral's `assert_that!` with `.named()` for context
7. **Helper functions**: Add at bottom of test module if needed (e.g., for linked lists, trees)
8. **No implementation**: DO NOT write the solution - leave stub only
