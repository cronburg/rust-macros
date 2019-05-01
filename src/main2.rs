
#[macro_use]
mod utils {
    #[macro_use]
    mod expect {
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
															
		macro_rules! masked_macro {
			() => {{
				println!("Only reachable from main!");
			}}
		}
    }
}

mod bar {
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
}

fn main() {
    masked_macro!();
    let x = check_expect!(5, 5);
    bar::bar_me();
    println!("Unreachable: {}", x);
}

