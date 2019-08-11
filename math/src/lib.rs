//! Super-fast mathematical functions for WalkDarkly
//!
//! WalkDarkly uses the angle of incidence of the sun in your location
//! plus the height of nearby buildings to determine where the shade is
//! right now. There's a fair bit of math involved, but nothing too far
//! outside of high school-level geometry and trigonometry.
//!

use std::f64;

/// Calculate the length of a shadow
///
/// In a 2D plane, given a wall and the angle of incidence of the sun,
/// we need to find out what horizontal distance along the ground is
/// covered in shade. For bonus points, we should only return the
/// horizontal distance where the shade extends up to a reasonable
/// height to cover most people, maybe 6' i.e. 12 * 6 = 72 inches i.e.
/// 72 * 2.54 = ~183 centimeters.
///
/// The problem looks like this:
///
/// ```ignore
///     (sun)
///          \
///            \
///              \
///              _
///             | | \
///             | |   \
///             | |     \
///             | |       \
///             | |         \
///             | |h1         \
///             | |             \
///             | |              |\
///             | |            h2|  \
///             | |              |    \
///             | |_______x______|_x'__Θ\
///                |_________x''_________|
/// ```
///
/// Where Θ is the angle of incidence of the sun (assuming the building
/// is horizontal), h1 is the height of the building, h2 is the height
/// of the person, and x is the "safe space" where there is enough shade
/// for the person to be in full shade. x' is the shaded area that would
/// not be considered full shade, and x'' is the total horizontal shade
/// coverage.
///
/// The angle of incidence of the sun with the building is equal to the
/// angle of incidence of the sun with the ground (Θ), so if we know the
/// height of the building, we can determine x'' via tan(Θ) = h1 / x'' ->
/// x'' = h1 / tan(Θ).
///
/// From there, x is just the difference of x'' - x'. x' can be derived
/// just like x'': x' = h2 / tan(Θ).
///
/// So, x = (h1 / tan(Θ)) - (h2 / tan(Θ)) = (1/tan(Θ)) * (h1 - h2)
///
/// The units of height don't matter as long as they are the same.
/// The angle should be in radians.
///
pub fn fully_shaded_len<H1, H2, Theta>(h1: H1, h2: H2, theta: Theta) -> f64
where
    H1: Into<f64>,
    H2: Into<f64>,
    Theta: Into<f64>,
{
    let tan = theta.into().tan();
    if approx_equal(tan, 0.0) {
        f64::INFINITY
    } else {
        (1.0 / tan).abs() * (h1.into() - h2.into())
    }
}


/// Get the shaded area of a rectangular structure of a given height
///
/// This just a simple extension of the above to get the rectangular
/// shaded area for a structure of a given length.
pub fn fully_shaded_area<Len, H1, H2, Theta>(len: Len, h1: H1, h2: H2, theta: Theta) -> f64
where
    Len: Into<f64>,
    H1: Into<f64>,
    H2: Into<f64>,
    Theta: Into<f64>,
{
    len.into() * fully_shaded_len(h1, h2, theta)
}


/// Convert degrees to radians
pub fn deg_to_rad<D: Into<f64>>(deg: D) -> f64 {
    deg.into().to_radians()
}


fn approx_equal<One, Two>(one: One, two: Two) -> bool
where
    One: Into<f64>,
    Two: Into<f64>,
{
    let abs_diff = (one.into() - two.into()).abs();
    abs_diff < 1e-10
}


#[cfg(test)]
mod tests {
    //! Tests for the shadow functions
    //!
    //! Note that since we're dealing with floating point math, our equality
    //! comparisons are generally just asserting that the absolute difference
    //! between numbers is < 1e-10
    use super::*;
    use std::f64::consts::PI;

    fn assert_approx<One, Two>(one: One, two: Two)
    where
        One: Into<f64>,
        Two: Into<f64>,
    {
        assert!(approx_equal(one, two));
    }

    #[test]
    fn shadow_len_45() {
        // tan(π/4) = tan(45°) = 1, so this is just h1 - h2
        let shadow_len = fully_shaded_len(1000.0, 100.0, PI / 4.0);
        assert_approx(shadow_len, 900);
    }

    #[test]
    fn shadow_area_45() {
        // tan(π/4) = tan(45°) = 1, so this is just h1 - h2
        let area = fully_shaded_area(10, 1000, 100, PI / 4.0);
        assert_approx(area, 9000);
    }

    #[test]
    fn shadow_len_0() {
        // tan(0) = tan(0°) = 0, so we have infinite shadow
        let shadow_len = fully_shaded_len(1000, 100, 0);
        assert_eq!(shadow_len, f64::INFINITY);
    }

    #[test]
    fn shadow_area_0() {
        // tan(0) = tan(0°) = 0, so we have infinite shadow
        let area = fully_shaded_area(10, 1000, 100, 0);
        assert_eq!(area, f64::INFINITY);
    }

    #[test]
    fn shadow_len_180() {
        // tan(π) = tan(180°) = 0, so we have infinite shadow
        let shadow_len = fully_shaded_len(1000, 100, PI);
        assert_eq!(shadow_len, f64::INFINITY);
    }

    #[test]
    fn shadow_area_180() {
        // tan(π) = tan(180°) = 0, so we have infinite shadow
        let area = fully_shaded_area(10, 1000, 100, PI);
        assert_eq!(area, f64::INFINITY);
    }

    #[test]
    fn shadow_len_90() {
        // tan(π/2) = tan(90) = ∞, so this is noon, no shadow
        let shadow_len = fully_shaded_len(1000.0, 100.0, PI / 2.0);
        assert_approx(shadow_len, 0);
    }

    #[test]
    fn shadow_area_90() {
        // tan(π/2) = tan(90) = ∞, so this is noon, no shadow
        let area = fully_shaded_area(10, 1000.0, 100.0, PI / 2.0);
        assert_approx(area, 0);
    }

    #[test]
    fn shadow_len_ints() {
        // tan(π/4) = tan(45°) = 1, so this is just h1 - h2
        let shadow_len = fully_shaded_len(1000, 100, PI / 4.0);
        assert_approx(shadow_len, 900);
    }

}