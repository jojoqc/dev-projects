extern crate orc;

use orc::Dot;
use orc::Swap;

fn main() {
    let x = vec![1.0, -2.0, 3.0, 4.0];
    let y = [1.0, 1.0, 1.0, 1.0, 7.0];

    let d = Dot::dot(&x, &y[..x.len()]);
    assert_eq!(d, 6.0);
    println!("{}", d);
}
