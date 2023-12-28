//! The `interval` module provides a representation of numeric intervals with a minimum and maximum value.

/// Represents a numeric interval with a minimum and maximum value.
#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    /// Creates a new empty interval
    /// (with +∞ as the minimum and -∞ as the maximum).
    pub fn new_empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    /// Creates a new interval with the specified minimum and maximum values.
    pub fn new(_min: f64, _max: f64) -> Self {
        Self {
            min: _min,
            max: _max,
        }
    }

    /// Checks if the interval contains a specific value.
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Checks if the interval surrounds a specific value (exclusive).
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

/// Represents an empty interval with +∞ as the minimum and -∞ as the maximum.
pub static EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};

/// Represents a universe interval with -∞ as the minimum and +∞ as the maximum.
pub static UNIVERSE: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};
