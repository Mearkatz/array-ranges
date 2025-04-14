/// Produces a range as an array
///
/// # Examples
///
/// ```
/// # use array_ranges::range;
/// assert_eq!(range!(1u8,6u8), [1,2,3,4,5]);
/// ```
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

/// Produces a range as an array
///
/// # Examples
///
/// ```
/// # use array_ranges::range_inclusive;
/// assert_eq!(range_inclusive!(1u8,5u8), [1,2,3,4,5]);
/// ```
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
