
macro_rules! masked_macro {
    ( $e1:expr ) => {{
        $e1
    }}
}

pub fn bar_me() {
    println!("I'm ignoring you src/utils/expect.rs:masked_macro ! {}"
        , masked_macro!(7)
        );
    let x = check_expect!(3, 4);
    println!("Unreachable: {}", x);
}

