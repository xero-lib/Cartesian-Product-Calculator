mod math;
use math::binary_cartesian::cartesian_product;

fn main() {
    println!("{:?}", cartesian_product(&[1, 2, 3], &[4, 5, 6]));
}
