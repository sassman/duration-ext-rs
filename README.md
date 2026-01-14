# duration-ext

[![Crates.io](https://img.shields.io/crates/v/duration-ext.svg)](https://crates.io/crates/duration-ext)
[![Documentation](https://docs.rs/duration-ext/badge.svg)](https://docs.rs/duration-ext)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

Ergonomic extension trait for creating `std::time::Duration` from numeric literals.

## Features

- Zero dependencies
- Supports integer types: `u64`, `u32`, `usize`, `i32`
- Supports floating point: `f64`, `f32`
- Fractional values work intuitively: `1.5.seconds()` equals 1500ms
- Negative values safely clamp to zero

## Installation

```toml
[dependencies]
duration-ext = "1.0"
```

## Usage

```rust
use duration_ext::DurationExt;
use std::time::Duration;

// Integer literals
let timeout = 30.seconds();
let delay = 500.millis();
let precision = 100.micros();
let benchmark = 1000.nanos();
let long_timeout = 5.minutes();
let very_long = 2.hours();

// Fractional values (f64/f32)
let animation = 0.3.seconds();      // 300ms
let half_second = 0.5.seconds();    // 500ms
let one_and_half = 1.5.seconds();   // 1500ms
let half_minute = 0.5.minutes();    // 30s
```

## Available Methods

| Method | Description | Example |
|--------|-------------|---------|
| `.nanos()` | Nanoseconds | `1000.nanos()` |
| `.micros()` | Microseconds | `100.micros()` |
| `.millis()` | Milliseconds | `500.millis()` |
| `.seconds()` | Seconds | `5.seconds()` |
| `.minutes()` | Minutes | `2.minutes()` |
| `.hours()` | Hours | `1.hours()` |

## Examples

### Animation Timing

```rust
use duration_ext::DurationExt;

let fade_in = 0.3.seconds();
let slide_duration = 0.25.seconds();
let bounce_delay = 50.millis();
```

### Timeouts and Delays

```rust
use duration_ext::DurationExt;

let connection_timeout = 30.seconds();
let retry_delay = 5.minutes();
let session_expiry = 24.hours();
```

### High-Precision Timing

```rust
use duration_ext::DurationExt;

let frame_budget = 16.millis();
let sample_interval = 100.micros();
let spin_wait = 500.nanos();
```

## License

This project is licensed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0).
