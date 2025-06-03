/**
Evaluates to all the elements in `$start..$stop` as an array.

## Examples

```
# use array_ranges::range;
assert_eq!(range!(1u8,5u8), [1,2,3,4]);
```
*/
#[macro_export]
macro_rules! range {
    ($start: expr, $stop: expr) => {{
        let mut arr = [0; $stop.abs_diff($start) as usize];
        let mut current = $start;
        let mut i: usize = 0;
        while current < $stop {
            arr[i] = current;
            current += 1;
            i += 1;
        }
        arr
    }};
}

/**
Evaluates to all the elements in `$start..=$stop` as an array.

## Examples

```
# use array_ranges::range_inclusive;
assert_eq!(range_inclusive!(1u8,5u8), [1,2,3,4,5]);
```
*/
#[macro_export]
macro_rules! range_inclusive {
    ($start: expr, $stop: expr) => {{
        let mut arr = [0; $stop.abs_diff($start) as usize + 1];
        let mut current = $start;
        let mut i: usize = 0;
        while current <= $stop {
            arr[i] = current;
            current += 1;
            i += 1;
        }
        arr
    }};
}

#[must_use]
pub const fn u8s() -> [u8; u8::MAX as usize + 1] {
    type T = u8;
    range_inclusive!(T::MIN, T::MAX)
}

#[must_use]
pub const fn u16s() -> [u16; u16::MAX as usize + 1] {
    type T = u16;
    range_inclusive!(T::MIN, T::MAX)
}

#[must_use]
pub const fn u32s() -> [u32; u32::MAX as usize + 1] {
    type T = u32;
    range_inclusive!(T::MIN, T::MAX)
}

// pub const fn u64s() -> [u64; u64::MAX as usize + 1] {
//     type T = u64;
//     range_inclusive!(T::MIN, T::MAX)
// }
