# LeetCode Performance Gauntlet - General Optimization Checklist

## Mission Statement
Beat the top 5% in both speed AND memory. Think like you're writing production systems code, not a coding interview answer.

---

## The Systematic Interrogation

### 1. Input Analysis
- **What's the input type?** `Vec<T>`, `&[T]`, `String`, `&str`?
  - Are you taking ownership when you could borrow?
  - Are there heap allocations in the input signature that can be avoided?
  - Can you work with slices instead of owned vectors?
- **What are the constraints?**
  - Min/max input size → pre-allocate buffers
  - Value ranges → can you use smaller integer types?
  - Guaranteed properties → what validation can you skip?

### 2. Memory Warfare
- **Allocations**: Count every `Vec::new()`, `String::from()`, `HashMap::new()`
  - Can you pre-allocate with `Vec::with_capacity(n)`?
  - Do you know the max size from constraints?
  - Are you cloning when you could reference?
- **Data structures**: Is your choice optimal?
  - `HashSet` vs `Vec` vs `[bool; N]` vs bit manipulation
  - `HashMap` vs `Vec` vs fixed-size array
  - `String` vs `Vec<u8>` vs `&str`
  - Consider `SmallVec`, `ArrayVec` for known-small sizes
- **String handling**:
  - `.to_string()`, `.clone()` → are they necessary?
  - Can you work with bytes instead of UTF-8?
  - String concatenation → use `Vec<u8>` or pre-allocated buffer?

### 3. Algorithm Selection
- **Complexity class**: What's optimal for this problem?
  - O(n) possible but you're doing O(n²)?
  - Sorting when you don't need to?
  - Multiple passes when one would work?
- **Data structure choice**:
  - Binary search vs hash table vs direct indexing
  - Stack vs recursion vs iteration
  - Queue vs two-pointer vs sliding window
- **Known patterns**:
  - Two pointers, sliding window, prefix sums, monotonic stack
  - DP, greedy, divide and conquer
  - Bit manipulation shortcuts

### 4. CPU Optimization
- **Branch prediction**:
  - Are your conditionals predictable?
  - Can you restructure to reduce branches?
  - Would a lookup table beat branching?
- **Loop efficiency**:
  - Are you iterating multiple times when once would work?
  - Can you eliminate bounds checks with `.get_unchecked()`?
  - Iterator chains vs raw loops — which is faster here?
- **Function calls**:
  - Should hot functions be `#[inline]` or `#[inline(always)]`?
  - Are you calling `.len()` repeatedly in a loop?
  - Method dispatch overhead vs direct implementation?
- **Parsing/conversion**:
  - `str.parse()` vs custom parser
  - `.chars()` vs `.bytes()` vs direct indexing
  - Can you avoid UTF-8 validation?

### 5. The Dirty Tricks Toolbox
- **Unsafe optimizations** (when you can prove safety):
  - `.get_unchecked()` instead of bounds checking
  - `std::hint::unreachable_unchecked()` for impossible branches
  - Raw pointer arithmetic if absolutely necessary
- **Constraint exploitation**:
  - Problem says "valid input" → skip all validation
  - Known ranges → use arrays instead of hash maps
  - "Unique elements" → skip duplicate checks
- **Bit manipulation**:
  - Bitmasks instead of `HashSet<usize>` for small ranges
  - Bit shifts instead of multiply/divide by powers of 2
  - XOR tricks for swapping, finding uniques
- **Integer optimization**:
  - Can you use `i8`/`u8` instead of `i32`/`u32`?
  - Would overflow wrapping (`wrapping_add`) be faster?
  - Is there a mathematical trick (modulo, XOR, etc.)?

### 6. Anti-Patterns to Hunt Down
- `unwrap()` in hot paths → prove unreachability or handle properly
- Unnecessary cloning/copying of data
- String operations in inner loops
- Hash map lookups when direct indexing works
- Sorting when partial ordering suffices
- Recursion when iteration is simpler
- Collecting iterators unnecessarily

### 7. The Scientific Method
1. **Measure baseline**: Benchmark current solution
2. **Hypothesis**: "X optimization should improve Y metric"
3. **Implement**: Single change at a time
4. **Benchmark**: Criterion for local, LeetCode for real
5. **Keep or discard**: Based on data, not intuition
6. **Document**: Comment why each optimization exists

---

## The Optimization Ladder

### Level 1: Algorithm (10-1000x gains)
Choose the right algorithm and data structures first. Nothing else matters if this is wrong.

### Level 2: Allocation (2-10x gains)
Eliminate unnecessary allocations, pre-size buffers, avoid clones.

### Level 3: Control Flow (1.2-3x gains)
Reduce branches, improve cache locality, restructure loops.

### Level 4: Micro-optimizations (1.05-1.5x gains)
Unsafe code, inline hints, bit tricks, custom parsers.

### Level 5: SIMD/Assembly (varies)
Only if you've exhausted everything else and have profiling data.

---

## Acceptance Criteria
- [ ] Top 10% speed (target: top 5%)
- [ ] Top 10% memory (target: top 5%)
- [ ] Every allocation justified
- [ ] Every `unsafe` block documented with safety invariants
- [ ] Criterion benchmarks created and compared
- [ ] Comments explain non-obvious optimizations

---

## Remember
> "Premature optimization is the root of all evil" — Knuth

But this is LeetCode. We optimize AFTER we have a working solution, not before. Correctness first, then speed.

Now profile, measure, and destroy the competition.

---

## Specific Prompt to Beat Top Solutions

Use this prompt after analyzing the problem:

```
I have a working solution for [PROBLEM_NAME] that currently runs in [X]ms / [Y]MB (beats [Z]%).

Constraints:
- [list all constraints from problem, especially: size bounds, value ranges, guaranteed properties]

My current approach:
- [describe algorithm and data structures]
- Time: O(?)
- Space: O(?)

ALGORITHM ANALYSIS:
1. What's the theoretical optimal complexity for this problem?
2. Is my algorithm already optimal, or is there a better approach?
3. Common patterns to consider: two-pointer, sliding window, monotonic stack, prefix sums, DP, greedy, binary search, bucket sort, counting sort, bit manipulation, mathematical formula

CONSTRAINT EXPLOITATION (THE GOLDMINE):
- Value range: [min, max] → Can I use fixed array [T; SIZE] instead of HashMap?
- Small alphabet (e.g., lowercase letters) → [u8; 26] array beats HashSet every time
- "Valid input guaranteed" → Remove ALL validation, bounds checks, Option unwrapping
- Known max size → Pre-allocate with Vec::with_capacity(N) or use [T; N] on stack
- Integer range fits in smaller type? → Use i8/u8 instead of i32 if intermediate calcs allow
- Unique elements? → Can skip duplicate tracking
- Sorted input? → Binary search, two-pointer, or direct indexing
- Limited operations? → Lookup table instead of match/if chains

INPUT SIGNATURE OPTIMIZATION:
- Currently taking Vec<String>? → Can I work with Vec<&str> or &[String]?
- Accepting owned types? → Can I just borrow (&[T], &str)?
- Do I need to clone the input? → LeetCode often allows mutation
- Can I work with bytes instead of strings? → .bytes() instead of .chars()
- Can I avoid collecting iterators? → Process on-the-fly

MEMORY WARFARE:
- Count allocations: every Vec::new(), HashMap::new(), String::from()
- Pre-allocate everything possible based on constraints
- Stack arrays [T; N] instead of heap Vec<T> when N is small/known
- Reuse buffers: pass &mut [T] instead of returning Vec<T>
- Avoid cloning: work with references, indices, or bit flags
- String building: Vec<u8> + String::from_utf8_unchecked vs String concatenation

CPU DIRTY TRICKS:
1. **Unsafe indexing**: .get_unchecked() when you KNOW bounds are valid
2. **Unreachable elimination**: std::hint::unreachable_unchecked() for impossible branches
3. **Pointer arithmetic**: Direct pointer ops instead of slice indexing (extreme cases)
4. **Bit manipulation**:
   - Use bitmask u64 instead of HashSet<usize> for small ranges (0..64)
   - XOR for swapping, finding single unique element
   - Bit shifts for multiply/divide by powers of 2
   - Count bits: .count_ones(), .trailing_zeros()
5. **Array indexing tricks**:
   - For ASCII letters: (byte - b'a') as usize gives 0..26
   - For digits: (byte - b'0') as usize gives 0..10
6. **Early exits**:
   - Check lengths first: if a.len() != b.len() { return false }
   - Fail fast on first mismatch
7. **Branchless code**:
   - min/max instead of if/else
   - Lookup tables instead of match
   - Arithmetic instead of conditionals
8. **Loop optimization**:
   - Hoist invariants out of loops
   - Cache .len() calls
   - Use .iter() instead of indexed access if not needed
   - Reverse loops to count down to 0 (sometimes faster)
9. **Integer optimization**:
   - wrapping_add/sub/mul when overflow is impossible
   - Signed vs unsigned: pick based on operations
   - Use i16/u16 for counters when range allows
10. **String/char tricks**:
    - Work with bytes: s.as_bytes()[i] instead of s.chars().nth(i)
    - ASCII assumptions: byte & 0x1F for case-insensitive (a-z only)
    - to_ascii_lowercase() vs to_lowercase() (ASCII is faster)
11. **Custom parsing**:
    - Write custom atoi for integers when you know format
    - Skip UTF-8 validation: from_utf8_unchecked when safe
12. **Function call overhead**:
    - #[inline] on small hot functions
    - #[inline(always)] for critical 1-2 line functions
    - Avoid function pointers in hot paths
13. **Iterator vs loop**:
    - Raw loops often faster for hot paths
    - Iterators good for chaining, but .collect() allocates
14. **Match optimization**:
    - Order cases by frequency (most common first)
    - Use lookup table for many cases
    - Eliminate unreachable arms with unreachable_unchecked

MATHEMATICAL SHORTCUTS:
- Can the problem be solved with a formula? (sum, XOR, bit tricks)
- Modular arithmetic instead of tracking full values
- Combinatorics: n choose k formulas
- Digit manipulation: sum of digits, digit frequency
- Two sum → hash map; three sum → two pointers after sort
- Cycle detection: Floyd's tortoise and hare
- Prefix sums for range queries
- Difference array for range updates

COMPARISON TRICKS:
- Comparing bytes is faster than comparing chars
- memcmp-style comparison for slices
- Avoid string comparison, compare byte slices
- For equality: check hash first, then full comparison

SYSTEMATIC CHECKLIST:
1. Is the algorithm optimal? (most important)
2. Pre-allocate all data structures based on constraints
3. Replace HashMap/HashSet with fixed arrays when possible
4. Work with bytes instead of chars/strings
5. Add early exits (length checks, first mismatch)
6. Use unsafe .get_unchecked() in proven-safe hot paths
7. Eliminate dead branches with unreachable_unchecked
8. Use bit manipulation where applicable
9. Consider custom integer parsing
10. Benchmark each change individually

QUESTIONS TO ASK:
- What constraint am I not exploiting?
- Where am I allocating unnecessarily?
- Can I replace this HashMap with an array?
- Can I work with bytes instead of UTF-8?
- Where can I add unsafe for proven-safe code?
- What validation can I skip due to "valid input"?
- Can I fail faster with early length/bounds checks?
- Is there a mathematical formula I'm missing?
- Am I doing multiple passes when one would work?
- Can I use bit manipulation instead of collections?

Don't write the code. Guide me with questions and concepts only. Make me think through each optimization.
```

This prompt is nuclear-level comprehensive for beating all LeetCode cases.
