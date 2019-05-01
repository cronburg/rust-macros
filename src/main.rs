
#[macro_use]
mod utils;

mod bar;

fn main() {
    masked_macro!();
    let x = check_expect!(5, 5);
    bar::bar_me();
    println!("Unreachable: {}", x);
}

