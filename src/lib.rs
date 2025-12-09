pub mod aocutils {

    #[macro_export]
    macro_rules! dbgp {

    () => {
        eprintln!("[{}:{}:{}]", file!(), line!(), column!())
    };
    ($val:expr $(,)?) => {

        match $val {
            tmp => {
                eprintln!("[{}:{}:{}] {} = {:?}",
                    file!(), line!(), column!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbgp!($val)),+,)
    };
}
    
}