use array_ranges::*;

fn main() {
    println!("{:?}", range!(u8, 0u8, 255u8));
    let r = range_inclusive!(u8, 0u8, 255u8);
    println!("{r:?}");
}
