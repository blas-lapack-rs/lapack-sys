//! Bindings to [LAPACK] \(Fortran).
//!
//! ## [Architecture]
//!
//! [architecture]: https://blas-lapack-rs.github.io/architecture
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK

#![allow(non_camel_case_types)]
#![no_std]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
