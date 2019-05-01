
#[macro_use]
mod utils;

mod bar;

fn main() {
    let x = check_expect!(5, 5);
    bar::bar_me();
    println!("Unreachable: {}", x);
}

