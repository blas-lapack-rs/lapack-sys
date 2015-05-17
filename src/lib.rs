//! Bindings to the [Linear Algebra PACKage][1].
//!
//! [1]: http://en.wikipedia.org/wiki/LAPACK

#![allow(non_camel_case_types)]

#[cfg(feature = "openblas")]
extern crate openblas_provider as raw;

#[cfg(feature = "netlib")]
extern crate netlib_provider as raw;

extern crate libc;

use libc::{c_char, c_double, c_float, c_void, c_int};

pub type LAPACK_S_SELECT2 = Option<extern "C" fn(*const c_float, *const c_float) -> c_int>;
pub type LAPACK_S_SELECT3 = Option<extern "C" fn(*const c_float, *const c_float, *const c_float) -> c_int>;
pub type LAPACK_D_SELECT2 = Option<extern "C" fn(*const c_double, *const c_double) -> c_int>;
pub type LAPACK_D_SELECT3 = Option<extern "C" fn(*const c_double, *const c_double, *const c_double) -> c_int>;
pub type LAPACK_C_SELECT1 = Option<extern "C" fn(*const c_void) -> c_int>;
pub type LAPACK_C_SELECT2 = Option<extern "C" fn(*const c_void, *const c_void) -> c_int>;
pub type LAPACK_Z_SELECT1 = Option<extern "C" fn(*const c_void) -> c_int>;
pub type LAPACK_Z_SELECT2 = Option<extern "C" fn(*const c_void, *const c_void) -> c_int>;

extern "C" {
    pub fn sgetrf_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, ipiv: *mut c_int, info: *mut c_int);
    pub fn dgetrf_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *mut c_int, info: *mut c_int);
    pub fn cgetrf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, info: *mut c_int);
    pub fn zgetrf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, info: *mut c_int);

    pub fn sgbtrf_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *mut c_float, ldab: *mut c_int, ipiv: *mut c_int, info: *mut c_int);
    pub fn dgbtrf_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *mut c_double, ldab: *mut c_int, ipiv: *mut c_int, info: *mut c_int);
    pub fn cgbtrf_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *mut c_void, ldab: *mut c_int, ipiv: *mut c_int, info: *mut c_int);
    pub fn zgbtrf_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *mut c_void, ldab: *mut c_int, ipiv: *mut c_int, info: *mut c_int);

    pub fn sgttrf_(n: *mut c_int, dl: *mut c_float, d: *mut c_float, du: *mut c_float, du2: *mut c_float, ipiv: *mut c_int, info: *mut c_int);
    pub fn dgttrf_(n: *mut c_int, dl: *mut c_double, d: *mut c_double, du: *mut c_double, du2: *mut c_double, ipiv: *mut c_int, info: *mut c_int);
    pub fn cgttrf_(n: *mut c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, du2: *mut c_void, ipiv: *mut c_int, info: *mut c_int);
    pub fn zgttrf_(n: *mut c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, du2: *mut c_void, ipiv: *mut c_int, info: *mut c_int);

    pub fn spotrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, info: *mut c_int);
    pub fn dpotrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, info: *mut c_int);
    pub fn cpotrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, info: *mut c_int);
    pub fn zpotrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, info: *mut c_int);

    pub fn spstrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, piv: *mut c_int, rank: *mut c_int, tol: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn dpstrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, piv: *mut c_int, rank: *mut c_int, tol: *mut c_double, work: *mut c_double, info: *mut c_int);
    pub fn cpstrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, piv: *mut c_int, rank: *mut c_int, tol: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn zpstrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, piv: *mut c_int, rank: *mut c_int, tol: *mut c_double, work: *mut c_double, info: *mut c_int);

    pub fn spftrf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_float, info: *mut c_int);
    pub fn dpftrf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_double, info: *mut c_int);
    pub fn cpftrf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, info: *mut c_int);
    pub fn zpftrf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, info: *mut c_int);

    pub fn spptrf_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, info: *mut c_int);
    pub fn dpptrf_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, info: *mut c_int);
    pub fn cpptrf_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, info: *mut c_int);
    pub fn zpptrf_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, info: *mut c_int);

    pub fn spbtrf_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_float, ldab: *mut c_int, info: *mut c_int);
    pub fn dpbtrf_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_double, ldab: *mut c_int, info: *mut c_int);
    pub fn cpbtrf_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_void, ldab: *mut c_int, info: *mut c_int);
    pub fn zpbtrf_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_void, ldab: *mut c_int, info: *mut c_int);

    pub fn spttrf_(n: *mut c_int, d: *mut c_float, e: *mut c_float, info: *mut c_int);
    pub fn dpttrf_(n: *mut c_int, d: *mut c_double, e: *mut c_double, info: *mut c_int);
    pub fn cpttrf_(n: *mut c_int, d: *mut c_float, e: *mut c_void, info: *mut c_int);
    pub fn zpttrf_(n: *mut c_int, d: *mut c_double, e: *mut c_void, info: *mut c_int);

    pub fn ssytrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, ipiv: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dsytrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn csytrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zsytrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn chetrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zhetrf_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn ssptrf_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, ipiv: *mut c_int, info: *mut c_int);
    pub fn dsptrf_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, ipiv: *mut c_int, info: *mut c_int);
    pub fn csptrf_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, ipiv: *mut c_int, info: *mut c_int);
    pub fn zsptrf_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, ipiv: *mut c_int, info: *mut c_int);

    pub fn chptrf_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, ipiv: *mut c_int, info: *mut c_int);
    pub fn zhptrf_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, ipiv: *mut c_int, info: *mut c_int);

    pub fn sgetrs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, ipiv: *const c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dgetrs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, ipiv: *const c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cgetrs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zgetrs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn sgbtrs_(trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_float, ldab: *mut c_int, ipiv: *const c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dgbtrs_(trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_double, ldab: *mut c_int, ipiv: *const c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cgbtrs_(trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zgbtrs_(trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn sgttrs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_float, d: *const c_float, du: *const c_float, du2: *const c_float, ipiv: *const c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dgttrs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_double, d: *const c_double, du: *const c_double, du2: *const c_double, ipiv: *const c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cgttrs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zgttrs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn spotrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dpotrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cpotrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zpotrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn spftrs_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dpftrs_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cpftrs_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zpftrs_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn spptrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_float, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dpptrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_double, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cpptrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zpptrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn spbtrs_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_float, ldab: *mut c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dpbtrs_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_double, ldab: *mut c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cpbtrs_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zpbtrs_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn spttrs_(n: *mut c_int, nrhs: *mut c_int, d: *const c_float, e: *const c_float, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dpttrs_(n: *mut c_int, nrhs: *mut c_int, d: *const c_double, e: *const c_double, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cpttrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, d: *const c_float, e: *const c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zpttrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, d: *const c_double, e: *const c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn ssytrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, ipiv: *const c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dsytrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, ipiv: *const c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn csytrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zsytrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn chetrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zhetrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn ssptrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_float, ipiv: *const c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dsptrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_double, ipiv: *const c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn csptrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zsptrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn chptrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zhptrs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn strtrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dtrtrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn ctrtrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn ztrtrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn stptrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_float, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dtptrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_double, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn ctptrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn ztptrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn stbtrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_float, ldab: *mut c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dtbtrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_double, ldab: *mut c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn ctbtrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn ztbtrs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn sgecon_(norm: *mut c_char, n: *mut c_int, a: *const c_float, lda: *mut c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgecon_(norm: *mut c_char, n: *mut c_int, a: *const c_double, lda: *mut c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgecon_(norm: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgecon_(norm: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgbcon_(norm: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_float, ldab: *mut c_int, ipiv: *const c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgbcon_(norm: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_double, ldab: *mut c_int, ipiv: *const c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgbcon_(norm: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_void, ldab: *mut c_int, ipiv: *const c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgbcon_(norm: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_void, ldab: *mut c_int, ipiv: *const c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgtcon_(norm: *mut c_char, n: *mut c_int, dl: *const c_float, d: *const c_float, du: *const c_float, du2: *const c_float, ipiv: *const c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgtcon_(norm: *mut c_char, n: *mut c_int, dl: *const c_double, d: *const c_double, du: *const c_double, du2: *const c_double, ipiv: *const c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgtcon_(norm: *mut c_char, n: *mut c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_void, info: *mut c_int);
    pub fn zgtcon_(norm: *mut c_char, n: *mut c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_void, info: *mut c_int);

    pub fn spocon_(uplo: *mut c_char, n: *mut c_int, a: *const c_float, lda: *mut c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dpocon_(uplo: *mut c_char, n: *mut c_int, a: *const c_double, lda: *mut c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cpocon_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zpocon_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sppcon_(uplo: *mut c_char, n: *mut c_int, ap: *const c_float, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dppcon_(uplo: *mut c_char, n: *mut c_int, ap: *const c_double, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cppcon_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zppcon_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn spbcon_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_float, ldab: *mut c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dpbcon_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_double, ldab: *mut c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cpbcon_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_void, ldab: *mut c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zpbcon_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_void, ldab: *mut c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sptcon_(n: *mut c_int, d: *const c_float, e: *const c_float, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn dptcon_(n: *mut c_int, d: *const c_double, e: *const c_double, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_double, info: *mut c_int);
    pub fn cptcon_(n: *mut c_int, d: *const c_float, e: *const c_void, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn zptcon_(n: *mut c_int, d: *const c_double, e: *const c_void, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_double, info: *mut c_int);

    pub fn ssycon_(uplo: *mut c_char, n: *mut c_int, a: *const c_float, lda: *mut c_int, ipiv: *const c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dsycon_(uplo: *mut c_char, n: *mut c_int, a: *const c_double, lda: *mut c_int, ipiv: *const c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn csycon_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_void, info: *mut c_int);
    pub fn zsycon_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_void, info: *mut c_int);

    pub fn checon_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_void, info: *mut c_int);
    pub fn zhecon_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_void, info: *mut c_int);

    pub fn sspcon_(uplo: *mut c_char, n: *mut c_int, ap: *const c_float, ipiv: *const c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dspcon_(uplo: *mut c_char, n: *mut c_int, ap: *const c_double, ipiv: *const c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cspcon_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, ipiv: *const c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_void, info: *mut c_int);
    pub fn zspcon_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, ipiv: *const c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_void, info: *mut c_int);

    pub fn chpcon_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, ipiv: *const c_int, anorm: *mut c_float, rcond: *mut c_float, work: *mut c_void, info: *mut c_int);
    pub fn zhpcon_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, ipiv: *const c_int, anorm: *mut c_double, rcond: *mut c_double, work: *mut c_void, info: *mut c_int);

    pub fn strcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *const c_float, lda: *mut c_int, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dtrcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *const c_double, lda: *mut c_int, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn ctrcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn ztrcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn stpcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, ap: *const c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dtpcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, ap: *const c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn ctpcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, ap: *const c_void, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn ztpcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, ap: *const c_void, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn stbcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_float, ldab: *mut c_int, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dtbcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_double, ldab: *mut c_int, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn ctbcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_void, ldab: *mut c_int, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn ztbcon_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_void, ldab: *mut c_int, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgerfs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, af: *const c_float, ldaf: *mut c_int, ipiv: *const c_int, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgerfs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, af: *const c_double, ldaf: *mut c_int, ipiv: *const c_int, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgerfs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgerfs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgerfsx_(trans: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, af: *const c_float, ldaf: *mut c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgerfsx_(trans: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, af: *const c_double, ldaf: *mut c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgerfsx_(trans: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgerfsx_(trans: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgbrfs_(trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_float, ldab: *mut c_int, afb: *const c_float, ldafb: *mut c_int, ipiv: *const c_int, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgbrfs_(trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_double, ldab: *mut c_int, afb: *const c_double, ldafb: *mut c_int, ipiv: *const c_int, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgbrfs_(trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, afb: *const c_void, ldafb: *mut c_int, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgbrfs_(trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, afb: *const c_void, ldafb: *mut c_int, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgbrfsx_(trans: *mut c_char, equed: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_float, ldab: *mut c_int, afb: *const c_float, ldafb: *mut c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgbrfsx_(trans: *mut c_char, equed: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_double, ldab: *mut c_int, afb: *const c_double, ldafb: *mut c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgbrfsx_(trans: *mut c_char, equed: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, afb: *const c_void, ldafb: *mut c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgbrfsx_(trans: *mut c_char, equed: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, afb: *const c_void, ldafb: *mut c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgtrfs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_float, d: *const c_float, du: *const c_float, dlf: *const c_float, df: *const c_float, duf: *const c_float, du2: *const c_float, ipiv: *const c_int, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgtrfs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_double, d: *const c_double, du: *const c_double, dlf: *const c_double, df: *const c_double, duf: *const c_double, du2: *const c_double, ipiv: *const c_int, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgtrfs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *const c_void, df: *const c_void, duf: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgtrfs_(trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *const c_void, df: *const c_void, duf: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sporfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, af: *const c_float, ldaf: *mut c_int, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dporfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, af: *const c_double, ldaf: *mut c_int, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cporfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zporfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sporfsx_(uplo: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, af: *const c_float, ldaf: *mut c_int, s: *const c_float, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dporfsx_(uplo: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, af: *const c_double, ldaf: *mut c_int, s: *const c_double, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cporfsx_(uplo: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, s: *const c_float, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zporfsx_(uplo: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, s: *const c_double, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn spprfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_float, afp: *const c_float, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dpprfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_double, afp: *const c_double, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cpprfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, afp: *const c_void, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zpprfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, afp: *const c_void, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn spbrfs_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_float, ldab: *mut c_int, afb: *const c_float, ldafb: *mut c_int, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dpbrfs_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_double, ldab: *mut c_int, afb: *const c_double, ldafb: *mut c_int, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cpbrfs_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, afb: *const c_void, ldafb: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zpbrfs_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, afb: *const c_void, ldafb: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sptrfs_(n: *mut c_int, nrhs: *mut c_int, d: *const c_float, e: *const c_float, df: *const c_float, ef: *const c_float, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn dptrfs_(n: *mut c_int, nrhs: *mut c_int, d: *const c_double, e: *const c_double, df: *const c_double, ef: *const c_double, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, info: *mut c_int);
    pub fn cptrfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, d: *const c_float, e: *const c_void, df: *const c_float, ef: *const c_void, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zptrfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, d: *const c_double, e: *const c_void, df: *const c_double, ef: *const c_void, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn ssyrfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, af: *const c_float, ldaf: *mut c_int, ipiv: *const c_int, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dsyrfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, af: *const c_double, ldaf: *mut c_int, ipiv: *const c_int, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn csyrfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zsyrfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn ssyrfsx_(uplo: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, af: *const c_float, ldaf: *mut c_int, ipiv: *const c_int, s: *const c_float, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dsyrfsx_(uplo: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, af: *const c_double, ldaf: *mut c_int, ipiv: *const c_int, s: *const c_double, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn csyrfsx_(uplo: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, s: *const c_float, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zsyrfsx_(uplo: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, s: *const c_double, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn cherfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zherfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn cherfsx_(uplo: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, s: *const c_float, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zherfsx_(uplo: *mut c_char, equed: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *const c_void, ldaf: *mut c_int, ipiv: *const c_int, s: *const c_double, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn ssprfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_float, afp: *const c_float, ipiv: *const c_int, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dsprfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_double, afp: *const c_double, ipiv: *const c_int, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn csprfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zsprfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn chprfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zhprfs_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn strrfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, b: *const c_float, ldb: *mut c_int, x: *const c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dtrrfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, b: *const c_double, ldb: *mut c_int, x: *const c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn ctrrfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *const c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn ztrrfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *const c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn stprfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_float, b: *const c_float, ldb: *mut c_int, x: *const c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dtprfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_double, b: *const c_double, ldb: *mut c_int, x: *const c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn ctprfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, b: *const c_void, ldb: *mut c_int, x: *const c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn ztprfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, b: *const c_void, ldb: *mut c_int, x: *const c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn stbrfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_float, ldab: *mut c_int, b: *const c_float, ldb: *mut c_int, x: *const c_float, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dtbrfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_double, ldab: *mut c_int, b: *const c_double, ldb: *mut c_int, x: *const c_double, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn ctbrfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *const c_void, ldx: *mut c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn ztbrfs_(uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *const c_void, ldab: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *const c_void, ldx: *mut c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgetri_(n: *mut c_int, a: *mut c_float, lda: *mut c_int, ipiv: *const c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgetri_(n: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *const c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgetri_(n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zgetri_(n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn spotri_(uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, info: *mut c_int);
    pub fn dpotri_(uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, info: *mut c_int);
    pub fn cpotri_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, info: *mut c_int);
    pub fn zpotri_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, info: *mut c_int);

    pub fn spftri_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_float, info: *mut c_int);
    pub fn dpftri_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_double, info: *mut c_int);
    pub fn cpftri_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, info: *mut c_int);
    pub fn zpftri_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, info: *mut c_int);

    pub fn spptri_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, info: *mut c_int);
    pub fn dpptri_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, info: *mut c_int);
    pub fn cpptri_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, info: *mut c_int);
    pub fn zpptri_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, info: *mut c_int);

    pub fn ssytri_(uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, ipiv: *const c_int, work: *mut c_float, info: *mut c_int);
    pub fn dsytri_(uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *const c_int, work: *mut c_double, info: *mut c_int);
    pub fn csytri_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, info: *mut c_int);
    pub fn zsytri_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, info: *mut c_int);

    pub fn chetri_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, info: *mut c_int);
    pub fn zhetri_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, info: *mut c_int);

    pub fn ssptri_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, ipiv: *const c_int, work: *mut c_float, info: *mut c_int);
    pub fn dsptri_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, ipiv: *const c_int, work: *mut c_double, info: *mut c_int);
    pub fn csptri_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, ipiv: *const c_int, work: *mut c_void, info: *mut c_int);
    pub fn zsptri_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, ipiv: *const c_int, work: *mut c_void, info: *mut c_int);

    pub fn chptri_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, ipiv: *const c_int, work: *mut c_void, info: *mut c_int);
    pub fn zhptri_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, ipiv: *const c_int, work: *mut c_void, info: *mut c_int);

    pub fn strtri_(uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, info: *mut c_int);
    pub fn dtrtri_(uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, info: *mut c_int);
    pub fn ctrtri_(uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, info: *mut c_int);
    pub fn ztrtri_(uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, info: *mut c_int);

    pub fn stftri_(transr: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *mut c_float, info: *mut c_int);
    pub fn dtftri_(transr: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *mut c_double, info: *mut c_int);
    pub fn ctftri_(transr: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *mut c_void, info: *mut c_int);
    pub fn ztftri_(transr: *mut c_char, uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, a: *mut c_void, info: *mut c_int);

    pub fn stptri_(uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, ap: *mut c_float, info: *mut c_int);
    pub fn dtptri_(uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, ap: *mut c_double, info: *mut c_int);
    pub fn ctptri_(uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, ap: *mut c_void, info: *mut c_int);
    pub fn ztptri_(uplo: *mut c_char, diag: *mut c_char, n: *mut c_int, ap: *mut c_void, info: *mut c_int);

    pub fn sgeequ_(m: *mut c_int, n: *mut c_int, a: *const c_float, lda: *mut c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn dgeequ_(m: *mut c_int, n: *mut c_int, a: *const c_double, lda: *mut c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);
    pub fn cgeequ_(m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn zgeequ_(m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);

    pub fn sgeequb_(m: *mut c_int, n: *mut c_int, a: *const c_float, lda: *mut c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn dgeequb_(m: *mut c_int, n: *mut c_int, a: *const c_double, lda: *mut c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);
    pub fn cgeequb_(m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn zgeequb_(m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);

    pub fn sgbequ_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_float, ldab: *mut c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn dgbequ_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_double, ldab: *mut c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);
    pub fn cgbequ_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_void, ldab: *mut c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn zgbequ_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_void, ldab: *mut c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);

    pub fn sgbequb_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_float, ldab: *mut c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn dgbequb_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_double, ldab: *mut c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);
    pub fn cgbequb_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_void, ldab: *mut c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn zgbequb_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *const c_void, ldab: *mut c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double, info: *mut c_int);

    pub fn spoequ_(n: *mut c_int, a: *const c_float, lda: *mut c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn dpoequ_(n: *mut c_int, a: *const c_double, lda: *mut c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, info: *mut c_int);
    pub fn cpoequ_(n: *mut c_int, a: *const c_void, lda: *mut c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn zpoequ_(n: *mut c_int, a: *const c_void, lda: *mut c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, info: *mut c_int);

    pub fn spoequb_(n: *mut c_int, a: *const c_float, lda: *mut c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn dpoequb_(n: *mut c_int, a: *const c_double, lda: *mut c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, info: *mut c_int);
    pub fn cpoequb_(n: *mut c_int, a: *const c_void, lda: *mut c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn zpoequb_(n: *mut c_int, a: *const c_void, lda: *mut c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, info: *mut c_int);

    pub fn sppequ_(uplo: *mut c_char, n: *mut c_int, ap: *const c_float, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn dppequ_(uplo: *mut c_char, n: *mut c_int, ap: *const c_double, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, info: *mut c_int);
    pub fn cppequ_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn zppequ_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, info: *mut c_int);

    pub fn spbequ_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_float, ldab: *mut c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn dpbequ_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_double, ldab: *mut c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, info: *mut c_int);
    pub fn cpbequ_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_void, ldab: *mut c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, info: *mut c_int);
    pub fn zpbequ_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *const c_void, ldab: *mut c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, info: *mut c_int);

    pub fn ssyequb_(uplo: *mut c_char, n: *mut c_int, a: *const c_float, lda: *mut c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn dsyequb_(uplo: *mut c_char, n: *mut c_int, a: *const c_double, lda: *mut c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, work: *mut c_double, info: *mut c_int);
    pub fn csyequb_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, work: *mut c_void, info: *mut c_int);
    pub fn zsyequb_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, work: *mut c_void, info: *mut c_int);

    pub fn cheequb_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, work: *mut c_void, info: *mut c_int);
    pub fn zheequb_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, work: *mut c_void, info: *mut c_int);

    pub fn sgesv_(n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dgesv_(n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cgesv_(n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zgesv_(n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn dsgesv_(n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, work: *mut c_double, swork: *mut c_float, iter: *mut c_int, info: *mut c_int);
    pub fn zcgesv_(n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, work: *mut c_void, swork: *mut c_void, rwork: *mut c_double, iter: *mut c_int, info: *mut c_int);

    pub fn sgesvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, af: *mut c_float, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgesvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, af: *mut c_double, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgesvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgesvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgesvxx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, af: *mut c_float, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgesvxx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, af: *mut c_double, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgesvxx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgesvxx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgbsv_(n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_float, ldab: *mut c_int, ipiv: *mut c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dgbsv_(n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_double, ldab: *mut c_int, ipiv: *mut c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cgbsv_(n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_void, ldab: *mut c_int, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zgbsv_(n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_void, ldab: *mut c_int, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn sgbsvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_float, ldab: *mut c_int, afb: *mut c_float, ldafb: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgbsvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_double, ldab: *mut c_int, afb: *mut c_double, ldafb: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgbsvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_void, ldab: *mut c_int, afb: *mut c_void, ldafb: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgbsvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_void, ldab: *mut c_int, afb: *mut c_void, ldafb: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgbsvxx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_float, ldab: *mut c_int, afb: *mut c_float, ldafb: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgbsvxx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_double, ldab: *mut c_int, afb: *mut c_double, ldafb: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgbsvxx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_void, ldab: *mut c_int, afb: *mut c_void, ldafb: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgbsvxx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, nrhs: *mut c_int, ab: *mut c_void, ldab: *mut c_int, afb: *mut c_void, ldafb: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgtsv_(n: *mut c_int, nrhs: *mut c_int, dl: *mut c_float, d: *mut c_float, du: *mut c_float, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dgtsv_(n: *mut c_int, nrhs: *mut c_int, dl: *mut c_double, d: *mut c_double, du: *mut c_double, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cgtsv_(n: *mut c_int, nrhs: *mut c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zgtsv_(n: *mut c_int, nrhs: *mut c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn sgtsvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_float, d: *const c_float, du: *const c_float, dlf: *mut c_float, df: *mut c_float, duf: *mut c_float, du2: *mut c_float, ipiv: *mut c_int, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dgtsvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_double, d: *const c_double, du: *const c_double, dlf: *mut c_double, df: *mut c_double, duf: *mut c_double, du2: *mut c_double, ipiv: *mut c_int, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cgtsvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *mut c_void, df: *mut c_void, duf: *mut c_void, du2: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgtsvx_(fact: *mut c_char, trans: *mut c_char, n: *mut c_int, nrhs: *mut c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *mut c_void, df: *mut c_void, duf: *mut c_void, du2: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sposv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dposv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cposv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zposv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn dsposv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, work: *mut c_double, swork: *mut c_float, iter: *mut c_int, info: *mut c_int);
    pub fn zcposv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, work: *mut c_void, swork: *mut c_void, rwork: *mut c_double, iter: *mut c_int, info: *mut c_int);

    pub fn sposvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, af: *mut c_float, ldaf: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dposvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, af: *mut c_double, ldaf: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cposvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zposvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sposvxx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, af: *mut c_float, ldaf: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dposvxx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, af: *mut c_double, ldaf: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cposvxx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zposvxx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sppsv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_float, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dppsv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_double, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cppsv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zppsv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn sppsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_float, afp: *mut c_float, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dppsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_double, afp: *mut c_double, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cppsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_void, afp: *mut c_void, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zppsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_void, afp: *mut c_void, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn spbsv_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *mut c_float, ldab: *mut c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dpbsv_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *mut c_double, ldab: *mut c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cpbsv_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *mut c_void, ldab: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zpbsv_(uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *mut c_void, ldab: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn spbsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *mut c_float, ldab: *mut c_int, afb: *mut c_float, ldafb: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dpbsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *mut c_double, ldab: *mut c_int, afb: *mut c_double, ldafb: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cpbsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *mut c_void, ldab: *mut c_int, afb: *mut c_void, ldafb: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zpbsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, nrhs: *mut c_int, ab: *mut c_void, ldab: *mut c_int, afb: *mut c_void, ldafb: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sptsv_(n: *mut c_int, nrhs: *mut c_int, d: *mut c_float, e: *mut c_float, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dptsv_(n: *mut c_int, nrhs: *mut c_int, d: *mut c_double, e: *mut c_double, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cptsv_(n: *mut c_int, nrhs: *mut c_int, d: *mut c_float, e: *mut c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zptsv_(n: *mut c_int, nrhs: *mut c_int, d: *mut c_double, e: *mut c_void, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn sptsvx_(fact: *mut c_char, n: *mut c_int, nrhs: *mut c_int, d: *const c_float, e: *const c_float, df: *mut c_float, ef: *mut c_float, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn dptsvx_(fact: *mut c_char, n: *mut c_int, nrhs: *mut c_int, d: *const c_double, e: *const c_double, df: *mut c_double, ef: *mut c_double, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, info: *mut c_int);
    pub fn cptsvx_(fact: *mut c_char, n: *mut c_int, nrhs: *mut c_int, d: *const c_float, e: *const c_void, df: *mut c_float, ef: *mut c_void, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zptsvx_(fact: *mut c_char, n: *mut c_int, nrhs: *mut c_int, d: *const c_double, e: *const c_void, df: *mut c_double, ef: *mut c_void, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn ssysv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_float, ldb: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dsysv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_double, ldb: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn csysv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zsysv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn ssysvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, af: *mut c_float, ldaf: *mut c_int, ipiv: *mut c_int, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn dsysvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, af: *mut c_double, ldaf: *mut c_int, ipiv: *mut c_int, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn csysvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zsysvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn dsysvxx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, af: *mut c_double, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn ssysvxx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, af: *mut c_float, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn csysvxx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zsysvxx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn chesv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zhesv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn chesvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zhesvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn chesvxx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: *mut c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zhesvxx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, af: *mut c_void, ldaf: *mut c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: *mut c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: *mut c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sspsv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_float, ipiv: *mut c_int, b: *mut c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dspsv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_double, ipiv: *mut c_int, b: *mut c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn cspsv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zspsv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn sspsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_float, afp: *mut c_float, ipiv: *mut c_int, b: *const c_float, ldb: *mut c_int, x: *mut c_float, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dspsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_double, afp: *mut c_double, ipiv: *mut c_int, b: *const c_double, ldb: *mut c_int, x: *mut c_double, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cspsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zspsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn chpsv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zhpsv_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn chpsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zhpsvx_(fact: *mut c_char, uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: *mut c_int, x: *mut c_void, ldx: *mut c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgeqrf_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgeqrf_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgeqrf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zgeqrf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sgeqpf_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, jpvt: *mut c_int, tau: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn dgeqpf_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, jpvt: *mut c_int, tau: *mut c_double, work: *mut c_double, info: *mut c_int);
    pub fn cgeqpf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, jpvt: *mut c_int, tau: *mut c_void, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgeqpf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, jpvt: *mut c_int, tau: *mut c_void, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sgeqp3_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, jpvt: *mut c_int, tau: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgeqp3_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, jpvt: *mut c_int, tau: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgeqp3_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, jpvt: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zgeqp3_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, jpvt: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn sorgqr_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *const c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dorgqr_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *const c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);

    pub fn sormqr_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_float, lda: *mut c_int, tau: *const c_float, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dormqr_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_double, lda: *mut c_int, tau: *const c_double, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);

    pub fn cungqr_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zungqr_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn cunmqr_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zunmqr_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sgelqf_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgelqf_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgelqf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zgelqf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sorglq_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *const c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dorglq_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *const c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);

    pub fn sormlq_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_float, lda: *mut c_int, tau: *const c_float, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dormlq_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_double, lda: *mut c_int, tau: *const c_double, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);

    pub fn cunglq_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zunglq_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn cunmlq_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zunmlq_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sgeqlf_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgeqlf_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgeqlf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zgeqlf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sorgql_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *const c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dorgql_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *const c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cungql_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zungql_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sormql_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_float, lda: *mut c_int, tau: *const c_float, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dormql_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_double, lda: *mut c_int, tau: *const c_double, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cunmql_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zunmql_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sgerqf_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgerqf_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgerqf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zgerqf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sorgrq_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *const c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dorgrq_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *const c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cungrq_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zungrq_(m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sormrq_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_float, lda: *mut c_int, tau: *const c_float, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dormrq_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_double, lda: *mut c_int, tau: *const c_double, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cunmrq_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zunmrq_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn stzrzf_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dtzrzf_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn ctzrzf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn ztzrzf_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sormrz_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, a: *const c_float, lda: *mut c_int, tau: *const c_float, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dormrz_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, a: *const c_double, lda: *mut c_int, tau: *const c_double, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cunmrz_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zunmrz_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sggqrf_(n: *mut c_int, m: *mut c_int, p: *mut c_int, a: *mut c_float, lda: *mut c_int, taua: *mut c_float, b: *mut c_float, ldb: *mut c_int, taub: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dggqrf_(n: *mut c_int, m: *mut c_int, p: *mut c_int, a: *mut c_double, lda: *mut c_int, taua: *mut c_double, b: *mut c_double, ldb: *mut c_int, taub: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cggqrf_(n: *mut c_int, m: *mut c_int, p: *mut c_int, a: *mut c_void, lda: *mut c_int, taua: *mut c_void, b: *mut c_void, ldb: *mut c_int, taub: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zggqrf_(n: *mut c_int, m: *mut c_int, p: *mut c_int, a: *mut c_void, lda: *mut c_int, taua: *mut c_void, b: *mut c_void, ldb: *mut c_int, taub: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sggrqf_(m: *mut c_int, p: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, taua: *mut c_float, b: *mut c_float, ldb: *mut c_int, taub: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dggrqf_(m: *mut c_int, p: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, taua: *mut c_double, b: *mut c_double, ldb: *mut c_int, taub: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cggrqf_(m: *mut c_int, p: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, taua: *mut c_void, b: *mut c_void, ldb: *mut c_int, taub: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zggrqf_(m: *mut c_int, p: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, taua: *mut c_void, b: *mut c_void, ldb: *mut c_int, taub: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sgebrd_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, d: *mut c_float, e: *mut c_float, tauq: *mut c_float, taup: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgebrd_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, d: *mut c_double, e: *mut c_double, tauq: *mut c_double, taup: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgebrd_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, d: *mut c_float, e: *mut c_float, tauq: *mut c_void, taup: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zgebrd_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, d: *mut c_double, e: *mut c_double, tauq: *mut c_void, taup: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sgbbrd_(vect: *mut c_char, m: *mut c_int, n: *mut c_int, ncc: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *mut c_float, ldab: *mut c_int, d: *mut c_float, e: *mut c_float, q: *mut c_float, ldq: *mut c_int, pt: *mut c_float, ldpt: *mut c_int, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dgbbrd_(vect: *mut c_char, m: *mut c_int, n: *mut c_int, ncc: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *mut c_double, ldab: *mut c_int, d: *mut c_double, e: *mut c_double, q: *mut c_double, ldq: *mut c_int, pt: *mut c_double, ldpt: *mut c_int, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn cgbbrd_(vect: *mut c_char, m: *mut c_int, n: *mut c_int, ncc: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *mut c_void, ldab: *mut c_int, d: *mut c_float, e: *mut c_float, q: *mut c_void, ldq: *mut c_int, pt: *mut c_void, ldpt: *mut c_int, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zgbbrd_(vect: *mut c_char, m: *mut c_int, n: *mut c_int, ncc: *mut c_int, kl: *mut c_int, ku: *mut c_int, ab: *mut c_void, ldab: *mut c_int, d: *mut c_double, e: *mut c_double, q: *mut c_void, ldq: *mut c_int, pt: *mut c_void, ldpt: *mut c_int, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sorgbr_(vect: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *const c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dorgbr_(vect: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *const c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);

    pub fn sormbr_(vect: *mut c_char, side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_float, lda: *mut c_int, tau: *const c_float, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dormbr_(vect: *mut c_char, side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_double, lda: *mut c_int, tau: *const c_double, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);

    pub fn cungbr_(vect: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zungbr_(vect: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn cunmbr_(vect: *mut c_char, side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zunmbr_(vect: *mut c_char, side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sbdsqr_(uplo: *mut c_char, n: *mut c_int, ncvt: *mut c_int, nru: *mut c_int, ncc: *mut c_int, d: *mut c_float, e: *mut c_float, vt: *mut c_float, ldvt: *mut c_int, u: *mut c_float, ldu: *mut c_int, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dbdsqr_(uplo: *mut c_char, n: *mut c_int, ncvt: *mut c_int, nru: *mut c_int, ncc: *mut c_int, d: *mut c_double, e: *mut c_double, vt: *mut c_double, ldvt: *mut c_int, u: *mut c_double, ldu: *mut c_int, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn cbdsqr_(uplo: *mut c_char, n: *mut c_int, ncvt: *mut c_int, nru: *mut c_int, ncc: *mut c_int, d: *mut c_float, e: *mut c_float, vt: *mut c_void, ldvt: *mut c_int, u: *mut c_void, ldu: *mut c_int, c: *mut c_void, ldc: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn zbdsqr_(uplo: *mut c_char, n: *mut c_int, ncvt: *mut c_int, nru: *mut c_int, ncc: *mut c_int, d: *mut c_double, e: *mut c_double, vt: *mut c_void, ldvt: *mut c_int, u: *mut c_void, ldu: *mut c_int, c: *mut c_void, ldc: *mut c_int, work: *mut c_double, info: *mut c_int);

    pub fn sbdsdc_(uplo: *mut c_char, compq: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, u: *mut c_float, ldu: *mut c_int, vt: *mut c_float, ldvt: *mut c_int, q: *mut c_float, iq: *mut c_int, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dbdsdc_(uplo: *mut c_char, compq: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, u: *mut c_double, ldu: *mut c_int, vt: *mut c_double, ldvt: *mut c_int, q: *mut c_double, iq: *mut c_int, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);

    pub fn ssytrd_(uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, d: *mut c_float, e: *mut c_float, tau: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dsytrd_(uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, d: *mut c_double, e: *mut c_double, tau: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);

    pub fn sorgtr_(uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *const c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dorgtr_(uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *const c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);

    pub fn sormtr_(side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_float, lda: *mut c_int, tau: *const c_float, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dormtr_(side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_double, lda: *mut c_int, tau: *const c_double, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);

    pub fn chetrd_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, d: *mut c_float, e: *mut c_float, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zhetrd_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, d: *mut c_double, e: *mut c_double, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn cungtr_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zungtr_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn cunmtr_(side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zunmtr_(side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn ssptrd_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, d: *mut c_float, e: *mut c_float, tau: *mut c_float, info: *mut c_int);
    pub fn dsptrd_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, d: *mut c_double, e: *mut c_double, tau: *mut c_double, info: *mut c_int);

    pub fn sopgtr_(uplo: *mut c_char, n: *mut c_int, ap: *const c_float, tau: *const c_float, q: *mut c_float, ldq: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dopgtr_(uplo: *mut c_char, n: *mut c_int, ap: *const c_double, tau: *const c_double, q: *mut c_double, ldq: *mut c_int, work: *mut c_double, info: *mut c_int);

    pub fn sopmtr_(side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, ap: *const c_float, tau: *const c_float, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dopmtr_(side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, ap: *const c_double, tau: *const c_double, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, info: *mut c_int);

    pub fn chptrd_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, d: *mut c_float, e: *mut c_float, tau: *mut c_void, info: *mut c_int);
    pub fn zhptrd_(uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, d: *mut c_double, e: *mut c_double, tau: *mut c_void, info: *mut c_int);

    pub fn cupgtr_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, tau: *const c_void, q: *mut c_void, ldq: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn zupgtr_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, tau: *const c_void, q: *mut c_void, ldq: *mut c_int, work: *mut c_void, info: *mut c_int);

    pub fn cupmtr_(side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, ap: *const c_void, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn zupmtr_(side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, ap: *const c_void, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, info: *mut c_int);

    pub fn ssbtrd_(vect: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_float, ldab: *mut c_int, d: *mut c_float, e: *mut c_float, q: *mut c_float, ldq: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dsbtrd_(vect: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_double, ldab: *mut c_int, d: *mut c_double, e: *mut c_double, q: *mut c_double, ldq: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn chbtrd_(vect: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_void, ldab: *mut c_int, d: *mut c_float, e: *mut c_float, q: *mut c_void, ldq: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn zhbtrd_(vect: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_void, ldab: *mut c_int, d: *mut c_double, e: *mut c_double, q: *mut c_void, ldq: *mut c_int, work: *mut c_void, info: *mut c_int);

    pub fn ssterf_(n: *mut c_int, d: *mut c_float, e: *mut c_float, info: *mut c_int);
    pub fn dsterf_(n: *mut c_int, d: *mut c_double, e: *mut c_double, info: *mut c_int);

    pub fn ssteqr_(compz: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dsteqr_(compz: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn csteqr_(compz: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn zsteqr_(compz: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_double, info: *mut c_int);

    pub fn sstemr_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, nzc: *mut c_int, isuppz: *mut c_int, tryrac: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dstemr_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, nzc: *mut c_int, isuppz: *mut c_int, tryrac: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn cstemr_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, nzc: *mut c_int, isuppz: *mut c_int, tryrac: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn zstemr_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, nzc: *mut c_int, isuppz: *mut c_int, tryrac: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn sstedc_(compz: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dstedc_(compz: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn cstedc_(compz: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn zstedc_(compz: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn sstegr_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, isuppz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dstegr_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, isuppz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn cstegr_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, isuppz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn zstegr_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, isuppz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn spteqr_(compz: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dpteqr_(compz: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn cpteqr_(compz: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn zpteqr_(compz: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_double, info: *mut c_int);

    pub fn sstebz_(range: *mut c_char, order: *mut c_char, n: *mut c_int, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, d: *const c_float, e: *const c_float, m: *mut c_int, nsplit: *mut c_int, w: *mut c_float, iblock: *mut c_int, isplit: *mut c_int, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dstebz_(range: *mut c_char, order: *mut c_char, n: *mut c_int, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, d: *const c_double, e: *const c_double, m: *mut c_int, nsplit: *mut c_int, w: *mut c_double, iblock: *mut c_int, isplit: *mut c_int, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);

    pub fn sstein_(n: *mut c_int, d: *const c_float, e: *const c_float, m: *mut c_int, w: *const c_float, iblock: *const c_int, isplit: *const c_int, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, iwork: *mut c_int, ifailv: *mut c_int, info: *mut c_int);
    pub fn dstein_(n: *mut c_int, d: *const c_double, e: *const c_double, m: *mut c_int, w: *const c_double, iblock: *const c_int, isplit: *const c_int, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, iwork: *mut c_int, ifailv: *mut c_int, info: *mut c_int);
    pub fn cstein_(n: *mut c_int, d: *const c_float, e: *const c_float, m: *mut c_int, w: *const c_float, iblock: *const c_int, isplit: *const c_int, z: *mut c_void, ldz: *mut c_int, work: *mut c_float, iwork: *mut c_int, ifailv: *mut c_int, info: *mut c_int);
    pub fn zstein_(n: *mut c_int, d: *const c_double, e: *const c_double, m: *mut c_int, w: *const c_double, iblock: *const c_int, isplit: *const c_int, z: *mut c_void, ldz: *mut c_int, work: *mut c_double, iwork: *mut c_int, ifailv: *mut c_int, info: *mut c_int);

    pub fn sdisna_(job: *mut c_char, m: *mut c_int, n: *mut c_int, d: *const c_float, sep: *mut c_float, info: *mut c_int);
    pub fn ddisna_(job: *mut c_char, m: *mut c_int, n: *mut c_int, d: *const c_double, sep: *mut c_double, info: *mut c_int);

    pub fn ssygst_(itype: *mut c_int, uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *const c_float, ldb: *mut c_int, info: *mut c_int);
    pub fn dsygst_(itype: *mut c_int, uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *const c_double, ldb: *mut c_int, info: *mut c_int);
    pub fn chegst_(itype: *mut c_int, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *const c_void, ldb: *mut c_int, info: *mut c_int);
    pub fn zhegst_(itype: *mut c_int, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *const c_void, ldb: *mut c_int, info: *mut c_int);

    pub fn sspgst_(itype: *mut c_int, uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, bp: *const c_float, info: *mut c_int);
    pub fn dspgst_(itype: *mut c_int, uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, bp: *const c_double, info: *mut c_int);
    pub fn chpgst_(itype: *mut c_int, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, bp: *const c_void, info: *mut c_int);
    pub fn zhpgst_(itype: *mut c_int, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, bp: *const c_void, info: *mut c_int);

    pub fn ssbgst_(vect: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_float, ldab: *mut c_int, bb: *const c_float, ldbb: *mut c_int, x: *mut c_float, ldx: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dsbgst_(vect: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_double, ldab: *mut c_int, bb: *const c_double, ldbb: *mut c_int, x: *mut c_double, ldx: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn chbgst_(vect: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_void, ldab: *mut c_int, bb: *const c_void, ldbb: *mut c_int, x: *mut c_void, ldx: *mut c_int, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zhbgst_(vect: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_void, ldab: *mut c_int, bb: *const c_void, ldbb: *mut c_int, x: *mut c_void, ldx: *mut c_int, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn spbstf_(uplo: *mut c_char, n: *mut c_int, kb: *mut c_int, bb: *mut c_float, ldbb: *mut c_int, info: *mut c_int);
    pub fn dpbstf_(uplo: *mut c_char, n: *mut c_int, kb: *mut c_int, bb: *mut c_double, ldbb: *mut c_int, info: *mut c_int);
    pub fn cpbstf_(uplo: *mut c_char, n: *mut c_int, kb: *mut c_int, bb: *mut c_void, ldbb: *mut c_int, info: *mut c_int);
    pub fn zpbstf_(uplo: *mut c_char, n: *mut c_int, kb: *mut c_int, bb: *mut c_void, ldbb: *mut c_int, info: *mut c_int);

    pub fn sgehrd_(n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgehrd_(n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgehrd_(n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zgehrd_(n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sorghr_(n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *const c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dorghr_(n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *const c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);

    pub fn sormhr_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *const c_float, lda: *mut c_int, tau: *const c_float, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dormhr_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *const c_double, lda: *mut c_int, tau: *const c_double, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);

    pub fn cunghr_(n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zunghr_(n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *const c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn cunmhr_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zunmhr_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *const c_void, lda: *mut c_int, tau: *const c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sgebal_(job: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float, info: *mut c_int);
    pub fn dgebal_(job: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double, info: *mut c_int);
    pub fn cgebal_(job: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float, info: *mut c_int);
    pub fn zgebal_(job: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double, info: *mut c_int);

    pub fn sgebak_(job: *mut c_char, side: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *const c_float, m: *mut c_int, v: *mut c_float, ldv: *mut c_int, info: *mut c_int);
    pub fn dgebak_(job: *mut c_char, side: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *const c_double, m: *mut c_int, v: *mut c_double, ldv: *mut c_int, info: *mut c_int);
    pub fn cgebak_(job: *mut c_char, side: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *const c_float, m: *mut c_int, v: *mut c_void, ldv: *mut c_int, info: *mut c_int);
    pub fn zgebak_(job: *mut c_char, side: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *const c_double, m: *mut c_int, v: *mut c_void, ldv: *mut c_int, info: *mut c_int);

    pub fn shseqr_(job: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, h: *mut c_float, ldh: *mut c_int, wr: *mut c_float, wi: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dhseqr_(job: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, h: *mut c_double, ldh: *mut c_int, wr: *mut c_double, wi: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn chseqr_(job: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, h: *mut c_void, ldh: *mut c_int, w: *mut c_void, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zhseqr_(job: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, h: *mut c_void, ldh: *mut c_int, w: *mut c_void, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn shsein_(job: *mut c_char, eigsrc: *mut c_char, initv: *mut c_char, select: *mut c_int, n: *mut c_int, h: *const c_float, ldh: *mut c_int, wr: *mut c_float, wi: *const c_float, vl: *mut c_float, ldvl: *mut c_int, vr: *mut c_float, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_float, ifaill: *mut c_int, ifailr: *mut c_int, info: *mut c_int);
    pub fn dhsein_(job: *mut c_char, eigsrc: *mut c_char, initv: *mut c_char, select: *mut c_int, n: *mut c_int, h: *const c_double, ldh: *mut c_int, wr: *mut c_double, wi: *const c_double, vl: *mut c_double, ldvl: *mut c_int, vr: *mut c_double, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_double, ifaill: *mut c_int, ifailr: *mut c_int, info: *mut c_int);
    pub fn chsein_(job: *mut c_char, eigsrc: *mut c_char, initv: *mut c_char, select: *const c_int, n: *mut c_int, h: *const c_void, ldh: *mut c_int, w: *mut c_void, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_float, ifaill: *mut c_int, ifailr: *mut c_int, info: *mut c_int);
    pub fn zhsein_(job: *mut c_char, eigsrc: *mut c_char, initv: *mut c_char, select: *const c_int, n: *mut c_int, h: *const c_void, ldh: *mut c_int, w: *mut c_void, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_double, ifaill: *mut c_int, ifailr: *mut c_int, info: *mut c_int);

    pub fn strevc_(side: *mut c_char, howmny: *mut c_char, select: *mut c_int, n: *mut c_int, t: *const c_float, ldt: *mut c_int, vl: *mut c_float, ldvl: *mut c_int, vr: *mut c_float, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dtrevc_(side: *mut c_char, howmny: *mut c_char, select: *mut c_int, n: *mut c_int, t: *const c_double, ldt: *mut c_int, vl: *mut c_double, ldvl: *mut c_int, vr: *mut c_double, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn ctrevc_(side: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, t: *mut c_void, ldt: *mut c_int, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn ztrevc_(side: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, t: *mut c_void, ldt: *mut c_int, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn strsna_(job: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, t: *const c_float, ldt: *mut c_int, vl: *const c_float, ldvl: *mut c_int, vr: *const c_float, ldvr: *mut c_int, s: *mut c_float, sep: *mut c_float, mm: *mut c_int, m: *mut c_int, work: *mut c_float, ldwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn dtrsna_(job: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, t: *const c_double, ldt: *mut c_int, vl: *const c_double, ldvl: *mut c_int, vr: *const c_double, ldvr: *mut c_int, s: *mut c_double, sep: *mut c_double, mm: *mut c_int, m: *mut c_int, work: *mut c_double, ldwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn ctrsna_(job: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, t: *const c_void, ldt: *mut c_int, vl: *const c_void, ldvl: *mut c_int, vr: *const c_void, ldvr: *mut c_int, s: *mut c_float, sep: *mut c_float, mm: *mut c_int, m: *mut c_int, work: *mut c_void, ldwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn ztrsna_(job: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, t: *const c_void, ldt: *mut c_int, vl: *const c_void, ldvl: *mut c_int, vr: *const c_void, ldvr: *mut c_int, s: *mut c_double, sep: *mut c_double, mm: *mut c_int, m: *mut c_int, work: *mut c_void, ldwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn strexc_(compq: *mut c_char, n: *mut c_int, t: *mut c_float, ldt: *mut c_int, q: *mut c_float, ldq: *mut c_int, ifst: *mut c_int, ilst: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dtrexc_(compq: *mut c_char, n: *mut c_int, t: *mut c_double, ldt: *mut c_int, q: *mut c_double, ldq: *mut c_int, ifst: *mut c_int, ilst: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn ctrexc_(compq: *mut c_char, n: *mut c_int, t: *mut c_void, ldt: *mut c_int, q: *mut c_void, ldq: *mut c_int, ifst: *mut c_int, ilst: *mut c_int, info: *mut c_int);
    pub fn ztrexc_(compq: *mut c_char, n: *mut c_int, t: *mut c_void, ldt: *mut c_int, q: *mut c_void, ldq: *mut c_int, ifst: *mut c_int, ilst: *mut c_int, info: *mut c_int);

    pub fn strsen_(job: *mut c_char, compq: *mut c_char, select: *const c_int, n: *mut c_int, t: *mut c_float, ldt: *mut c_int, q: *mut c_float, ldq: *mut c_int, wr: *mut c_float, wi: *mut c_float, m: *mut c_int, s: *mut c_float, sep: *mut c_float, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dtrsen_(job: *mut c_char, compq: *mut c_char, select: *const c_int, n: *mut c_int, t: *mut c_double, ldt: *mut c_int, q: *mut c_double, ldq: *mut c_int, wr: *mut c_double, wi: *mut c_double, m: *mut c_int, s: *mut c_double, sep: *mut c_double, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn ctrsen_(job: *mut c_char, compq: *mut c_char, select: *const c_int, n: *mut c_int, t: *mut c_void, ldt: *mut c_int, q: *mut c_void, ldq: *mut c_int, w: *mut c_void, m: *mut c_int, s: *mut c_float, sep: *mut c_float, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn ztrsen_(job: *mut c_char, compq: *mut c_char, select: *const c_int, n: *mut c_int, t: *mut c_void, ldt: *mut c_int, q: *mut c_void, ldq: *mut c_int, w: *mut c_void, m: *mut c_int, s: *mut c_double, sep: *mut c_double, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn strsyl_(trana: *mut c_char, tranb: *mut c_char, isgn: *mut c_int, m: *mut c_int, n: *mut c_int, a: *const c_float, lda: *mut c_int, b: *const c_float, ldb: *mut c_int, c: *mut c_float, ldc: *mut c_int, scale: *mut c_float, info: *mut c_int);
    pub fn dtrsyl_(trana: *mut c_char, tranb: *mut c_char, isgn: *mut c_int, m: *mut c_int, n: *mut c_int, a: *const c_double, lda: *mut c_int, b: *const c_double, ldb: *mut c_int, c: *mut c_double, ldc: *mut c_int, scale: *mut c_double, info: *mut c_int);
    pub fn ctrsyl_(trana: *mut c_char, tranb: *mut c_char, isgn: *mut c_int, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, b: *const c_void, ldb: *mut c_int, c: *mut c_void, ldc: *mut c_int, scale: *mut c_float, info: *mut c_int);
    pub fn ztrsyl_(trana: *mut c_char, tranb: *mut c_char, isgn: *mut c_int, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, b: *const c_void, ldb: *mut c_int, c: *mut c_void, ldc: *mut c_int, scale: *mut c_double, info: *mut c_int);

    pub fn sgghrd_(compq: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, q: *mut c_float, ldq: *mut c_int, z: *mut c_float, ldz: *mut c_int, info: *mut c_int);
    pub fn dgghrd_(compq: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, q: *mut c_double, ldq: *mut c_int, z: *mut c_double, ldz: *mut c_int, info: *mut c_int);
    pub fn cgghrd_(compq: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, q: *mut c_void, ldq: *mut c_int, z: *mut c_void, ldz: *mut c_int, info: *mut c_int);
    pub fn zgghrd_(compq: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, q: *mut c_void, ldq: *mut c_int, z: *mut c_void, ldz: *mut c_int, info: *mut c_int);

    pub fn sggbal_(job: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn dggbal_(job: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double, work: *mut c_double, info: *mut c_int);
    pub fn cggbal_(job: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn zggbal_(job: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double, work: *mut c_double, info: *mut c_int);

    pub fn sggbak_(job: *mut c_char, side: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *const c_float, rscale: *const c_float, m: *mut c_int, v: *mut c_float, ldv: *mut c_int, info: *mut c_int);
    pub fn dggbak_(job: *mut c_char, side: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *const c_double, rscale: *const c_double, m: *mut c_int, v: *mut c_double, ldv: *mut c_int, info: *mut c_int);
    pub fn cggbak_(job: *mut c_char, side: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *const c_float, rscale: *const c_float, m: *mut c_int, v: *mut c_void, ldv: *mut c_int, info: *mut c_int);
    pub fn zggbak_(job: *mut c_char, side: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *const c_double, rscale: *const c_double, m: *mut c_int, v: *mut c_void, ldv: *mut c_int, info: *mut c_int);

    pub fn shgeqz_(job: *mut c_char, compq: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, h: *mut c_float, ldh: *mut c_int, t: *mut c_float, ldt: *mut c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, q: *mut c_float, ldq: *mut c_int, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dhgeqz_(job: *mut c_char, compq: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, h: *mut c_double, ldh: *mut c_int, t: *mut c_double, ldt: *mut c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, q: *mut c_double, ldq: *mut c_int, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn chgeqz_(job: *mut c_char, compq: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, h: *mut c_void, ldh: *mut c_int, t: *mut c_void, ldt: *mut c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: *mut c_int, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zhgeqz_(job: *mut c_char, compq: *mut c_char, compz: *mut c_char, n: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, h: *mut c_void, ldh: *mut c_int, t: *mut c_void, ldt: *mut c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: *mut c_int, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn stgevc_(side: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, s: *const c_float, lds: *mut c_int, p: *const c_float, ldp: *mut c_int, vl: *mut c_float, ldvl: *mut c_int, vr: *mut c_float, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dtgevc_(side: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, s: *const c_double, lds: *mut c_int, p: *const c_double, ldp: *mut c_int, vl: *mut c_double, ldvl: *mut c_int, vr: *mut c_double, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn ctgevc_(side: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, s: *const c_void, lds: *mut c_int, p: *const c_void, ldp: *mut c_int, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn ztgevc_(side: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, s: *const c_void, lds: *mut c_int, p: *const c_void, ldp: *mut c_int, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, mm: *mut c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn stgexc_(wantq: *mut c_int, wantz: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, q: *mut c_float, ldq: *mut c_int, z: *mut c_float, ldz: *mut c_int, ifst: *mut c_int, ilst: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dtgexc_(wantq: *mut c_int, wantz: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, q: *mut c_double, ldq: *mut c_int, z: *mut c_double, ldz: *mut c_int, ifst: *mut c_int, ilst: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn ctgexc_(wantq: *mut c_int, wantz: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, q: *mut c_void, ldq: *mut c_int, z: *mut c_void, ldz: *mut c_int, ifst: *mut c_int, ilst: *mut c_int, info: *mut c_int);
    pub fn ztgexc_(wantq: *mut c_int, wantz: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, q: *mut c_void, ldq: *mut c_int, z: *mut c_void, ldz: *mut c_int, ifst: *mut c_int, ilst: *mut c_int, info: *mut c_int);

    pub fn stgsen_(ijob: *mut c_int, wantq: *mut c_int, wantz: *mut c_int, select: *const c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, q: *mut c_float, ldq: *mut c_int, z: *mut c_float, ldz: *mut c_int, m: *mut c_int, pl: *mut c_float, pr: *mut c_float, dif: *mut c_float, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dtgsen_(ijob: *mut c_int, wantq: *mut c_int, wantz: *mut c_int, select: *const c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, q: *mut c_double, ldq: *mut c_int, z: *mut c_double, ldz: *mut c_int, m: *mut c_int, pl: *mut c_double, pr: *mut c_double, dif: *mut c_double, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn ctgsen_(ijob: *mut c_int, wantq: *mut c_int, wantz: *mut c_int, select: *const c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: *mut c_int, z: *mut c_void, ldz: *mut c_int, m: *mut c_int, pl: *mut c_float, pr: *mut c_float, dif: *mut c_float, work: *mut c_void, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn ztgsen_(ijob: *mut c_int, wantq: *mut c_int, wantz: *mut c_int, select: *const c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: *mut c_int, z: *mut c_void, ldz: *mut c_int, m: *mut c_int, pl: *mut c_double, pr: *mut c_double, dif: *mut c_double, work: *mut c_void, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn stgsyl_(trans: *mut c_char, ijob: *mut c_int, m: *mut c_int, n: *mut c_int, a: *const c_float, lda: *mut c_int, b: *const c_float, ldb: *mut c_int, c: *mut c_float, ldc: *mut c_int, d: *const c_float, ldd: *mut c_int, e: *const c_float, lde: *mut c_int, f: *mut c_float, ldf: *mut c_int, scale: *mut c_float, dif: *mut c_float, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn dtgsyl_(trans: *mut c_char, ijob: *mut c_int, m: *mut c_int, n: *mut c_int, a: *const c_double, lda: *mut c_int, b: *const c_double, ldb: *mut c_int, c: *mut c_double, ldc: *mut c_int, d: *const c_double, ldd: *mut c_int, e: *const c_double, lde: *mut c_int, f: *mut c_double, ldf: *mut c_int, scale: *mut c_double, dif: *mut c_double, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn ctgsyl_(trans: *mut c_char, ijob: *mut c_int, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, b: *const c_void, ldb: *mut c_int, c: *mut c_void, ldc: *mut c_int, d: *const c_void, ldd: *mut c_int, e: *const c_void, lde: *mut c_int, f: *mut c_void, ldf: *mut c_int, scale: *mut c_float, dif: *mut c_float, work: *mut c_void, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn ztgsyl_(trans: *mut c_char, ijob: *mut c_int, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, b: *const c_void, ldb: *mut c_int, c: *mut c_void, ldc: *mut c_int, d: *const c_void, ldd: *mut c_int, e: *const c_void, lde: *mut c_int, f: *mut c_void, ldf: *mut c_int, scale: *mut c_double, dif: *mut c_double, work: *mut c_void, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);

    pub fn stgsna_(job: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, a: *const c_float, lda: *mut c_int, b: *const c_float, ldb: *mut c_int, vl: *const c_float, ldvl: *mut c_int, vr: *const c_float, ldvr: *mut c_int, s: *mut c_float, dif: *mut c_float, mm: *mut c_int, m: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn dtgsna_(job: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, a: *const c_double, lda: *mut c_int, b: *const c_double, ldb: *mut c_int, vl: *const c_double, ldvl: *mut c_int, vr: *const c_double, ldvr: *mut c_int, s: *mut c_double, dif: *mut c_double, mm: *mut c_int, m: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn ctgsna_(job: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, b: *const c_void, ldb: *mut c_int, vl: *const c_void, ldvl: *mut c_int, vr: *const c_void, ldvr: *mut c_int, s: *mut c_float, dif: *mut c_float, mm: *mut c_int, m: *mut c_int, work: *mut c_void, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn ztgsna_(job: *mut c_char, howmny: *mut c_char, select: *const c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, b: *const c_void, ldb: *mut c_int, vl: *const c_void, ldvl: *mut c_int, vr: *const c_void, ldvr: *mut c_int, s: *mut c_double, dif: *mut c_double, mm: *mut c_int, m: *mut c_int, work: *mut c_void, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);

    pub fn sggsvp_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, p: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, tola: *mut c_float, tolb: *mut c_float, k: *mut c_int, l: *mut c_int, u: *mut c_float, ldu: *mut c_int, v: *mut c_float, ldv: *mut c_int, q: *mut c_float, ldq: *mut c_int, iwork: *mut c_int, tau: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn dggsvp_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, p: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, tola: *mut c_double, tolb: *mut c_double, k: *mut c_int, l: *mut c_int, u: *mut c_double, ldu: *mut c_int, v: *mut c_double, ldv: *mut c_int, q: *mut c_double, ldq: *mut c_int, iwork: *mut c_int, tau: *mut c_double, work: *mut c_double, info: *mut c_int);
    pub fn cggsvp_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, p: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, tola: *mut c_float, tolb: *mut c_float, k: *mut c_int, l: *mut c_int, u: *mut c_void, ldu: *mut c_int, v: *mut c_void, ldv: *mut c_int, q: *mut c_void, ldq: *mut c_int, iwork: *mut c_int, rwork: *mut c_float, tau: *mut c_void, work: *mut c_void, info: *mut c_int);
    pub fn zggsvp_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, p: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, tola: *mut c_double, tolb: *mut c_double, k: *mut c_int, l: *mut c_int, u: *mut c_void, ldu: *mut c_int, v: *mut c_void, ldv: *mut c_int, q: *mut c_void, ldq: *mut c_int, iwork: *mut c_int, rwork: *mut c_double, tau: *mut c_void, work: *mut c_void, info: *mut c_int);

    pub fn stgsja_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, p: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, tola: *mut c_float, tolb: *mut c_float, alpha: *mut c_float, beta: *mut c_float, u: *mut c_float, ldu: *mut c_int, v: *mut c_float, ldv: *mut c_int, q: *mut c_float, ldq: *mut c_int, work: *mut c_float, ncycle: *mut c_int, info: *mut c_int);
    pub fn dtgsja_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, p: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, tola: *mut c_double, tolb: *mut c_double, alpha: *mut c_double, beta: *mut c_double, u: *mut c_double, ldu: *mut c_int, v: *mut c_double, ldv: *mut c_int, q: *mut c_double, ldq: *mut c_int, work: *mut c_double, ncycle: *mut c_int, info: *mut c_int);
    pub fn ctgsja_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, p: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, tola: *mut c_float, tolb: *mut c_float, alpha: *mut c_float, beta: *mut c_float, u: *mut c_void, ldu: *mut c_int, v: *mut c_void, ldv: *mut c_int, q: *mut c_void, ldq: *mut c_int, work: *mut c_void, ncycle: *mut c_int, info: *mut c_int);
    pub fn ztgsja_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, p: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, tola: *mut c_double, tolb: *mut c_double, alpha: *mut c_double, beta: *mut c_double, u: *mut c_void, ldu: *mut c_int, v: *mut c_void, ldv: *mut c_int, q: *mut c_void, ldq: *mut c_int, work: *mut c_void, ncycle: *mut c_int, info: *mut c_int);

    pub fn sgels_(trans: *mut c_char, m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgels_(trans: *mut c_char, m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgels_(trans: *mut c_char, m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zgels_(trans: *mut c_char, m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sgelsy_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, jpvt: *mut c_int, rcond: *mut c_float, rank: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgelsy_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, jpvt: *mut c_int, rcond: *mut c_double, rank: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgelsy_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, jpvt: *mut c_int, rcond: *mut c_float, rank: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zgelsy_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, jpvt: *mut c_int, rcond: *mut c_double, rank: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn sgelss_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, s: *mut c_float, rcond: *mut c_float, rank: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgelss_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, s: *mut c_double, rcond: *mut c_double, rank: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgelss_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, s: *mut c_float, rcond: *mut c_float, rank: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zgelss_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, s: *mut c_double, rcond: *mut c_double, rank: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn sgelsd_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, s: *mut c_float, rcond: *mut c_float, rank: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn dgelsd_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, s: *mut c_double, rcond: *mut c_double, rank: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn cgelsd_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, s: *mut c_float, rcond: *mut c_float, rank: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn zgelsd_(m: *mut c_int, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, s: *mut c_double, rcond: *mut c_double, rank: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, iwork: *mut c_int, info: *mut c_int);

    pub fn sgglse_(m: *mut c_int, n: *mut c_int, p: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, c: *mut c_float, d: *mut c_float, x: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgglse_(m: *mut c_int, n: *mut c_int, p: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, c: *mut c_double, d: *mut c_double, x: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgglse_(m: *mut c_int, n: *mut c_int, p: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, c: *mut c_void, d: *mut c_void, x: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zgglse_(m: *mut c_int, n: *mut c_int, p: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, c: *mut c_void, d: *mut c_void, x: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn sggglm_(n: *mut c_int, m: *mut c_int, p: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, d: *mut c_float, x: *mut c_float, y: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dggglm_(n: *mut c_int, m: *mut c_int, p: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, d: *mut c_double, x: *mut c_double, y: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cggglm_(n: *mut c_int, m: *mut c_int, p: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, d: *mut c_void, x: *mut c_void, y: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zggglm_(n: *mut c_int, m: *mut c_int, p: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, d: *mut c_void, x: *mut c_void, y: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn ssyev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, w: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dsyev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, w: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cheev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, w: *mut c_float, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zheev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, w: *mut c_double, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn ssyevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, w: *mut c_float, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dsyevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, w: *mut c_double, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn cheevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, w: *mut c_float, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn zheevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, w: *mut c_double, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn ssyevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn dsyevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn cheevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn zheevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);

    pub fn ssyevr_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, isuppz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dsyevr_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, isuppz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn cheevr_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, isuppz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn zheevr_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, isuppz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn sspev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dspev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn chpev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zhpev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sspevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dspevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn chpevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn zhpevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn sspevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn dspevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn chpevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn zhpevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);

    pub fn ssbev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_float, ldab: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dsbev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_double, ldab: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn chbev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_void, ldab: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zhbev_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_void, ldab: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn ssbevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_float, ldab: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dsbevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_double, ldab: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn chbevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_void, ldab: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn zhbevd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_void, ldab: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn ssbevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_float, ldab: *mut c_int, q: *mut c_float, ldq: *mut c_int, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn dsbevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_double, ldab: *mut c_int, q: *mut c_double, ldq: *mut c_int, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn chbevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_void, ldab: *mut c_int, q: *mut c_void, ldq: *mut c_int, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn zhbevx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, kd: *mut c_int, ab: *mut c_void, ldab: *mut c_int, q: *mut c_void, ldq: *mut c_int, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);

    pub fn sstev_(jobz: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dstev_(jobz: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, info: *mut c_int);

    pub fn sstevd_(jobz: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dstevd_(jobz: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn sstevx_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn dstevx_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);

    pub fn sstevr_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_float, e: *mut c_float, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, isuppz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dstevr_(jobz: *mut c_char, range: *mut c_char, n: *mut c_int, d: *mut c_double, e: *mut c_double, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, isuppz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn sgees_(jobvs: *mut c_char, sort: *mut c_char, select: LAPACK_S_SELECT2, n: *mut c_int, a: *mut c_float, lda: *mut c_int, sdim: *mut c_int, wr: *mut c_float, wi: *mut c_float, vs: *mut c_float, ldvs: *mut c_int, work: *mut c_float, lwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn dgees_(jobvs: *mut c_char, sort: *mut c_char, select: LAPACK_D_SELECT2, n: *mut c_int, a: *mut c_double, lda: *mut c_int, sdim: *mut c_int, wr: *mut c_double, wi: *mut c_double, vs: *mut c_double, ldvs: *mut c_int, work: *mut c_double, lwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn cgees_(jobvs: *mut c_char, sort: *mut c_char, select: LAPACK_C_SELECT1, n: *mut c_int, a: *mut c_void, lda: *mut c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, bwork: *mut c_int, info: *mut c_int);
    pub fn zgees_(jobvs: *mut c_char, sort: *mut c_char, select: LAPACK_Z_SELECT1, n: *mut c_int, a: *mut c_void, lda: *mut c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, bwork: *mut c_int, info: *mut c_int);

    pub fn sgeesx_(jobvs: *mut c_char, sort: *mut c_char, select: LAPACK_S_SELECT2, sense: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, sdim: *mut c_int, wr: *mut c_float, wi: *mut c_float, vs: *mut c_float, ldvs: *mut c_int, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn dgeesx_(jobvs: *mut c_char, sort: *mut c_char, select: LAPACK_D_SELECT2, sense: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, sdim: *mut c_int, wr: *mut c_double, wi: *mut c_double, vs: *mut c_double, ldvs: *mut c_int, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn cgeesx_(jobvs: *mut c_char, sort: *mut c_char, select: LAPACK_C_SELECT1, sense: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: *mut c_int, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, bwork: *mut c_int, info: *mut c_int);
    pub fn zgeesx_(jobvs: *mut c_char, sort: *mut c_char, select: LAPACK_Z_SELECT1, sense: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: *mut c_int, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, bwork: *mut c_int, info: *mut c_int);

    pub fn sgeev_(jobvl: *mut c_char, jobvr: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, wr: *mut c_float, wi: *mut c_float, vl: *mut c_float, ldvl: *mut c_int, vr: *mut c_float, ldvr: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgeev_(jobvl: *mut c_char, jobvr: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, wr: *mut c_double, wi: *mut c_double, vl: *mut c_double, ldvl: *mut c_int, vr: *mut c_double, ldvr: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgeev_(jobvl: *mut c_char, jobvr: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, w: *mut c_void, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zgeev_(jobvl: *mut c_char, jobvr: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, w: *mut c_void, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn sgeevx_(balanc: *mut c_char, jobvl: *mut c_char, jobvr: *mut c_char, sense: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, wr: *mut c_float, wi: *mut c_float, vl: *mut c_float, ldvl: *mut c_int, vr: *mut c_float, ldvr: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float, abnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn dgeevx_(balanc: *mut c_char, jobvl: *mut c_char, jobvr: *mut c_char, sense: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, wr: *mut c_double, wi: *mut c_double, vl: *mut c_double, ldvl: *mut c_int, vr: *mut c_double, ldvr: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double, abnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn cgeevx_(balanc: *mut c_char, jobvl: *mut c_char, jobvr: *mut c_char, sense: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, w: *mut c_void, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float, abnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zgeevx_(balanc: *mut c_char, jobvl: *mut c_char, jobvr: *mut c_char, sense: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, w: *mut c_void, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double, abnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn sgesvd_(jobu: *mut c_char, jobvt: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, s: *mut c_float, u: *mut c_float, ldu: *mut c_int, vt: *mut c_float, ldvt: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgesvd_(jobu: *mut c_char, jobvt: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, s: *mut c_double, u: *mut c_double, ldu: *mut c_int, vt: *mut c_double, ldvt: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgesvd_(jobu: *mut c_char, jobvt: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, s: *mut c_float, u: *mut c_void, ldu: *mut c_int, vt: *mut c_void, ldvt: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zgesvd_(jobu: *mut c_char, jobvt: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, s: *mut c_double, u: *mut c_void, ldu: *mut c_int, vt: *mut c_void, ldvt: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn sgesdd_(jobz: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, s: *mut c_float, u: *mut c_float, ldu: *mut c_int, vt: *mut c_float, ldvt: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn dgesdd_(jobz: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, s: *mut c_double, u: *mut c_double, ldu: *mut c_int, vt: *mut c_double, ldvt: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn cgesdd_(jobz: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, s: *mut c_float, u: *mut c_void, ldu: *mut c_int, vt: *mut c_void, ldvt: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn zgesdd_(jobz: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, s: *mut c_double, u: *mut c_void, ldu: *mut c_int, vt: *mut c_void, ldvt: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, iwork: *mut c_int, info: *mut c_int);

    pub fn dgejsv_(joba: *mut c_char, jobu: *mut c_char, jobv: *mut c_char, jobr: *mut c_char, jobt: *mut c_char, jobp: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, sva: *mut c_double, u: *mut c_double, ldu: *mut c_int, v: *mut c_double, ldv: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn sgejsv_(joba: *mut c_char, jobu: *mut c_char, jobv: *mut c_char, jobr: *mut c_char, jobt: *mut c_char, jobp: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, sva: *mut c_float, u: *mut c_float, ldu: *mut c_int, v: *mut c_float, ldv: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);

    pub fn dgesvj_(joba: *mut c_char, jobu: *mut c_char, jobv: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, sva: *mut c_double, mv: *mut c_int, v: *mut c_double, ldv: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn sgesvj_(joba: *mut c_char, jobu: *mut c_char, jobv: *mut c_char, m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, sva: *mut c_float, mv: *mut c_int, v: *mut c_float, ldv: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);

    pub fn sggsvd_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, n: *mut c_int, p: *mut c_int, k: *mut c_int, l: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, alpha: *mut c_float, beta: *mut c_float, u: *mut c_float, ldu: *mut c_int, v: *mut c_float, ldv: *mut c_int, q: *mut c_float, ldq: *mut c_int, work: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn dggsvd_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, n: *mut c_int, p: *mut c_int, k: *mut c_int, l: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, alpha: *mut c_double, beta: *mut c_double, u: *mut c_double, ldu: *mut c_int, v: *mut c_double, ldv: *mut c_int, q: *mut c_double, ldq: *mut c_int, work: *mut c_double, iwork: *mut c_int, info: *mut c_int);
    pub fn cggsvd_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, n: *mut c_int, p: *mut c_int, k: *mut c_int, l: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, alpha: *mut c_float, beta: *mut c_float, u: *mut c_void, ldu: *mut c_int, v: *mut c_void, ldv: *mut c_int, q: *mut c_void, ldq: *mut c_int, work: *mut c_void, rwork: *mut c_float, iwork: *mut c_int, info: *mut c_int);
    pub fn zggsvd_(jobu: *mut c_char, jobv: *mut c_char, jobq: *mut c_char, m: *mut c_int, n: *mut c_int, p: *mut c_int, k: *mut c_int, l: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, alpha: *mut c_double, beta: *mut c_double, u: *mut c_void, ldu: *mut c_int, v: *mut c_void, ldv: *mut c_int, q: *mut c_void, ldq: *mut c_int, work: *mut c_void, rwork: *mut c_double, iwork: *mut c_int, info: *mut c_int);

    pub fn ssygv_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, w: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dsygv_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, w: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn chegv_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, w: *mut c_float, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zhegv_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, w: *mut c_double, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn ssygvd_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, w: *mut c_float, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dsygvd_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, w: *mut c_double, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn chegvd_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, w: *mut c_float, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn zhegvd_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, w: *mut c_double, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn ssygvx_(itype: *mut c_int, jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn dsygvx_(itype: *mut c_int, jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn chegvx_(itype: *mut c_int, jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn zhegvx_(itype: *mut c_int, jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);

    pub fn sspgv_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dspgv_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, bp: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn chpgv_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zhpgv_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn sspgvd_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dspgvd_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, bp: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn chpgvd_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn zhpgvd_(itype: *mut c_int, jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn sspgvx_(itype: *mut c_int, jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_float, bp: *mut c_float, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn dspgvx_(itype: *mut c_int, jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_double, bp: *mut c_double, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn chpgvx_(itype: *mut c_int, jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, bp: *mut c_void, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn zhpgvx_(itype: *mut c_int, jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *mut c_void, bp: *mut c_void, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);

    pub fn ssbgv_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_float, ldab: *mut c_int, bb: *mut c_float, ldbb: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dsbgv_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_double, ldab: *mut c_int, bb: *mut c_double, ldbb: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn chbgv_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_void, ldab: *mut c_int, bb: *mut c_void, ldbb: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_float, info: *mut c_int);
    pub fn zhbgv_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_void, ldab: *mut c_int, bb: *mut c_void, ldbb: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_double, info: *mut c_int);

    pub fn ssbgvd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_float, ldab: *mut c_int, bb: *mut c_float, ldbb: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn dsbgvd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_double, ldab: *mut c_int, bb: *mut c_double, ldbb: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn chbgvd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_void, ldab: *mut c_int, bb: *mut c_void, ldbb: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);
    pub fn zhbgvd_(jobz: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_void, ldab: *mut c_int, bb: *mut c_void, ldbb: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, lrwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, info: *mut c_int);

    pub fn ssbgvx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_float, ldab: *mut c_int, bb: *mut c_float, ldbb: *mut c_int, q: *mut c_float, ldq: *mut c_int, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: *mut c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn dsbgvx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_double, ldab: *mut c_int, bb: *mut c_double, ldbb: *mut c_int, q: *mut c_double, ldq: *mut c_int, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: *mut c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn chbgvx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_void, ldab: *mut c_int, bb: *mut c_void, ldbb: *mut c_int, q: *mut c_void, ldq: *mut c_int, vl: *mut c_float, vu: *mut c_float, il: *mut c_int, iu: *mut c_int, abstol: *mut c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);
    pub fn zhbgvx_(jobz: *mut c_char, range: *mut c_char, uplo: *mut c_char, n: *mut c_int, ka: *mut c_int, kb: *mut c_int, ab: *mut c_void, ldab: *mut c_int, bb: *mut c_void, ldbb: *mut c_int, q: *mut c_void, ldq: *mut c_int, vl: *mut c_double, vu: *mut c_double, il: *mut c_int, iu: *mut c_int, abstol: *mut c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: *mut c_int, work: *mut c_void, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int, info: *mut c_int);

    pub fn sgges_(jobvsl: *mut c_char, jobvsr: *mut c_char, sort: *mut c_char, selctg: LAPACK_S_SELECT3, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, sdim: *mut c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vsl: *mut c_float, ldvsl: *mut c_int, vsr: *mut c_float, ldvsr: *mut c_int, work: *mut c_float, lwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn dgges_(jobvsl: *mut c_char, jobvsr: *mut c_char, sort: *mut c_char, selctg: LAPACK_D_SELECT3, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, sdim: *mut c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vsl: *mut c_double, ldvsl: *mut c_int, vsr: *mut c_double, ldvsr: *mut c_int, work: *mut c_double, lwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn cgges_(jobvsl: *mut c_char, jobvsr: *mut c_char, sort: *mut c_char, selctg: LAPACK_C_SELECT2, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: *mut c_int, vsr: *mut c_void, ldvsr: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, bwork: *mut c_int, info: *mut c_int);
    pub fn zgges_(jobvsl: *mut c_char, jobvsr: *mut c_char, sort: *mut c_char, selctg: LAPACK_Z_SELECT2, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: *mut c_int, vsr: *mut c_void, ldvsr: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, bwork: *mut c_int, info: *mut c_int);

    pub fn sggesx_(jobvsl: *mut c_char, jobvsr: *mut c_char, sort: *mut c_char, selctg: LAPACK_S_SELECT3, sense: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, sdim: *mut c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vsl: *mut c_float, ldvsl: *mut c_int, vsr: *mut c_float, ldvsr: *mut c_int, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn dggesx_(jobvsl: *mut c_char, jobvsr: *mut c_char, sort: *mut c_char, selctg: LAPACK_D_SELECT3, sense: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, sdim: *mut c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vsl: *mut c_double, ldvsl: *mut c_int, vsr: *mut c_double, ldvsr: *mut c_int, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, liwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn cggesx_(jobvsl: *mut c_char, jobvsr: *mut c_char, sort: *mut c_char, selctg: LAPACK_C_SELECT2, sense: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: *mut c_int, vsr: *mut c_void, ldvsr: *mut c_int, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, iwork: *mut c_int, liwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn zggesx_(jobvsl: *mut c_char, jobvsr: *mut c_char, sort: *mut c_char, selctg: LAPACK_Z_SELECT2, sense: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: *mut c_int, vsr: *mut c_void, ldvsr: *mut c_int, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, iwork: *mut c_int, liwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);

    pub fn sggev_(jobvl: *mut c_char, jobvr: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vl: *mut c_float, ldvl: *mut c_int, vr: *mut c_float, ldvr: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dggev_(jobvl: *mut c_char, jobvr: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vl: *mut c_double, ldvl: *mut c_int, vr: *mut c_double, ldvr: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cggev_(jobvl: *mut c_char, jobvr: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, info: *mut c_int);
    pub fn zggev_(jobvl: *mut c_char, jobvr: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, info: *mut c_int);

    pub fn sggevx_(balanc: *mut c_char, jobvl: *mut c_char, jobvr: *mut c_char, sense: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vl: *mut c_float, ldvl: *mut c_int, vr: *mut c_float, ldvr: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float, abnrm: *mut c_float, bbnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn dggevx_(balanc: *mut c_char, jobvl: *mut c_char, jobvr: *mut c_char, sense: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vl: *mut c_double, ldvl: *mut c_int, vr: *mut c_double, ldvr: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double, abnrm: *mut c_double, bbnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn cggevx_(balanc: *mut c_char, jobvl: *mut c_char, jobvr: *mut c_char, sense: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float, abnrm: *mut c_float, bbnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, iwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);
    pub fn zggevx_(balanc: *mut c_char, jobvl: *mut c_char, jobvr: *mut c_char, sense: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: *mut c_int, vr: *mut c_void, ldvr: *mut c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double, abnrm: *mut c_double, bbnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, iwork: *mut c_int, bwork: *mut c_int, info: *mut c_int);

    pub fn ssfrk_(transr: *mut c_char, uplo: *mut c_char, trans: *mut c_char, n: *mut c_int, k: *mut c_int, alpha: *mut c_float, a: *const c_float, lda: *mut c_int, beta: *mut c_float, c: *mut c_float);
    pub fn dsfrk_(transr: *mut c_char, uplo: *mut c_char, trans: *mut c_char, n: *mut c_int, k: *mut c_int, alpha: *mut c_double, a: *const c_double, lda: *mut c_int, beta: *mut c_double, c: *mut c_double);
    pub fn chfrk_(transr: *mut c_char, uplo: *mut c_char, trans: *mut c_char, n: *mut c_int, k: *mut c_int, alpha: *mut c_float, a: *const c_void, lda: *mut c_int, beta: *mut c_float, c: *mut c_void);
    pub fn zhfrk_(transr: *mut c_char, uplo: *mut c_char, trans: *mut c_char, n: *mut c_int, k: *mut c_int, alpha: *mut c_double, a: *const c_void, lda: *mut c_int, beta: *mut c_double, c: *mut c_void);

    pub fn stfsm_(transr: *mut c_char, side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, m: *mut c_int, n: *mut c_int, alpha: *mut c_float, a: *const c_float, b: *mut c_float, ldb: *mut c_int);
    pub fn dtfsm_(transr: *mut c_char, side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, m: *mut c_int, n: *mut c_int, alpha: *mut c_double, a: *const c_double, b: *mut c_double, ldb: *mut c_int);
    pub fn ctfsm_(transr: *mut c_char, side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, m: *mut c_int, n: *mut c_int, alpha: *mut c_void, a: *const c_void, b: *mut c_void, ldb: *mut c_int);
    pub fn ztfsm_(transr: *mut c_char, side: *mut c_char, uplo: *mut c_char, trans: *mut c_char, diag: *mut c_char, m: *mut c_int, n: *mut c_int, alpha: *mut c_void, a: *const c_void, b: *mut c_void, ldb: *mut c_int);

    pub fn stfttp_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, arf: *const c_float, ap: *mut c_float, info: *mut c_int);
    pub fn dtfttp_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, arf: *const c_double, ap: *mut c_double, info: *mut c_int);
    pub fn ctfttp_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, arf: *const c_void, ap: *mut c_void, info: *mut c_int);
    pub fn ztfttp_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, arf: *const c_void, ap: *mut c_void, info: *mut c_int);

    pub fn stfttr_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, arf: *const c_float, a: *mut c_float, lda: *mut c_int, info: *mut c_int);
    pub fn dtfttr_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, arf: *const c_double, a: *mut c_double, lda: *mut c_int, info: *mut c_int);
    pub fn ctfttr_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, arf: *const c_void, a: *mut c_void, lda: *mut c_int, info: *mut c_int);
    pub fn ztfttr_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, arf: *const c_void, a: *mut c_void, lda: *mut c_int, info: *mut c_int);

    pub fn stpttf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *const c_float, arf: *mut c_float, info: *mut c_int);
    pub fn dtpttf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *const c_double, arf: *mut c_double, info: *mut c_int);
    pub fn ctpttf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *const c_void, arf: *mut c_void, info: *mut c_int);
    pub fn ztpttf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, ap: *const c_void, arf: *mut c_void, info: *mut c_int);

    pub fn stpttr_(uplo: *mut c_char, n: *mut c_int, ap: *const c_float, a: *mut c_float, lda: *mut c_int, info: *mut c_int);
    pub fn dtpttr_(uplo: *mut c_char, n: *mut c_int, ap: *const c_double, a: *mut c_double, lda: *mut c_int, info: *mut c_int);
    pub fn ctpttr_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, a: *mut c_void, lda: *mut c_int, info: *mut c_int);
    pub fn ztpttr_(uplo: *mut c_char, n: *mut c_int, ap: *const c_void, a: *mut c_void, lda: *mut c_int, info: *mut c_int);

    pub fn strttf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *const c_float, lda: *mut c_int, arf: *mut c_float, info: *mut c_int);
    pub fn dtrttf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *const c_double, lda: *mut c_int, arf: *mut c_double, info: *mut c_int);
    pub fn ctrttf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, arf: *mut c_void, info: *mut c_int);
    pub fn ztrttf_(transr: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, arf: *mut c_void, info: *mut c_int);

    pub fn strttp_(uplo: *mut c_char, n: *mut c_int, a: *const c_float, lda: *mut c_int, ap: *mut c_float, info: *mut c_int);
    pub fn dtrttp_(uplo: *mut c_char, n: *mut c_int, a: *const c_double, lda: *mut c_int, ap: *mut c_double, info: *mut c_int);
    pub fn ctrttp_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, ap: *mut c_void, info: *mut c_int);
    pub fn ztrttp_(uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, ap: *mut c_void, info: *mut c_int);

    pub fn sgeqrfp_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dgeqrfp_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn cgeqrfp_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zgeqrfp_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn clacgv_(n: *mut c_int, x: *mut c_void, incx: *mut c_int);
    pub fn zlacgv_(n: *mut c_int, x: *mut c_void, incx: *mut c_int);

    pub fn slarnv_(idist: *mut c_int, iseed: *mut c_int, n: *mut c_int, x: *mut c_float);
    pub fn dlarnv_(idist: *mut c_int, iseed: *mut c_int, n: *mut c_int, x: *mut c_double);
    pub fn clarnv_(idist: *mut c_int, iseed: *mut c_int, n: *mut c_int, x: *mut c_void);
    pub fn zlarnv_(idist: *mut c_int, iseed: *mut c_int, n: *mut c_int, x: *mut c_void);

    pub fn sgeqr2_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn dgeqr2_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *mut c_double, work: *mut c_double, info: *mut c_int);
    pub fn cgeqr2_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, info: *mut c_int);
    pub fn zgeqr2_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, info: *mut c_int);

    pub fn slacn2_(n: *mut c_int, v: *mut c_float, x: *mut c_float, isgn: *mut c_int, est: *mut c_float, kase: *mut c_int, isave: *mut c_int);
    pub fn dlacn2_(n: *mut c_int, v: *mut c_double, x: *mut c_double, isgn: *mut c_int, est: *mut c_double, kase: *mut c_int, isave: *mut c_int);
    pub fn clacn2_(n: *mut c_int, v: *mut c_void, x: *mut c_void, est: *mut c_float, kase: *mut c_int, isave: *mut c_int);
    pub fn zlacn2_(n: *mut c_int, v: *mut c_void, x: *mut c_void, est: *mut c_double, kase: *mut c_int, isave: *mut c_int);

    pub fn slacpy_(uplo: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int);
    pub fn dlacpy_(uplo: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int);
    pub fn clacpy_(uplo: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int);
    pub fn zlacpy_(uplo: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int);

    pub fn clacp2_(uplo: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_float, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int);
    pub fn zlacp2_(uplo: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_double, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int);

    pub fn sgetf2_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, ipiv: *mut c_int, info: *mut c_int);
    pub fn dgetf2_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *mut c_int, info: *mut c_int);
    pub fn cgetf2_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, info: *mut c_int);
    pub fn zgetf2_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, info: *mut c_int);

    pub fn slaswp_(n: *mut c_int, a: *mut c_float, lda: *mut c_int, k1: *mut c_int, k2: *mut c_int, ipiv: *const c_int, incx: *mut c_int);
    pub fn dlaswp_(n: *mut c_int, a: *mut c_double, lda: *mut c_int, k1: *mut c_int, k2: *mut c_int, ipiv: *const c_int, incx: *mut c_int);
    pub fn claswp_(n: *mut c_int, a: *mut c_void, lda: *mut c_int, k1: *mut c_int, k2: *mut c_int, ipiv: *const c_int, incx: *mut c_int);
    pub fn zlaswp_(n: *mut c_int, a: *mut c_void, lda: *mut c_int, k1: *mut c_int, k2: *mut c_int, ipiv: *const c_int, incx: *mut c_int);

    pub fn slange_(norm: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_float, lda: *mut c_int, work: *mut c_float) -> c_float;
    pub fn dlange_(norm: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_double, lda: *mut c_int, work: *mut c_double) -> c_double;
    pub fn clange_(norm: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, work: *mut c_float) -> c_float;
    pub fn zlange_(norm: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, work: *mut c_double) -> c_double;

    pub fn clanhe_(norm: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, work: *mut c_float) -> c_float;
    pub fn zlanhe_(norm: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, work: *mut c_double) -> c_double;

    pub fn slansy_(norm: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *const c_float, lda: *mut c_int, work: *mut c_float) -> c_float;
    pub fn dlansy_(norm: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *const c_double, lda: *mut c_int, work: *mut c_double) -> c_double;
    pub fn clansy_(norm: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, work: *mut c_float) -> c_float;
    pub fn zlansy_(norm: *mut c_char, uplo: *mut c_char, n: *mut c_int, a: *const c_void, lda: *mut c_int, work: *mut c_double) -> c_double;

    pub fn slantr_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_float, lda: *mut c_int, work: *mut c_float) -> c_float;
    pub fn dlantr_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_double, lda: *mut c_int, work: *mut c_double) -> c_double;
    pub fn clantr_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, work: *mut c_float) -> c_float;
    pub fn zlantr_(norm: *mut c_char, uplo: *mut c_char, diag: *mut c_char, m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, work: *mut c_double) -> c_double;

    pub fn slamch_(cmach: *mut c_char) -> c_float;
    pub fn dlamch_(cmach: *mut c_char) -> c_double;

    pub fn sgelq2_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, tau: *mut c_float, work: *mut c_float, info: *mut c_int);
    pub fn dgelq2_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, tau: *mut c_double, work: *mut c_double, info: *mut c_int);
    pub fn cgelq2_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, info: *mut c_int);
    pub fn zgelq2_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, tau: *mut c_void, work: *mut c_void, info: *mut c_int);

    pub fn slarfb_(side: *mut c_char, trans: *mut c_char, direct: *mut c_char, storev: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, v: *const c_float, ldv: *mut c_int, t: *const c_float, ldt: *mut c_int, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, ldwork: *mut c_int);
    pub fn dlarfb_(side: *mut c_char, trans: *mut c_char, direct: *mut c_char, storev: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, v: *const c_double, ldv: *mut c_int, t: *const c_double, ldt: *mut c_int, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, ldwork: *mut c_int);
    pub fn clarfb_(side: *mut c_char, trans: *mut c_char, direct: *mut c_char, storev: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, v: *const c_void, ldv: *mut c_int, t: *const c_void, ldt: *mut c_int, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, ldwork: *mut c_int);
    pub fn zlarfb_(side: *mut c_char, trans: *mut c_char, direct: *mut c_char, storev: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, v: *const c_void, ldv: *mut c_int, t: *const c_void, ldt: *mut c_int, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, ldwork: *mut c_int);

    pub fn slarfg_(n: *mut c_int, alpha: *mut c_float, x: *mut c_float, incx: *mut c_int, tau: *mut c_float);
    pub fn dlarfg_(n: *mut c_int, alpha: *mut c_double, x: *mut c_double, incx: *mut c_int, tau: *mut c_double);
    pub fn clarfg_(n: *mut c_int, alpha: *mut c_void, x: *mut c_void, incx: *mut c_int, tau: *mut c_void);
    pub fn zlarfg_(n: *mut c_int, alpha: *mut c_void, x: *mut c_void, incx: *mut c_int, tau: *mut c_void);

    pub fn slarft_(direct: *mut c_char, storev: *mut c_char, n: *mut c_int, k: *mut c_int, v: *const c_float, ldv: *mut c_int, tau: *const c_float, t: *mut c_float, ldt: *mut c_int);
    pub fn dlarft_(direct: *mut c_char, storev: *mut c_char, n: *mut c_int, k: *mut c_int, v: *const c_double, ldv: *mut c_int, tau: *const c_double, t: *mut c_double, ldt: *mut c_int);
    pub fn clarft_(direct: *mut c_char, storev: *mut c_char, n: *mut c_int, k: *mut c_int, v: *const c_void, ldv: *mut c_int, tau: *const c_void, t: *mut c_void, ldt: *mut c_int);
    pub fn zlarft_(direct: *mut c_char, storev: *mut c_char, n: *mut c_int, k: *mut c_int, v: *const c_void, ldv: *mut c_int, tau: *const c_void, t: *mut c_void, ldt: *mut c_int);

    pub fn slarfx_(side: *mut c_char, m: *mut c_int, n: *mut c_int, v: *const c_float, tau: *mut c_float, c: *mut c_float, ldc: *mut c_int, work: *mut c_float);
    pub fn dlarfx_(side: *mut c_char, m: *mut c_int, n: *mut c_int, v: *const c_double, tau: *mut c_double, c: *mut c_double, ldc: *mut c_int, work: *mut c_double);
    pub fn clarfx_(side: *mut c_char, m: *mut c_int, n: *mut c_int, v: *const c_void, tau: *mut c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void);
    pub fn zlarfx_(side: *mut c_char, m: *mut c_int, n: *mut c_int, v: *const c_void, tau: *mut c_void, c: *mut c_void, ldc: *mut c_int, work: *mut c_void);

    pub fn slatms_(m: *mut c_int, n: *mut c_int, dist: *mut c_char, iseed: *mut c_int, sym: *mut c_char, d: *mut c_float, mode: *mut c_int, cond: *mut c_float, dmax: *mut c_float, kl: *mut c_int, ku: *mut c_int, pack: *mut c_char, a: *mut c_float, lda: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dlatms_(m: *mut c_int, n: *mut c_int, dist: *mut c_char, iseed: *mut c_int, sym: *mut c_char, d: *mut c_double, mode: *mut c_int, cond: *mut c_double, dmax: *mut c_double, kl: *mut c_int, ku: *mut c_int, pack: *mut c_char, a: *mut c_double, lda: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn clatms_(m: *mut c_int, n: *mut c_int, dist: *mut c_char, iseed: *mut c_int, sym: *mut c_char, d: *mut c_float, mode: *mut c_int, cond: *mut c_float, dmax: *mut c_float, kl: *mut c_int, ku: *mut c_int, pack: *mut c_char, a: *mut c_void, lda: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn zlatms_(m: *mut c_int, n: *mut c_int, dist: *mut c_char, iseed: *mut c_int, sym: *mut c_char, d: *mut c_double, mode: *mut c_int, cond: *mut c_double, dmax: *mut c_double, kl: *mut c_int, ku: *mut c_int, pack: *mut c_char, a: *mut c_void, lda: *mut c_int, work: *mut c_void, info: *mut c_int);

    pub fn dlag2s_(m: *mut c_int, n: *mut c_int, a: *const c_double, lda: *mut c_int, sa: *mut c_float, ldsa: *mut c_int, info: *mut c_int);

    pub fn slag2d_(m: *mut c_int, n: *mut c_int, sa: *const c_float, ldsa: *mut c_int, a: *mut c_double, lda: *mut c_int, info: *mut c_int);

    pub fn zlag2c_(m: *mut c_int, n: *mut c_int, a: *const c_void, lda: *mut c_int, sa: *mut c_void, ldsa: *mut c_int, info: *mut c_int);

    pub fn clag2z_(m: *mut c_int, n: *mut c_int, sa: *const c_void, ldsa: *mut c_int, a: *mut c_void, lda: *mut c_int, info: *mut c_int);

    pub fn slauum_(uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, info: *mut c_int);
    pub fn dlauum_(uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, info: *mut c_int);
    pub fn clauum_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, info: *mut c_int);
    pub fn zlauum_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, info: *mut c_int);

    pub fn slagge_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, d: *const c_float, a: *mut c_float, lda: *mut c_int, iseed: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dlagge_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, d: *const c_double, a: *mut c_double, lda: *mut c_int, iseed: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn clagge_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, d: *const c_float, a: *mut c_void, lda: *mut c_int, iseed: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn zlagge_(m: *mut c_int, n: *mut c_int, kl: *mut c_int, ku: *mut c_int, d: *const c_double, a: *mut c_void, lda: *mut c_int, iseed: *mut c_int, work: *mut c_void, info: *mut c_int);

    pub fn slaset_(uplo: *mut c_char, m: *mut c_int, n: *mut c_int, alpha: *mut c_float, beta: *mut c_float, a: *mut c_float, lda: *mut c_int);
    pub fn dlaset_(uplo: *mut c_char, m: *mut c_int, n: *mut c_int, alpha: *mut c_double, beta: *mut c_double, a: *mut c_double, lda: *mut c_int);
    pub fn claset_(uplo: *mut c_char, m: *mut c_int, n: *mut c_int, alpha: *mut c_void, beta: *mut c_void, a: *mut c_void, lda: *mut c_int);
    pub fn zlaset_(uplo: *mut c_char, m: *mut c_int, n: *mut c_int, alpha: *mut c_void, beta: *mut c_void, a: *mut c_void, lda: *mut c_int);

    pub fn slasrt_(id: *mut c_char, n: *mut c_int, d: *mut c_float, info: *mut c_int);
    pub fn dlasrt_(id: *mut c_char, n: *mut c_int, d: *mut c_double, info: *mut c_int);
    pub fn claghe_(n: *mut c_int, k: *mut c_int, d: *const c_float, a: *mut c_void, lda: *mut c_int, iseed: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn zlaghe_(n: *mut c_int, k: *mut c_int, d: *const c_double, a: *mut c_void, lda: *mut c_int, iseed: *mut c_int, work: *mut c_void, info: *mut c_int);

    pub fn slagsy_(n: *mut c_int, k: *mut c_int, d: *const c_float, a: *mut c_float, lda: *mut c_int, iseed: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dlagsy_(n: *mut c_int, k: *mut c_int, d: *const c_double, a: *mut c_double, lda: *mut c_int, iseed: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn clagsy_(n: *mut c_int, k: *mut c_int, d: *const c_float, a: *mut c_void, lda: *mut c_int, iseed: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn zlagsy_(n: *mut c_int, k: *mut c_int, d: *const c_double, a: *mut c_void, lda: *mut c_int, iseed: *mut c_int, work: *mut c_void, info: *mut c_int);

    pub fn slapmr_(forwrd: *mut c_int, m: *mut c_int, n: *mut c_int, x: *mut c_float, ldx: *mut c_int, k: *mut c_int);
    pub fn dlapmr_(forwrd: *mut c_int, m: *mut c_int, n: *mut c_int, x: *mut c_double, ldx: *mut c_int, k: *mut c_int);
    pub fn clapmr_(forwrd: *mut c_int, m: *mut c_int, n: *mut c_int, x: *mut c_void, ldx: *mut c_int, k: *mut c_int);
    pub fn zlapmr_(forwrd: *mut c_int, m: *mut c_int, n: *mut c_int, x: *mut c_void, ldx: *mut c_int, k: *mut c_int);

    pub fn slapy2_(x: *mut c_float, y: *mut c_float) -> c_float;
    pub fn dlapy2_(x: *mut c_double, y: *mut c_double) -> c_double;

    pub fn slapy3_(x: *mut c_float, y: *mut c_float, z: *mut c_float) -> c_float;
    pub fn dlapy3_(x: *mut c_double, y: *mut c_double, z: *mut c_double) -> c_double;

    pub fn slartgp_(f: *mut c_float, g: *mut c_float, cs: *mut c_float, sn: *mut c_float, r: *mut c_float);
    pub fn dlartgp_(f: *mut c_double, g: *mut c_double, cs: *mut c_double, sn: *mut c_double, r: *mut c_double);

    pub fn slartgs_(x: *mut c_float, y: *mut c_float, sigma: *mut c_float, cs: *mut c_float, sn: *mut c_float);
    pub fn dlartgs_(x: *mut c_double, y: *mut c_double, sigma: *mut c_double, cs: *mut c_double, sn: *mut c_double);

    // LAPACK 3.3.0
    pub fn cbbcsd_(jobu1: *mut c_char, jobu2: *mut c_char, jobv1t: *mut c_char, jobv2t: *mut c_char, trans: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, theta: *mut c_float, phi: *mut c_float, u1: *mut c_void, ldu1: *mut c_int, u2: *mut c_void, ldu2: *mut c_int, v1t: *mut c_void, ldv1t: *mut c_int, v2t: *mut c_void, ldv2t: *mut c_int, b11d: *mut c_float, b11e: *mut c_float, b12d: *mut c_float, b12e: *mut c_float, b21d: *mut c_float, b21e: *mut c_float, b22d: *mut c_float, b22e: *mut c_float, rwork: *mut c_float, lrwork: *mut c_int, info: *mut c_int);
    pub fn cheswapr_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, i1: *mut c_int, i2: *mut c_int);
    pub fn chetri2_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn chetri2x_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, nb: *mut c_int, info: *mut c_int);
    pub fn chetrs2_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn csyconv_(uplo: *mut c_char, way: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, info: *mut c_int);
    pub fn csyswapr_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, i1: *mut c_int, i2: *mut c_int);
    pub fn csytri2_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn csytri2x_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, nb: *mut c_int, info: *mut c_int);
    pub fn csytrs2_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn cunbdb_(trans: *mut c_char, signs: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, x11: *mut c_void, ldx11: *mut c_int, x12: *mut c_void, ldx12: *mut c_int, x21: *mut c_void, ldx21: *mut c_int, x22: *mut c_void, ldx22: *mut c_int, theta: *mut c_float, phi: *mut c_float, taup1: *mut c_void, taup2: *mut c_void, tauq1: *mut c_void, tauq2: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn cuncsd_(jobu1: *mut c_char, jobu2: *mut c_char, jobv1t: *mut c_char, jobv2t: *mut c_char, trans: *mut c_char, signs: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, x11: *mut c_void, ldx11: *mut c_int, x12: *mut c_void, ldx12: *mut c_int, x21: *mut c_void, ldx21: *mut c_int, x22: *mut c_void, ldx22: *mut c_int, theta: *mut c_float, u1: *mut c_void, ldu1: *mut c_int, u2: *mut c_void, ldu2: *mut c_int, v1t: *mut c_void, ldv1t: *mut c_int, v2t: *mut c_void, ldv2t: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_float, lrwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn dbbcsd_(jobu1: *mut c_char, jobu2: *mut c_char, jobv1t: *mut c_char, jobv2t: *mut c_char, trans: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, theta: *mut c_double, phi: *mut c_double, u1: *mut c_double, ldu1: *mut c_int, u2: *mut c_double, ldu2: *mut c_int, v1t: *mut c_double, ldv1t: *mut c_int, v2t: *mut c_double, ldv2t: *mut c_int, b11d: *mut c_double, b11e: *mut c_double, b12d: *mut c_double, b12e: *mut c_double, b21d: *mut c_double, b21e: *mut c_double, b22d: *mut c_double, b22e: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn dorbdb_(trans: *mut c_char, signs: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, x11: *mut c_double, ldx11: *mut c_int, x12: *mut c_double, ldx12: *mut c_int, x21: *mut c_double, ldx21: *mut c_int, x22: *mut c_double, ldx22: *mut c_int, theta: *mut c_double, phi: *mut c_double, taup1: *mut c_double, taup2: *mut c_double, tauq1: *mut c_double, tauq2: *mut c_double, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn dorcsd_(jobu1: *mut c_char, jobu2: *mut c_char, jobv1t: *mut c_char, jobv2t: *mut c_char, trans: *mut c_char, signs: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, x11: *mut c_double, ldx11: *mut c_int, x12: *mut c_double, ldx12: *mut c_int, x21: *mut c_double, ldx21: *mut c_int, x22: *mut c_double, ldx22: *mut c_int, theta: *mut c_double, u1: *mut c_double, ldu1: *mut c_int, u2: *mut c_double, ldu2: *mut c_int, v1t: *mut c_double, ldv1t: *mut c_int, v2t: *mut c_double, ldv2t: *mut c_int, work: *mut c_double, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn dsyconv_(uplo: *mut c_char, way: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *const c_int, work: *mut c_double, info: *mut c_int);
    pub fn dsyswapr_(uplo: *mut c_char, n: *mut c_int, a: *mut c_double, i1: *mut c_int, i2: *mut c_int);
    pub fn dsytri2_(uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn dsytri2x_(uplo: *mut c_char, n: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *const c_int, work: *mut c_double, nb: *mut c_int, info: *mut c_int);
    pub fn dsytrs2_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_double, lda: *mut c_int, ipiv: *const c_int, b: *mut c_double, ldb: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn sbbcsd_(jobu1: *mut c_char, jobu2: *mut c_char, jobv1t: *mut c_char, jobv2t: *mut c_char, trans: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, theta: *mut c_float, phi: *mut c_float, u1: *mut c_float, ldu1: *mut c_int, u2: *mut c_float, ldu2: *mut c_int, v1t: *mut c_float, ldv1t: *mut c_int, v2t: *mut c_float, ldv2t: *mut c_int, b11d: *mut c_float, b11e: *mut c_float, b12d: *mut c_float, b12e: *mut c_float, b21d: *mut c_float, b21e: *mut c_float, b22d: *mut c_float, b22e: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn sorbdb_(trans: *mut c_char, signs: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, x11: *mut c_float, ldx11: *mut c_int, x12: *mut c_float, ldx12: *mut c_int, x21: *mut c_float, ldx21: *mut c_int, x22: *mut c_float, ldx22: *mut c_int, theta: *mut c_float, phi: *mut c_float, taup1: *mut c_float, taup2: *mut c_float, tauq1: *mut c_float, tauq2: *mut c_float, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn sorcsd_(jobu1: *mut c_char, jobu2: *mut c_char, jobv1t: *mut c_char, jobv2t: *mut c_char, trans: *mut c_char, signs: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, x11: *mut c_float, ldx11: *mut c_int, x12: *mut c_float, ldx12: *mut c_int, x21: *mut c_float, ldx21: *mut c_int, x22: *mut c_float, ldx22: *mut c_int, theta: *mut c_float, u1: *mut c_float, ldu1: *mut c_int, u2: *mut c_float, ldu2: *mut c_int, v1t: *mut c_float, ldv1t: *mut c_int, v2t: *mut c_float, ldv2t: *mut c_int, work: *mut c_float, lwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);
    pub fn ssyconv_(uplo: *mut c_char, way: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, ipiv: *const c_int, work: *mut c_float, info: *mut c_int);
    pub fn ssyswapr_(uplo: *mut c_char, n: *mut c_int, a: *mut c_float, i1: *mut c_int, i2: *mut c_int);
    pub fn ssytri2_(uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn ssytri2x_(uplo: *mut c_char, n: *mut c_int, a: *mut c_float, lda: *mut c_int, ipiv: *const c_int, work: *mut c_float, nb: *mut c_int, info: *mut c_int);
    pub fn ssytrs2_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_float, lda: *mut c_int, ipiv: *const c_int, b: *mut c_float, ldb: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn zbbcsd_(jobu1: *mut c_char, jobu2: *mut c_char, jobv1t: *mut c_char, jobv2t: *mut c_char, trans: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, theta: *mut c_double, phi: *mut c_double, u1: *mut c_void, ldu1: *mut c_int, u2: *mut c_void, ldu2: *mut c_int, v1t: *mut c_void, ldv1t: *mut c_int, v2t: *mut c_void, ldv2t: *mut c_int, b11d: *mut c_double, b11e: *mut c_double, b12d: *mut c_double, b12e: *mut c_double, b21d: *mut c_double, b21e: *mut c_double, b22d: *mut c_double, b22e: *mut c_double, rwork: *mut c_double, lrwork: *mut c_int, info: *mut c_int);
    pub fn zheswapr_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, i1: *mut c_int, i2: *mut c_int);
    pub fn zhetri2_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zhetri2x_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, nb: *mut c_int, info: *mut c_int);
    pub fn zhetrs2_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn zsyconv_(uplo: *mut c_char, way: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, info: *mut c_int);
    pub fn zsyswapr_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, i1: *mut c_int, i2: *mut c_int);
    pub fn zsytri2_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zsytri2x_(uplo: *mut c_char, n: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *const c_int, work: *mut c_void, nb: *mut c_int, info: *mut c_int);
    pub fn zsytrs2_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *const c_void, lda: *mut c_int, ipiv: *const c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn zunbdb_(trans: *mut c_char, signs: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, x11: *mut c_void, ldx11: *mut c_int, x12: *mut c_void, ldx12: *mut c_int, x21: *mut c_void, ldx21: *mut c_int, x22: *mut c_void, ldx22: *mut c_int, theta: *mut c_double, phi: *mut c_double, taup1: *mut c_void, taup2: *mut c_void, tauq1: *mut c_void, tauq2: *mut c_void, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zuncsd_(jobu1: *mut c_char, jobu2: *mut c_char, jobv1t: *mut c_char, jobv2t: *mut c_char, trans: *mut c_char, signs: *mut c_char, m: *mut c_int, p: *mut c_int, q: *mut c_int, x11: *mut c_void, ldx11: *mut c_int, x12: *mut c_void, ldx12: *mut c_int, x21: *mut c_void, ldx21: *mut c_int, x22: *mut c_void, ldx22: *mut c_int, theta: *mut c_double, u1: *mut c_void, ldu1: *mut c_int, u2: *mut c_void, ldu2: *mut c_int, v1t: *mut c_void, ldv1t: *mut c_int, v2t: *mut c_void, ldv2t: *mut c_int, work: *mut c_void, lwork: *mut c_int, rwork: *mut c_double, lrwork: *mut c_int, iwork: *mut c_int, info: *mut c_int);

    // LAPACK 3.4.0
    pub fn sgemqrt_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, nb: *mut c_int, v: *const c_float, ldv: *mut c_int, t: *const c_float, ldt: *mut c_int, c: *mut c_float, ldc: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dgemqrt_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, nb: *mut c_int, v: *const c_double, ldv: *mut c_int, t: *const c_double, ldt: *mut c_int, c: *mut c_double, ldc: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn cgemqrt_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, nb: *mut c_int, v: *const c_void, ldv: *mut c_int, t: *const c_void, ldt: *mut c_int, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn zgemqrt_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, nb: *mut c_int, v: *const c_void, ldv: *mut c_int, t: *const c_void, ldt: *mut c_int, c: *mut c_void, ldc: *mut c_int, work: *mut c_void, info: *mut c_int);

    pub fn sgeqrt_(m: *mut c_int, n: *mut c_int, nb: *mut c_int, a: *mut c_float, lda: *mut c_int, t: *mut c_float, ldt: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dgeqrt_(m: *mut c_int, n: *mut c_int, nb: *mut c_int, a: *mut c_double, lda: *mut c_int, t: *mut c_double, ldt: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn cgeqrt_(m: *mut c_int, n: *mut c_int, nb: *mut c_int, a: *mut c_void, lda: *mut c_int, t: *mut c_void, ldt: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn zgeqrt_(m: *mut c_int, n: *mut c_int, nb: *mut c_int, a: *mut c_void, lda: *mut c_int, t: *mut c_void, ldt: *mut c_int, work: *mut c_void, info: *mut c_int);

    pub fn sgeqrt2_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, t: *mut c_float, ldt: *mut c_int, info: *mut c_int);
    pub fn dgeqrt2_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, t: *mut c_double, ldt: *mut c_int, info: *mut c_int);
    pub fn cgeqrt2_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, t: *mut c_void, ldt: *mut c_int, info: *mut c_int);
    pub fn zgeqrt2_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, t: *mut c_void, ldt: *mut c_int, info: *mut c_int);

    pub fn sgeqrt3_(m: *mut c_int, n: *mut c_int, a: *mut c_float, lda: *mut c_int, t: *mut c_float, ldt: *mut c_int, info: *mut c_int);
    pub fn dgeqrt3_(m: *mut c_int, n: *mut c_int, a: *mut c_double, lda: *mut c_int, t: *mut c_double, ldt: *mut c_int, info: *mut c_int);
    pub fn cgeqrt3_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, t: *mut c_void, ldt: *mut c_int, info: *mut c_int);
    pub fn zgeqrt3_(m: *mut c_int, n: *mut c_int, a: *mut c_void, lda: *mut c_int, t: *mut c_void, ldt: *mut c_int, info: *mut c_int);

    pub fn stpmqrt_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, nb: *mut c_int, v: *const c_float, ldv: *mut c_int, t: *const c_float, ldt: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dtpmqrt_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, nb: *mut c_int, v: *const c_double, ldv: *mut c_int, t: *const c_double, ldt: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn ctpmqrt_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, nb: *mut c_int, v: *const c_void, ldv: *mut c_int, t: *const c_void, ldt: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn ztpmqrt_(side: *mut c_char, trans: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, nb: *mut c_int, v: *const c_void, ldv: *mut c_int, t: *const c_void, ldt: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, info: *mut c_int);

    pub fn stpqrt_(m: *mut c_int, n: *mut c_int, l: *mut c_int, nb: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, t: *mut c_float, ldt: *mut c_int, work: *mut c_float, info: *mut c_int);
    pub fn dtpqrt_(m: *mut c_int, n: *mut c_int, l: *mut c_int, nb: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, t: *mut c_double, ldt: *mut c_int, work: *mut c_double, info: *mut c_int);
    pub fn ctpqrt_(m: *mut c_int, n: *mut c_int, l: *mut c_int, nb: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, t: *mut c_void, ldt: *mut c_int, work: *mut c_void, info: *mut c_int);
    pub fn ztpqrt_(m: *mut c_int, n: *mut c_int, l: *mut c_int, nb: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, t: *mut c_void, ldt: *mut c_int, work: *mut c_void, info: *mut c_int);

    pub fn stpqrt2_(m: *mut c_int, n: *mut c_int, l: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, t: *mut c_float, ldt: *mut c_int, info: *mut c_int);
    pub fn dtpqrt2_(m: *mut c_int, n: *mut c_int, l: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, t: *mut c_double, ldt: *mut c_int, info: *mut c_int);
    pub fn ctpqrt2_(m: *mut c_int, n: *mut c_int, l: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, t: *mut c_void, ldt: *mut c_int, info: *mut c_int);
    pub fn ztpqrt2_(m: *mut c_int, n: *mut c_int, l: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, t: *mut c_void, ldt: *mut c_int, info: *mut c_int);

    pub fn stprfb_(side: *mut c_char, trans: *mut c_char, direct: *mut c_char, storev: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, v: *const c_float, ldv: *mut c_int, t: *const c_float, ldt: *mut c_int, a: *mut c_float, lda: *mut c_int, b: *mut c_float, ldb: *mut c_int, work: *const c_float, ldwork: *mut c_int);
    pub fn dtprfb_(side: *mut c_char, trans: *mut c_char, direct: *mut c_char, storev: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, v: *const c_double, ldv: *mut c_int, t: *const c_double, ldt: *mut c_int, a: *mut c_double, lda: *mut c_int, b: *mut c_double, ldb: *mut c_int, work: *const c_double, ldwork: *mut c_int);
    pub fn ctprfb_(side: *mut c_char, trans: *mut c_char, direct: *mut c_char, storev: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, v: *const c_void, ldv: *mut c_int, t: *const c_void, ldt: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *const c_float, ldwork: *mut c_int);
    pub fn ztprfb_(side: *mut c_char, trans: *mut c_char, direct: *mut c_char, storev: *mut c_char, m: *mut c_int, n: *mut c_int, k: *mut c_int, l: *mut c_int, v: *const c_void, ldv: *mut c_int, t: *const c_void, ldt: *mut c_int, a: *mut c_void, lda: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *const c_double, ldwork: *mut c_int);

    // LAPACK 3.X.X
    pub fn ssysv_rook_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_float, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_float, ldb: *mut c_int, work: *mut c_float, lwork: *mut c_int, info: *mut c_int);
    pub fn dsysv_rook_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_double, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_double, ldb: *mut c_int, work: *mut c_double, lwork: *mut c_int, info: *mut c_int);
    pub fn csysv_rook_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);
    pub fn zsysv_rook_(uplo: *mut c_char, n: *mut c_int, nrhs: *mut c_int, a: *mut c_void, lda: *mut c_int, ipiv: *mut c_int, b: *mut c_void, ldb: *mut c_int, work: *mut c_void, lwork: *mut c_int, info: *mut c_int);

    pub fn csyr_(uplo: *mut c_char, n: *mut c_int, alpha: *mut c_void, x: *const c_void, incx: *mut c_int, a: *mut c_void, lda: *mut c_int);
    pub fn zsyr_(uplo: *mut c_char, n: *mut c_int, alpha: *mut c_void, x: *const c_void, incx: *mut c_int, a: *mut c_void, lda: *mut c_int);

    pub fn ilaver_(vers_major: *const c_int, vers_minor: *const c_int, vers_patch: *const c_int);
}
