//! Bindings to the [Linear Algebra PACKage][1].
//!
//! [1]: http://en.wikipedia.org/wiki/LAPACK

extern crate libc;

#[cfg(feature = "accelerate")]
extern crate accelerate_provider as raw;

#[cfg(feature = "netlib")]
extern crate netlib_provider as raw;

#[cfg(feature = "openblas")]
extern crate openblas_provider as raw;

/// A complex number with 64-bit parts.
#[allow(bad_style)]
pub type complex_double = [libc::c_double; 2];

/// A complex number with 32-bit parts.
#[allow(bad_style)]
pub type complex_float = [libc::c_float; 2];

pub mod fortran;
