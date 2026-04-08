use array_ranges::*;
use std::mem::MaybeUninit;

const fn f(x: u8) -> i32 {
    (x as i32) * 2
}

#[allow(unsafe_code)]
fn main() {
    // println!("{:?}", range!(u8, 0u8, 255u8));
    // let r = range_inclusive!(u8, 0u8, 255u8);
    // println!("{r:?}");

    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let new_arr = map!(arr, f, 5);
    println!("{arr:?}");
    println!("{new_arr:?}");
}
