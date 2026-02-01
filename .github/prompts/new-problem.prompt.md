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

pub fn <function_name_1>(<params>) -> <return_type> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        <function_name_1>,
    );

    fn run_test(target: fn(<params>) -> <return_type>) {
        vec![
            (<input>, <expected>), // Example 1: description
            (<input>, <expected>), // Example 2: description
            // ... more test cases
        ]
        .into_iter()
        .for_each(|(<params>, expected)| {
            let name = format!("<descriptive name>");
            let actual = target(<params>);
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
4. **Test structure**: Use `solution_tests!` macro with `run_test` function pattern
5. **Test cases**: Extract ALL examples from problem description
6. **Test format**: Use `Vec<(input, expected)>` with descriptive comments in `run_test` function
7. **Assertions**: Use spectral's `assert_that!` with `.named()` for context
8. **Helper functions**: Add at bottom of test module if needed (e.g., for linked lists, trees)
9. **No implementation**: DO NOT write the solution - leave stub only
