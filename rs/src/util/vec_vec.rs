#[macro_export]
macro_rules! vec_vec {
    ($($e:expr),*) => {vec![$($e.to_vec()), *]};
    ($($e:expr,)*) => {vec![$($e.to_vec()), *]};
}