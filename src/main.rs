extern crate all_args_string;
extern crate string_stupidify;

use all_args_string::*;
use string_stupidify::*;

fn main() {
    let args = all_args_string();
    let args = args.alternate_case().unwrap();
    println!("{}", args);
}
