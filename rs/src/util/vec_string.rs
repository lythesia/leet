#[macro_export]
macro_rules! vec_string {
    ($($e:expr),*) => {vec![$($e.to_owned()), *]};
    ($($e:expr,)*) => {vec![$($e.to_owned()), *]};
}

#[macro_export]
macro_rules! vec_vec_string {
    [ $( [ $($e:expr),* ] ),* ] => {
        vec![
            $(
                vec![$($e.to_owned()),*],
            )*
        ]
    };
}