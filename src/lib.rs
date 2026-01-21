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
