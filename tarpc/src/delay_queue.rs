// Copyright 2018 Google LLC
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

//! Delay queue utilities that work on both native and WASM targets.
//!
//! On native targets, this re-exports from `tokio_util::time::delay_queue`.
//! On WASM targets, this re-exports from `wasmtimer::tokio_util::delay_queue`.

#[cfg(not(target_arch = "wasm32"))]
pub use tokio_util::time::delay_queue::*;

#[cfg(target_arch = "wasm32")]
pub use wasmtimer::tokio_util::delay_queue::*;
