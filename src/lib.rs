//! Bindings to [LAPACK] (Fortran)
//!
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK

#![allow(non_camel_case_types)]
#![no_std]

extern crate libc;

/// A complex number with 64-bit parts.
#[allow(bad_style)]
pub type c_double_complex = [libc::c_double; 2];

/// A complex number with 32-bit parts.
#[allow(bad_style)]
pub type c_float_complex = [libc::c_float; 2];

pub mod c;
pub mod fortran;
