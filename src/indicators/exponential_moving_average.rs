use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Close, Nexta, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An exponential moving average (EMA), also known as an exponentially weighted moving average
/// (EWMA).
///
/// It is a type of infinite impulse response filter that applies weighting factors which decrease exponentially.
/// The weighting for each older datum decreases exponentially, never reaching zero.
///
/// # Formula
///
/// ![EMA formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/05d06bdbee2c14031fd91ead6f5f772aec1ec964)
///
/// Where:
///
/// * _EMA<sub>t</sub>_ - is the value of the EMA at any time period _t_.
/// * _EMA<sub>t-1</sub>_ - is the value of the EMA at the previous period _t-1_.
/// * _p<sub>t</sub>_ - is the input value at a time period t.
/// * _α_ - is the coefficient that represents the degree of weighting decrease, a constant smoothing factor between 0 and 1.
///
/// _α_ is calculated with the following formula:
///
/// ![alpha formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/d9f6258e152db0644af548972bd6c50a8becf7ee)
///
/// Where:
///
/// * _period_ - number of periods
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use tars::indicators::ExponentialMovingAverage;
/// use tars::Nexta;
///
/// let mut ema = ExponentialMovingAverage::new(3).unwrap();
/// assert_eq!(ema.nexta(2.0), 2.0);
/// assert_eq!(ema.nexta(5.0), 3.5);
/// assert_eq!(ema.nexta(1.0), 2.25);
/// assert_eq!(ema.nexta(6.25), 4.25);
/// ```
///
/// # Links
///
/// * [Exponential moving average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average)
///

#[doc(alias = "EMA")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct ExponentialMovingAverage {
    period: usize,
    k: f64,
    current: f64,
    is_new: bool,
}

impl ExponentialMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                k: 1.0 / period as f64,
                current: 0.0,
                is_new: true,
            }),
        }
    }
}

impl Period for ExponentialMovingAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Nexta<f64> for ExponentialMovingAverage {
    type Output = f64;

    fn nexta(&mut self, input: f64) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            self.current = input;
        } else {
            self.current = self.k * input + (1.0 - self.k) * self.current;
        }
        self.current
    }
}

impl<T: Close> Nexta<&T> for ExponentialMovingAverage {
    type Output = f64;

    fn nexta(&mut self, input: &T) -> Self::Output {
        self.nexta(input.close())
    }
}

impl Reset for ExponentialMovingAverage {
    fn reset(&mut self) {
        self.current = 0.0;
        self.is_new = true;
    }
}

impl Default for ExponentialMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for ExponentialMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EMA({})", self.period)
    }
}