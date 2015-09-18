extern crate durado;

use std::time::Duration;
use durado::Durado;

fn main() {
  let d = Durado::new(
    Duration::from_secs(1)
  );
  println!("{}", d);
}
