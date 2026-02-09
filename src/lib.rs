pub mod monotonic_stack;
mod tree;

pub use monotonic_stack::*;
pub use tree::*;

#[macro_export]
macro_rules! solution_tests {
    ($run_test:ident: $($func:ident),+ $(,)?) => {
        $(
            paste::paste! {
                #[test]
                fn [<test_ $func>]() {
                    $run_test($func);
                }
            }
        )+
    };
}
