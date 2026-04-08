#![allow(clippy::cast_possible_truncation, unused_macros)]

#[macro_export]
macro_rules! into_panic {
    ($x: expr, $from: ty, $to: ty) => {{
        let _: $from = $x; // ensure `$x` is of type `$from`
        let ans = $x as $to;

        assert!($x == ans as $from, "value changed between casts");

        ans
    }};
}

/// Casts `$x` as a `$t`
///
/// ## Panics
/// If the cast would truncate the value, this panics.
#[macro_export]
macro_rules! to_usize_panic {
    ($x: expr, $t: ty) => {{ into_panic!($x, $t, usize) }};
}

/// Returns the unsigned version of a primitive integer type.
/// If the type is already unsigned, it is returned.
#[macro_export]
macro_rules! unsigned_compliment {
    (u8) => {
        u8
    };
    (u16) => {
        u16
    };
    (u32) => {
        u32
    };
    (u64) => {
        u64
    };
    (u128) => {
        u128
    };
    (usize) => {
        usize
    };

    (i8) => {
        u8
    };
    (i16) => {
        u16
    };
    (i32) => {
        u32
    };
    (i64) => {
        u64
    };
    (i128) => {
        u128
    };
    (isize) => {
        usize
    };
}

#[macro_export]
macro_rules! diff {
    ($x: expr, $y: expr,$t:tt) => {{
        // let ans: unsigned_compliment!($t) = ($x).abs_diff($y);
        // ans
        $x.abs_diff($y)
    }};
}

/// Used for implementing `range` and `range_inclusive`, since they are 99% the same.
#[macro_export]
macro_rules! generic_range {
    ($t:tt,$start:expr,$stop:expr,$inclusive:expr) => {{
        // Making sure arguments passed to the macro are valid types and match constraints.
        let _: $t = $start;
        let _: $t = $stop;
        let _: bool = $inclusive;
        assert!($start <= $stop);

        const LEN: usize =
            if $inclusive { 1 } else { 0 } + into_panic!(diff!($start, $stop, $t), $t, usize);

        let mut arr: [$t; LEN] = [0; LEN];
        let mut current: $t = $start;
        let mut i: usize = 0;
        while current < $stop {
            arr[i] = current;
            current += 1;
            i += 1;
        }
        if $inclusive {
            arr[i] = current;
        }
        arr
    }};
}

/**
The elements in `$start..$stop` as an array.

## Examples

```
# use array_ranges::*;
assert_eq!(range!(u8,1,5), [1,2,3,4]);
```
*/
#[macro_export]
macro_rules! range {
    ($t: tt, $start: expr, $stop: expr) => {{ generic_range!($t, $start, $stop, false) }};
}

/**
The elements in `$start..=$stop` as an array.

## Examples

```
# use array_ranges::*;
assert_eq!(range_inclusive!(u8,1,5), [1,2,3,4,5]);
```
*/
#[macro_export]
macro_rules! range_inclusive {
    ($t:tt,$start: expr, $stop: expr) => {{ generic_range!($t, $start, $stop, true) }};
}

#[macro_export]
macro_rules! fn_min_to_max {
    ($t: ty, $fn_name: ident) => {
        #[must_use]
        pub const fn $fn_name() -> [$t; to_usize_panic!(<$t>::MAX, $t) + 1] {
            range_inclusive!($t, <$t>::MIN, <$t>::MAX)
        }
    };
}

/// Applies a function to each element in an array, producing a new one.
/// This is mainly for const functions, since arrays can be mapped at runtime.
#[macro_export]
macro_rules! map {
    ($arr: expr, $func: expr, $len: expr) => {{
        debug_assert!($arr.len() == $len);
        const LEN: usize = $len;
        let mut new_arr: [_; LEN] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut i = 0;
        while i < LEN {
            new_arr[i] = $func($arr[i]);
            i += 1;
        }
        new_arr
    }};
}
