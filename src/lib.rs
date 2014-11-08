//! The library facilitates static linking with the [Linear Algebra PACKage][1].
//!
//![1]: http://www.netlib.org/lapack/

extern crate libc;

use libc::{c_char, c_double, c_int};

pub use dsyev_ as dsyev;

#[link(name = "gfortran")]
extern {
    /// http://www.netlib.org/lapack/explore-html/dd/d4c/dsyev_8f.html
    fn dsyev_(jobz: *const c_char, uplo: *const c_char, n: *const c_int,
              a: *mut c_double, lda: *const c_int, w: *mut c_double,
              work: *mut c_double, lwork: *const c_int, info: *mut c_int);
}
