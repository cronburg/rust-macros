
pub fn bar_me() {
    let x = check_expect!(3, 4);
    println!("Unreachable: {}", x);
}

