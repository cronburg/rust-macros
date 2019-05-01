
#[macro_export]
macro_rules! check_expect {
    ( $e1:expr, $e2:expr ) => {{
        let __e1 = $e1;
        let __e2 = $e2;
        if __e1 != __e2 {
            eprintln!("{}:{}] Expected {} but got {}."
                , file!(), line!()
                , __e2, __e1);
            std::process::exit(1);
        }
        __e1
    }}
}

