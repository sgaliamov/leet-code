# LeetCode Optimization Prompt

Target: top 5% in speed and memory.

```
Problem: [NAME]
Current: [X]ms / [Y]MB (beats [Z]%)
Constraints: [list bounds, ranges, guarantees]

ALGORITHM:
- Optimal complexity?
- Better approach exists?
- Patterns: two-pointer, sliding window, monotonic stack, prefix sums, DP, greedy, binary search, bit manipulation

CONSTRAINTS:
- Value range → fixed array [T; SIZE] instead of HashMap?
- Small alphabet → [u8; 26] beats HashSet
- "Valid input" → skip validation
- Known max size → Vec::with_capacity(N) or [T; N]
- Smaller types? i8/u8 vs i32?

MEMORY:
- Pre-allocate based on constraints
- Stack arrays [T; N] vs heap Vec<T>
- Work with bytes vs strings
- Avoid cloning

SPEED:
- .get_unchecked() when bounds proven
- unreachable_unchecked() for impossible branches
- Bitmask u64 vs HashSet for small ranges
- Byte indexing: (byte - b'a') as usize
- Early exits: length checks, fail fast
- Cache .len() calls
- #[inline] hot functions
- BuildHasherDefault::<DefaultHasher>::default() for faster hashing

Guide with questions, not code.
```
