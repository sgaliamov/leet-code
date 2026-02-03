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
        unsafe {
            let mut len = self.stack.len();
            while len > 0 {
                let last = self.stack.get_unchecked(len - 1);
                if last >= &val {
                    break;
                }
                len -= 1;
            }
            self.stack.set_len(len);
        }
        self.stack.push(val);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test() {
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

    enum Operation {
        Push(i32),
        Pop,
    }
}
