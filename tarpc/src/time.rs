// Copyright 2018 Google LLC
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

//! Time utilities that work on both native and WASM targets.
//!
//! On native targets, this re-exports from `std::time`.
//! On WASM targets, this re-exports from `wasmtimer::std`.

// Duration is platform-agnostic and works on WASM
pub use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

#[cfg(target_arch = "wasm32")]
pub use wasmtimer::std::Instant;
