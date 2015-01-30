//! Facilitation of static linking with the [Linear Algebra PACKage][1].
//!
//! [1]: http://www.netlib.org/lapack/

#![feature(libc)]

extern crate libc;

// Only for the purpose of linking with BLAS.
extern crate "libblas-sys" as raw;

use libc::{c_char, c_double, c_int};

pub use dsyev_ as dsyev;

#[link(name = "gfortran")]
extern {
    /// http://www.netlib.org/lapack/explore-html/dd/d4c/dsyev_8f.html
    fn dsyev_(JOBZ: *const c_char, UPLO: *const c_char, N: *const c_int,
              A: *mut c_double, LDA: *const c_int, W: *mut c_double,
              WORK: *mut c_double, LWORK: *const c_int, INFO: *mut c_int);
}
