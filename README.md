# Technical Analysis for Rust (tars)

[![Build Status](https://img.shields.io/travis/greyblake/tars-rs)](https://travis-ci.org/greyblake/tars-rs)
[![Crates.io](https://img.shields.io/crates/v/tars)](https://crates.io/crates/tars)
[![Docs.rs](https://docs.rs/tars/badge.svg)](https://docs.rs/tars)
[![License](https://img.shields.io/crates/l/tars)](https://raw.githubusercontent.com/greyblake/tars-rs/master/LICENSE)

Technical analysis library for Rust.

* [Getting started](#getting-started)
* [Basic ideas](#basic-ideas)
* [List of indicators](#list-of-indicators)
* [Running benchmarks](#running-benchmarks)
* [Donations](#donations)
* [License](#license)
* [Contributors](#contributors)

## Getting started

Add to you `Cargo.toml`:
```
[dependencies]
tars = "0.4.0"
```

Example:

```rust
use tars::indicators::ExponentialMovingAverage;
use tars::Nexta;

// it can return an error, when an invalid length is passed (e.g. 0)
let mut ema = ExponentialMovingAverage::new(3).unwrap();

assert_eq!(ema.nexta(2.0), 2.0);
assert_eq!(ema.nexta(5.0), 3.5);
assert_eq!(ema.nexta(1.0), 2.25);
assert_eq!(ema.nexta(6.25), 4.25);
```

See more in the examples [here](https://github.com/greyblake/tars-rs/tree/master/examples).
Check also the [documentation](https://docs.rs/tars).

## Basic ideas

A data item which represent a stock quote may implement the following traits:

* `Open`
* `High`
* `Low`
* `Close`
* `Volume`

It's not necessary to implement all of them, but it must be enough to fulfill requirements for a particular indicator.
You probably should prefer using `DataItema` unless you have reasons to implement your own structure.

Indicators typically implement the following traits:

* `Nexta<T>` (often `Nexta<f64>` and `Nexta<&DataItema>`) - to feed and get the nexta value
* `Reset` - to reset an indicator
* `Debug`
* `Display`
* `Default`
* `Clone`

## List of indicators

So far there are the following indicators available.

* Trend
  * Exponential Moving Average (EMA)
  * Simple Moving Average (SMA)
* Oscillators
  * Relative Strength Index (RSI)
  * Fast Stochastic
  * Slow Stochastic
  * Moving Average Convergence Divergence (MACD)
  * Percentage Price Oscillator (PPO)
  * Commodity Channel Index (CCI)
  * Money Flow Index (MFI)
* Other
  * Minimum
  * Maximum
  * True Range
  * Standard Deviation (SD)
  * Mean Absolute Deviation (MAD)
  * Average True Range (AR)
  * Efficiency Ratio (ER)
  * Bollinger Bands (BB)
  * Chandelier Exit (CE)
  * Keltner Channel (KC)
  * Rate of Change (ROC)
  * On Balance Volume (OBV)


## Features

* `serde` - allows to serialize and deserialize indicators. NOTE: the backward compatibility of serialized
data with the future versions of tars is not guaranteed because internal implementation of the indicators is a subject to change.

## Running benchmarks

```
cargo bench
```

## Donations

You can support the project by donating [NEAR tokens](https://near.org).

Our NEAR wallet address is `tars-rs.near`


## License

[MIT](https://github.com/greyblake/tars-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)


## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
- [Bartoshko](https://github.com/Bartoshko) - BollingerBands
- [shreyasdeotare](https://github.com/shreyasdeotare) Shreyas Deotare - MoneyFlowIndex, OnBalanceVolume
- [edwardycl](https://github.com/edwardycl) - StandardDeviation Implementation & More Efficient BollingerBands
- [rideron89](https://github.com/rideron89) Ron Rider - Keltner Channel
- [tirz](https://github.com/tirz) - CCI, CE, MAD, PPO, refactorings
- [Devin Gunay](https://github.com/dgunay) - serde support
- [Youngchan Lee](https://github.com/edwardycl) - bugfix
- [tommady](https://github.com/tommady) - get rid of error-chain dependency
