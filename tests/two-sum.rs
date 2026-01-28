// 100/82/2.28
pub fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use itertools::Itertools;
    let sorted = nums.iter().sorted_unstable().collect_vec();

    for i in 0..nums.len() {
        let t = target - nums[i];

        if let Ok(r) = sorted.binary_search(&&t) {
            let v = sorted[r];

            if let Some((p, _)) = nums
                .iter()
                .enumerate()
                .find_position(|&(p, x)| x == v && p != i)
            {
                let i = i as i32;
                let p = p as i32;
                return if i > p { vec![p, i] } else { vec![i, p] };
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        two_sum_1,
    );

    fn run_test(target_fn: fn(Vec<i32>, i32) -> Vec<i32>) {
        vec![
            (vec![3, 2, 4], 6, vec![1, 2]), //
        ]
        .into_iter()
        .for_each(|(nums, target, expected)| {
            let name = format!("{nums:?} {target}");
            let actual = target_fn(nums, target);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
