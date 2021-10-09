use std::convert::TryInto;
use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Close, Nexta, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Windowed exponential moving average (WEMA).
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)

#[doc(alias = "WEMA")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct WindowedExponentialMovingAverage {
    period: usize,
    index: usize,
    count: usize,
    wsum: f64,
    k: f64,
    deque: Box<[f64]>,
}

impl WindowedExponentialMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                index: 0,
                count: 0,
                wsum: 0.0,
                k: 2.0 / (period + 1) as f64,
                deque: vec![0.0; period].into_boxed_slice(),
            }),
        }
    }
}

impl Period for WindowedExponentialMovingAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Nexta<f64> for WindowedExponentialMovingAverage {
    type Output = f64;

    fn nexta(&mut self, input: f64) -> Self::Output {
        let old_val = self.deque[self.index];
        self.deque[self.index] = input;

        self.index = if self.index + 1 < self.period {
            self.index + 1
        } else {
            0
        };

        if self.count == 0 {
            self.count += 1;
            self.wsum = input;
            return input;
        }

        let mut remove_old_val = true;
        if self.count < self.period {
            remove_old_val = false;
            self.count += 1;
        }

        self.wsum = input * self.k + self.wsum * (1.0 - self.k);

        if remove_old_val {
            // we want to pretend that the oldest(sill existing) value in our deque
            // is the first one that we ever had.

            let factor = (1.0 - self.k).powi((self.period).try_into().unwrap());

            // first we compute the weighted value of our oldest value, as it would
            // be at the position the one we're dropping
            let weighted_oldest_valid_val = self.deque[self.index] * factor;

            // now we compute the weighted value of the value we don't want to
            // take into accout anymore.
            let weighted_dropped_val = old_val * factor;

            // now we want to pretend that value we're dropping didn't exsit. We
            // can do this by "altering" the old value to match the oldest one we
            // want to keep. If the old value is equal to the oldest one we want
            // to keep, it doesn't have any effect at all.

            self.wsum -= weighted_dropped_val;
            // now wsum is as if the dropped value where zero.

            self.wsum += weighted_oldest_valid_val;
            // now wsum is as if the dropped value where equal to the oldest one we want to keep
        }

        self.wsum
    }
}

impl<T: Close> Nexta<&T> for WindowedExponentialMovingAverage {
    type Output = f64;

    fn nexta(&mut self, input: &T) -> Self::Output {
        self.nexta(input.close())
    }
}

impl Reset for WindowedExponentialMovingAverage {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.wsum = 0.0;
        for i in 0..self.period {
            self.deque[i] = 0.0;
        }
    }
}

impl Default for WindowedExponentialMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for WindowedExponentialMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WEMA({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::ExponentialMovingAverage;
    use crate::test_helper::*;

    #[test]
    fn check_against_ema() {
        let numbers = [
            10.0f64, 9.4, 23.1, 0.0, 0.0, 91.837261, 0.0, 0.5, -1.5, 25.1, -84.1235, 101.0, 78.0,
            1.0, 6.232,
        ];
        for period in 1..9 {
            let mut wema = WindowedExponentialMovingAverage::new(period).unwrap();
            for &n in &numbers[..period - 1] {
                wema.nexta(n);
            }
            for i in 0..numbers.len() - period + 1 {
                let mut ema = ExponentialMovingAverage::new(period).unwrap();
                let mut last_ema = 0.0;
                for &n in &numbers[i..i + period] {
                    last_ema = ema.nexta(n);
                }
                let last_wema = wema.nexta(numbers[i + period - 1]);
                assert!((last_ema - last_wema).abs() < 0.000000001);
            }
        }
    }

    #[test]
    fn test_new() {
        assert!(WindowedExponentialMovingAverage::new(0).is_err());
        assert!(WindowedExponentialMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_display() {
        let wema = WindowedExponentialMovingAverage::new(7).unwrap();
        assert_eq!(format!("{}", wema), "WEMA(7)");
    }
}
