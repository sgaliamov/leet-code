#![allow(clippy::new_without_default)]

/// Decreasing monotonic stack
pub struct DecreasingStack<T> {
    stack: Vec<T>,
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

    #[inline]
    pub fn push(&mut self, val: T) {
        while let Some(last) = self.stack.last()
            && last < &val
        {
            self.stack.pop();
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
