#![allow(clippy::new_without_default)]

/// Decreasing monotonic stack (optimized with unsafe)
///
/// Benchmark results (criterion):
/// - Typical case (n=100):  160.53 ns vs 201.80 ns (20% faster)
/// - Typical case (n=1000): 1.325 µs vs 1.425 µs (7% faster)
/// - Worst case: ~equal (no popping work = no optimization gain)
pub struct DecreasingStack<T> {
    pub stack: Vec<T>,
}

impl<T: PartialOrd> DecreasingStack<T> {
    #[inline]
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            stack: Vec::with_capacity(capacity),
        }
    }

    /// Push value onto stack, maintaining decreasing order.
    ///
    /// Pops all elements smaller than `val` before pushing.
    ///
    /// Time: O(n) amortized per operation (each element pushed/popped once)
    /// Space: O(1) auxiliary
    #[inline]
    pub fn push(&mut self, val: T) {
        push_decreasing(&mut self.stack, val);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

/// Push value onto stack, maintaining decreasing order.
///
/// Pops all elements smaller than `val` before pushing.
///
/// Time: O(n) amortized per operation (each element pushed/popped once)
/// Space: O(1) auxiliary
#[inline]
pub fn push_decreasing<T: PartialOrd>(stack: &mut Vec<T>, val: T) {
    unsafe {
        let mut len = stack.len();
        while len > 0 {
            let last = stack.get_unchecked(len - 1);
            if last >= &val {
                break;
            }
            len -= 1;
        }
        stack.set_len(len);
    }
    stack.push(val);
}

#[inline]
pub fn push_increasing<T: PartialOrd>(stack: &mut Vec<T>, val: T) {
    unsafe {
        let mut len = stack.len();
        while len > 0 {
            let last = stack.get_unchecked(len - 1);
            if last <= &val {
                break;
            }
            len -= 1;
        }
        stack.set_len(len);
    }
    stack.push(val);
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test_push_decreasing() {
        use Operation::*;
        let mut stack = DecreasingStack::new();

        let operations = vec![
            (Push(5), Some(vec![5])),
            (Push(3), Some(vec![5, 3])),
            (Push(7), Some(vec![7])),
            (Push(2), Some(vec![7, 2])),
            (Pop, Some(vec![7])),
            (Pop, Some(vec![])),
            (Pop, None),
        ];

        for (operation, expected_stack) in operations {
            match operation {
                Operation::Push(val) => {
                    stack.push(val);
                    if let Some(expected) = expected_stack {
                        assert_that!(&stack.stack).is_equal_to(&expected);
                    }
                }
                Operation::Pop => {
                    let result = stack.pop();
                    if let Some(expected) = expected_stack {
                        assert_that!(&stack.stack).is_equal_to(&expected);
                    } else {
                        assert_that!(result).is_none();
                    }
                }
            }
        }
    }

    #[test]
    fn test_push_increasing() {
        let cases = vec![
            (3, vec![3]),       // initial push
            (5, vec![3, 5]),    // push larger, stack grows
            (2, vec![2]),       // push smaller, pops all
            (7, vec![2, 7]),    // push larger again
            (4, vec![2, 4]),    // push smaller, pops 7
            (8, vec![2, 4, 8]), // push larger, stack grows
            (1, vec![1]),       // push smallest, clears stack
        ];

        let mut stack: Vec<i32> = Vec::new();
        for (input, expected) in cases {
            push_increasing(&mut stack, input);
            assert_that!(&stack).is_equal_to(&expected);
        }
    }

    enum Operation {
        Push(i32),
        Pop,
    }
}
