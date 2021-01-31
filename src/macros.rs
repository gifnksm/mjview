macro_rules! format_args_join {
    ($n:expr) => {
        format_args!("{}", $n)
    };
    ($n:expr, $($rest:expr),+ $(,)?) => {
        format_args!("{}{}", $n, format_args_join!($($rest),+))
    }
}

macro_rules! write_join {
    ($f:expr, $($rest:expr),+ $(,)?) => {
        write!($f, "{}", format_args_join!($($rest),+))
    };
}
