extern crate durado;

use std::time::Duration;
use durado::Durado;

fn main() {
    let d = Durado::new(Duration::new(0, 1050000));
    println!("{}", d);
}
