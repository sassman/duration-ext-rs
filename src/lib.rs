//! Ergonomic extension trait for creating `Duration` from numeric literals.
//!
//! This crate provides the [`DurationExt`] trait which extends numeric types
//! with convenient methods for creating [`std::time::Duration`] values.
//!
//! # Examples
//!
//! ```
//! use duration_ext::DurationExt;
//! use std::time::Duration;
//!
//! // Integer literals
//! assert_eq!(5.seconds(), Duration::from_secs(5));
//! assert_eq!(500.millis(), Duration::from_millis(500));
//! assert_eq!(100.micros(), Duration::from_micros(100));
//! assert_eq!(1000.nanos(), Duration::from_nanos(1000));
//! assert_eq!(2.minutes(), Duration::from_secs(120));
//! assert_eq!(1.hours(), Duration::from_secs(3600));
//!
//! // Fractional values (f64)
//! assert_eq!(1.5.seconds(), Duration::from_millis(1500));
//! assert_eq!(0.5.minutes(), Duration::from_secs(30));
//! ```
//!
//! # Supported Types
//!
//! The trait is implemented for:
//! - `u64`, `u32`, `usize` - unsigned integers
//! - `i32` - signed integers (negative values clamp to zero)
//! - `f64`, `f32` - floating point (negative values clamp to zero)

use std::time::Duration;

/// Extension trait for creating [`Duration`] from numeric values.
///
/// This trait provides ergonomic methods for creating durations from
/// numeric literals, making code more readable than using `Duration::from_*`.
pub trait DurationExt {
    /// Create a `Duration` representing this many nanoseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use duration_ext::DurationExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(1000.nanos(), Duration::from_nanos(1000));
    /// assert_eq!(1.5.nanos(), Duration::from_nanos(1)); // fractional truncates
    /// ```
    fn nanos(self) -> Duration;

    /// Create a `Duration` representing this many microseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use duration_ext::DurationExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(100.micros(), Duration::from_micros(100));
    /// assert_eq!(1.5.micros(), Duration::from_nanos(1500));
    /// ```
    fn micros(self) -> Duration;

    /// Create a `Duration` representing this many milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use duration_ext::DurationExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(500.millis(), Duration::from_millis(500));
    /// assert_eq!(1.5.millis(), Duration::from_micros(1500));
    /// ```
    fn millis(self) -> Duration;

    /// Create a `Duration` representing this many seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use duration_ext::DurationExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(5.seconds(), Duration::from_secs(5));
    /// assert_eq!(1.5.seconds(), Duration::from_millis(1500));
    /// ```
    fn seconds(self) -> Duration;

    /// Create a `Duration` representing this many minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use duration_ext::DurationExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(2.minutes(), Duration::from_secs(120));
    /// assert_eq!(1.5.minutes(), Duration::from_secs(90));
    /// ```
    fn minutes(self) -> Duration;

    /// Create a `Duration` representing this many hours.
    ///
    /// # Examples
    ///
    /// ```
    /// use duration_ext::DurationExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(1.hours(), Duration::from_secs(3600));
    /// assert_eq!(1.5.hours(), Duration::from_secs(5400));
    /// ```
    fn hours(self) -> Duration;
}

impl DurationExt for u64 {
    fn nanos(self) -> Duration {
        Duration::from_nanos(self)
    }

    fn micros(self) -> Duration {
        Duration::from_micros(self)
    }

    fn millis(self) -> Duration {
        Duration::from_millis(self)
    }

    fn seconds(self) -> Duration {
        Duration::from_secs(self)
    }

    fn minutes(self) -> Duration {
        Duration::from_secs(self * 60)
    }

    fn hours(self) -> Duration {
        Duration::from_secs(self * 3600)
    }
}

impl DurationExt for u32 {
    fn nanos(self) -> Duration {
        Duration::from_nanos(self as u64)
    }

    fn micros(self) -> Duration {
        Duration::from_micros(self as u64)
    }

    fn millis(self) -> Duration {
        Duration::from_millis(self as u64)
    }

    fn seconds(self) -> Duration {
        Duration::from_secs(self as u64)
    }

    fn minutes(self) -> Duration {
        Duration::from_secs(self as u64 * 60)
    }

    fn hours(self) -> Duration {
        Duration::from_secs(self as u64 * 3600)
    }
}

impl DurationExt for usize {
    fn nanos(self) -> Duration {
        Duration::from_nanos(self as u64)
    }

    fn micros(self) -> Duration {
        Duration::from_micros(self as u64)
    }

    fn millis(self) -> Duration {
        Duration::from_millis(self as u64)
    }

    fn seconds(self) -> Duration {
        Duration::from_secs(self as u64)
    }

    fn minutes(self) -> Duration {
        Duration::from_secs(self as u64 * 60)
    }

    fn hours(self) -> Duration {
        Duration::from_secs(self as u64 * 3600)
    }
}

impl DurationExt for i32 {
    fn nanos(self) -> Duration {
        Duration::from_nanos(self.max(0) as u64)
    }

    fn micros(self) -> Duration {
        Duration::from_micros(self.max(0) as u64)
    }

    fn millis(self) -> Duration {
        Duration::from_millis(self.max(0) as u64)
    }

    fn seconds(self) -> Duration {
        Duration::from_secs(self.max(0) as u64)
    }

    fn minutes(self) -> Duration {
        Duration::from_secs(self.max(0) as u64 * 60)
    }

    fn hours(self) -> Duration {
        Duration::from_secs(self.max(0) as u64 * 3600)
    }
}

impl DurationExt for f64 {
    fn nanos(self) -> Duration {
        Duration::from_nanos(self.max(0.0) as u64)
    }

    fn micros(self) -> Duration {
        Duration::from_secs_f64(self.max(0.0) / 1_000_000.0)
    }

    fn millis(self) -> Duration {
        Duration::from_secs_f64(self.max(0.0) / 1000.0)
    }

    fn seconds(self) -> Duration {
        Duration::from_secs_f64(self.max(0.0))
    }

    fn minutes(self) -> Duration {
        Duration::from_secs_f64(self.max(0.0) * 60.0)
    }

    fn hours(self) -> Duration {
        Duration::from_secs_f64(self.max(0.0) * 3600.0)
    }
}

impl DurationExt for f32 {
    fn nanos(self) -> Duration {
        Duration::from_nanos(self.max(0.0) as u64)
    }

    fn micros(self) -> Duration {
        Duration::from_secs_f32(self.max(0.0) / 1_000_000.0)
    }

    fn millis(self) -> Duration {
        Duration::from_secs_f32(self.max(0.0) / 1000.0)
    }

    fn seconds(self) -> Duration {
        Duration::from_secs_f32(self.max(0.0))
    }

    fn minutes(self) -> Duration {
        Duration::from_secs_f32(self.max(0.0) * 60.0)
    }

    fn hours(self) -> Duration {
        Duration::from_secs_f32(self.max(0.0) * 3600.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== u64 tests ==========

    #[test]
    fn test_u64_nanos() {
        assert_eq!(1000u64.nanos(), Duration::from_nanos(1000));
    }

    #[test]
    fn test_u64_micros() {
        assert_eq!(100u64.micros(), Duration::from_micros(100));
    }

    #[test]
    fn test_u64_millis() {
        assert_eq!(500u64.millis(), Duration::from_millis(500));
    }

    #[test]
    fn test_u64_seconds() {
        assert_eq!(2u64.seconds(), Duration::from_secs(2));
    }

    #[test]
    fn test_u64_minutes() {
        assert_eq!(2u64.minutes(), Duration::from_secs(120));
    }

    #[test]
    fn test_u64_hours() {
        assert_eq!(2u64.hours(), Duration::from_secs(7200));
    }

    // ========== u32 tests ==========

    #[test]
    fn test_u32_seconds() {
        assert_eq!(3u32.seconds(), Duration::from_secs(3));
    }

    #[test]
    fn test_u32_minutes() {
        assert_eq!(5u32.minutes(), Duration::from_secs(300));
    }

    #[test]
    fn test_u32_hours() {
        assert_eq!(1u32.hours(), Duration::from_secs(3600));
    }

    // ========== usize tests ==========

    #[test]
    fn test_usize_seconds() {
        assert_eq!(5usize.seconds(), Duration::from_secs(5));
    }

    #[test]
    fn test_usize_minutes() {
        assert_eq!(10usize.minutes(), Duration::from_secs(600));
    }

    // ========== i32 tests ==========

    #[test]
    fn test_i32_seconds() {
        assert_eq!(4i32.seconds(), Duration::from_secs(4));
    }

    #[test]
    fn test_i32_negative_clamps_to_zero() {
        assert_eq!((-1i32).seconds(), Duration::from_secs(0));
        assert_eq!((-100i32).millis(), Duration::from_millis(0));
        assert_eq!((-5i32).minutes(), Duration::from_secs(0));
    }

    // ========== f64 tests ==========

    #[test]
    fn test_f64_seconds() {
        assert_eq!(1.5f64.seconds(), Duration::from_secs_f64(1.5));
    }

    #[test]
    fn test_f64_fractional_seconds_equals_millis() {
        // The specific test requested: 1.5.seconds() == 1500ms
        assert_eq!(1.5.seconds(), Duration::from_millis(1500));
    }

    #[test]
    fn test_f64_millis() {
        assert_eq!(1.5f64.millis(), Duration::from_micros(1500));
    }

    #[test]
    fn test_f64_micros() {
        assert_eq!(1.5f64.micros(), Duration::from_nanos(1500));
    }

    #[test]
    fn test_f64_minutes() {
        assert_eq!(1.5f64.minutes(), Duration::from_secs(90));
        assert_eq!(0.5f64.minutes(), Duration::from_secs(30));
    }

    #[test]
    fn test_f64_hours() {
        assert_eq!(1.5f64.hours(), Duration::from_secs(5400));
        assert_eq!(0.5f64.hours(), Duration::from_secs(1800));
    }

    #[test]
    fn test_f64_negative_clamps_to_zero() {
        assert_eq!((-1.5f64).seconds(), Duration::from_secs(0));
        assert_eq!((-100.0f64).millis(), Duration::from_millis(0));
    }

    // ========== f32 tests ==========

    #[test]
    fn test_f32_seconds() {
        assert_eq!(1.5f32.seconds(), Duration::from_secs_f32(1.5));
    }

    #[test]
    fn test_f32_fractional_seconds_equals_millis() {
        // Same test for f32
        assert_eq!(1.5f32.seconds(), Duration::from_millis(1500));
    }

    #[test]
    fn test_f32_minutes() {
        assert_eq!(2.5f32.minutes(), Duration::from_secs(150));
    }

    // ========== Edge cases ==========

    #[test]
    fn test_zero_duration() {
        assert_eq!(0u64.seconds(), Duration::ZERO);
        assert_eq!(0.0f64.seconds(), Duration::ZERO);
        assert_eq!(0u32.minutes(), Duration::ZERO);
    }

    #[test]
    fn test_large_values() {
        // 24 hours in seconds
        assert_eq!(24u64.hours(), Duration::from_secs(86400));
        // 1 week in hours
        assert_eq!(168u64.hours(), Duration::from_secs(604800));
    }

    #[test]
    fn test_small_fractional_values() {
        // Very small durations
        assert_eq!(0.001f64.seconds(), Duration::from_millis(1));
        assert_eq!(0.000001f64.seconds(), Duration::from_micros(1));
    }

    // ========== Practical usage examples ==========

    #[test]
    fn test_common_animation_durations() {
        // Common animation timing values
        assert_eq!(0.3.seconds(), Duration::from_millis(300));
        assert_eq!(0.25.seconds(), Duration::from_millis(250));
        assert_eq!(0.5.seconds(), Duration::from_millis(500));
        assert_eq!(1.0.seconds(), Duration::from_secs(1));
    }

    #[test]
    fn test_timeout_durations() {
        // Common timeout values
        assert_eq!(30.seconds(), Duration::from_secs(30));
        assert_eq!(5.minutes(), Duration::from_secs(300));
        assert_eq!(1.hours(), Duration::from_secs(3600));
    }
}
