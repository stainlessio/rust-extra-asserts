use std::ops::Sub;
use std::fmt::Debug;

/// Asserts if a floating point is within some epsilon.  This allows
/// you to compare calculations to make sure it's within some error factor
/// of accuracy.
///
/// This one should pass
/// ```
/// use extra_asserts::assert_approx_eq;
/// let x : f64 = 10.123456789;
/// let y : f64 = 10.123467890;
/// assert_approx_eq(x, y, &1e-4);
/// ```
///
/// This one should fail
/// ```should_panic
/// use extra_asserts::assert_approx_eq;
/// let x : f64 = 10.123456789;
/// let y : f64 = 10.123467890;
/// assert_approx_eq(x, y, &1e-10);
/// ```
pub fn assert_approx_eq<T>(l : T, r : T, epsilon : &T::Output)
where T : Sub + PartialOrd + Debug + Copy,
T::Output : Debug + PartialOrd {
    let diff = if l < r {
        r - l
    } else {
        l - r
    };
    assert!(diff < *epsilon, format!("{:?} != {:?}", l, r));
}
