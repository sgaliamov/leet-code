# LeetCode

This repository contains solutions to coding problems from LeetCode, implemented in Rust. \
The goal is to practice problem-solving skills, improve Rust proficiency, and maintain high code quality.

## Useful tricks

1. Use `String::as_bytes` to efficiently access characters in a string.
1. Use bitwise flags for tracking multiple boolean states:
   ``` rust
   let mut flags = 0_u32;
   flags |= 1 << i; // Set flag i
   flags ^= 1 << i; // Clear flag i
   ```
1. Use array indexing instead of HashMap for small fixed-size data sets to improve performance:
    ``` rust
    let mut counts = [0; 26]; // For lowercase letters 'a' to 'z'
    for &b in s.into_bytes() {
         counts[(b - b'a') as usize] += 1;
    }
    ```
1. Using linear search in small arrays is efficient enough and simple. No need for complex data structures.
1. Use bitwise operations to count unique characters in a string:
   ``` rust
   let mut mask = 0_u32;
   for &b in s.into_bytes() {
       mask |= 1 << (b - b'a');
   }
   let unique_count = mask.count_ones();
   ```
1. Use faster hasher:
   ``` rust
    let mut map: HashMap<_, _, _> =
        HashMap::with_capacity_and_hasher(26, BuildHasherDefault::<DefaultHasher>::default());
    ```
