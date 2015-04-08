//! Bindings to the [Linear Algebra PACKage][1].
//!
//! [1]: http://en.wikipedia.org/wiki/LAPACK

#![allow(bad_style)]

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
pub type LAPACK_Z_SELECT3 = Option<extern "C" fn(*const c_void, *const c_void) -> c_int>;

pub type complex_float = [c_float; 2];
pub type complex_double = [c_double; 2];

pub const LAPACK_ROW_MAJOR: c_int = 101;
pub const LAPACK_COL_MAJOR: c_int = 102;

pub const LAPACK_WORK_MEMORY_ERROR: c_int = -1010;
pub const LAPACK_TRANSPOSE_MEMORY_ERROR: c_int = -1011;

extern "C" {
    pub fn lapack_make_complex_float(re: c_float, im: c_float) -> complex_float;
    pub fn lapack_make_complex_double(re: c_double, im: c_double) -> complex_double;
    pub fn LAPACK_lsame(ca: *mut c_char, cb: *mut c_char, lca: c_int, lcb: c_int) -> c_int;
}

// High-level C interface
extern "C" {
    pub fn LAPACKE_sbdsdc(matrix_order: c_int, uplo: c_char, compq: c_char, n: c_int, d: *mut c_float, e: *mut c_float, u: *mut c_float, ldu: c_int, vt: *mut c_float, ldvt: c_int, q: *mut c_float, iq: *mut c_int) -> c_int;
    pub fn LAPACKE_dbdsdc(matrix_order: c_int, uplo: c_char, compq: c_char, n: c_int, d: *mut c_double, e: *mut c_double, u: *mut c_double, ldu: c_int, vt: *mut c_double, ldvt: c_int, q: *mut c_double, iq: *mut c_int) -> c_int;

    pub fn LAPACKE_sbdsqr(matrix_order: c_int, uplo: c_char, n: c_int, ncvt: c_int, nru: c_int, ncc: c_int, d: *mut c_float, e: *mut c_float, vt: *mut c_float, ldvt: c_int, u: *mut c_float, ldu: c_int, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dbdsqr(matrix_order: c_int, uplo: c_char, n: c_int, ncvt: c_int, nru: c_int, ncc: c_int, d: *mut c_double, e: *mut c_double, vt: *mut c_double, ldvt: c_int, u: *mut c_double, ldu: c_int, c: *mut c_double, ldc: c_int) -> c_int;
    pub fn LAPACKE_cbdsqr(matrix_order: c_int, uplo: c_char, n: c_int, ncvt: c_int, nru: c_int, ncc: c_int, d: *mut c_float, e: *mut c_float, vt: *mut c_void, ldvt: c_int, u: *mut c_void, ldu: c_int, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zbdsqr(matrix_order: c_int, uplo: c_char, n: c_int, ncvt: c_int, nru: c_int, ncc: c_int, d: *mut c_double, e: *mut c_double, vt: *mut c_void, ldvt: c_int, u: *mut c_void, ldu: c_int, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_sdisna(job: c_char, m: c_int, n: c_int, d: *const c_float, sep: *mut c_float) -> c_int;
    pub fn LAPACKE_ddisna(job: c_char, m: c_int, n: c_int, d: *const c_double, sep: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbbrd(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, ncc: c_int, kl: c_int, ku: c_int, ab: *mut c_float, ldab: c_int, d: *mut c_float, e: *mut c_float, q: *mut c_float, ldq: c_int, pt: *mut c_float, ldpt: c_int, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dgbbrd(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, ncc: c_int, kl: c_int, ku: c_int, ab: *mut c_double, ldab: c_int, d: *mut c_double, e: *mut c_double, q: *mut c_double, ldq: c_int, pt: *mut c_double, ldpt: c_int, c: *mut c_double, ldc: c_int) -> c_int;
    pub fn LAPACKE_cgbbrd(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, ncc: c_int, kl: c_int, ku: c_int, ab: *mut c_void, ldab: c_int, d: *mut c_float, e: *mut c_float, q: *mut c_void, ldq: c_int, pt: *mut c_void, ldpt: c_int, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zgbbrd(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, ncc: c_int, kl: c_int, ku: c_int, ab: *mut c_void, ldab: c_int, d: *mut c_double, e: *mut c_double, q: *mut c_void, ldq: c_int, pt: *mut c_void, ldpt: c_int, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_sgbcon(matrix_order: c_int, norm: c_char, n: c_int, kl: c_int, ku: c_int, ab: *const c_float, ldab: c_int, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dgbcon(matrix_order: c_int, norm: c_char, n: c_int, kl: c_int, ku: c_int, ab: *const c_double, ldab: c_int, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_cgbcon(matrix_order: c_int, norm: c_char, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbcon(matrix_order: c_int, norm: c_char, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbequ(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_float, ldab: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dgbequ(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_double, ldab: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cgbequ(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbequ(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbequb(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_float, ldab: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dgbequb(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_double, ldab: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cgbequb(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbequb(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbrfs(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, afb: *const c_float, ldafb: c_int, ipiv: *const c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dgbrfs(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, afb: *const c_double, ldafb: c_int, ipiv: *const c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cgbrfs(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbrfs(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbrfsx(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, afb: *const c_float, ldafb: c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_dgbrfsx(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, afb: *const c_double, ldafb: c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;
    pub fn LAPACKE_cgbrfsx(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbrfsx(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbsv(matrix_order: c_int, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_float, ldab: c_int, ipiv: *mut c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgbsv(matrix_order: c_int, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_double, ldab: c_int, ipiv: *mut c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgbsv(matrix_order: c_int, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgbsv(matrix_order: c_int, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sgbsvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_float, ldab: c_int, afb: *mut c_float, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, rpivot: *mut c_float) -> c_int;
    pub fn LAPACKE_dgbsvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_double, ldab: c_int, afb: *mut c_double, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, rpivot: *mut c_double) -> c_int;
    pub fn LAPACKE_cgbsvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, rpivot: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbsvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, rpivot: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbsvxx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_float, ldab: c_int, afb: *mut c_float, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_dgbsvxx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_double, ldab: c_int, afb: *mut c_double, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;
    pub fn LAPACKE_cgbsvxx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbsvxx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbtrf(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *mut c_float, ldab: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_dgbtrf(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *mut c_double, ldab: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_cgbtrf(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *mut c_void, ldab: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zgbtrf(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *mut c_void, ldab: c_int, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_sgbtrs(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, ipiv: *const c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgbtrs(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, ipiv: *const c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgbtrs(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgbtrs(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sgebak(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, scale: *const c_float, m: c_int, v: *mut c_float, ldv: c_int) -> c_int;
    pub fn LAPACKE_dgebak(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, scale: *const c_double, m: c_int, v: *mut c_double, ldv: c_int) -> c_int;
    pub fn LAPACKE_cgebak(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, scale: *const c_float, m: c_int, v: *mut c_void, ldv: c_int) -> c_int;
    pub fn LAPACKE_zgebak(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, scale: *const c_double, m: c_int, v: *mut c_void, ldv: c_int) -> c_int;

    pub fn LAPACKE_sgebal(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_float, lda: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float) -> c_int;
    pub fn LAPACKE_dgebal(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_double, lda: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double) -> c_int;
    pub fn LAPACKE_cgebal(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_void, lda: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float) -> c_int;
    pub fn LAPACKE_zgebal(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_void, lda: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double) -> c_int;

    pub fn LAPACKE_sgebrd(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, d: *mut c_float, e: *mut c_float, tauq: *mut c_float, taup: *mut c_float) -> c_int;
    pub fn LAPACKE_dgebrd(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, d: *mut c_double, e: *mut c_double, tauq: *mut c_double, taup: *mut c_double) -> c_int;
    pub fn LAPACKE_cgebrd(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, d: *mut c_float, e: *mut c_float, tauq: *mut c_void, taup: *mut c_void) -> c_int;
    pub fn LAPACKE_zgebrd(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, d: *mut c_double, e: *mut c_double, tauq: *mut c_void, taup: *mut c_void) -> c_int;

    pub fn LAPACKE_sgecon(matrix_order: c_int, norm: c_char, n: c_int, a: *const c_float, lda: c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dgecon(matrix_order: c_int, norm: c_char, n: c_int, a: *const c_double, lda: c_int, anorm: c_double, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_cgecon(matrix_order: c_int, norm: c_char, n: c_int, a: *const c_void, lda: c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_zgecon(matrix_order: c_int, norm: c_char, n: c_int, a: *const c_void, lda: c_int, anorm: c_double, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_sgeequ(matrix_order: c_int, m: c_int, n: c_int, a: *const c_float, lda: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeequ(matrix_order: c_int, m: c_int, n: c_int, a: *const c_double, lda: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeequ(matrix_order: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zgeequ(matrix_order: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_sgeequb(matrix_order: c_int, m: c_int, n: c_int, a: *const c_float, lda: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeequb(matrix_order: c_int, m: c_int, n: c_int, a: *const c_double, lda: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeequb(matrix_order: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zgeequb(matrix_order: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_sgees(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_S_SELECT2, n: c_int, a: *mut c_float, lda: c_int, sdim: *mut c_int, wr: *mut c_float, wi: *mut c_float, vs: *mut c_float, ldvs: c_int) -> c_int;
    pub fn LAPACKE_dgees(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_D_SELECT2, n: c_int, a: *mut c_double, lda: c_int, sdim: *mut c_int, wr: *mut c_double, wi: *mut c_double, vs: *mut c_double, ldvs: c_int) -> c_int;
    pub fn LAPACKE_cgees(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_C_SELECT1, n: c_int, a: *mut c_void, lda: c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: c_int) -> c_int;
    pub fn LAPACKE_zgees(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_Z_SELECT1, n: c_int, a: *mut c_void, lda: c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: c_int) -> c_int;

    pub fn LAPACKE_sgeesx(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_S_SELECT2, sense: c_char, n: c_int, a: *mut c_float, lda: c_int, sdim: *mut c_int, wr: *mut c_float, wi: *mut c_float, vs: *mut c_float, ldvs: c_int, rconde: *mut c_float, rcondv: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeesx(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_D_SELECT2, sense: c_char, n: c_int, a: *mut c_double, lda: c_int, sdim: *mut c_int, wr: *mut c_double, wi: *mut c_double, vs: *mut c_double, ldvs: c_int, rconde: *mut c_double, rcondv: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeesx(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_C_SELECT1, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: c_int, rconde: *mut c_float, rcondv: *mut c_float) -> c_int;
    pub fn LAPACKE_zgeesx(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_Z_SELECT1, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: c_int, rconde: *mut c_double, rcondv: *mut c_double) -> c_int;

    pub fn LAPACKE_sgeev(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_float, lda: c_int, wr: *mut c_float, wi: *mut c_float, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int) -> c_int;
    pub fn LAPACKE_dgeev(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_double, lda: c_int, wr: *mut c_double, wi: *mut c_double, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int) -> c_int;
    pub fn LAPACKE_cgeev(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int) -> c_int;
    pub fn LAPACKE_zgeev(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int) -> c_int;

    pub fn LAPACKE_sgeevx(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_float, lda: c_int, wr: *mut c_float, wi: *mut c_float, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float, abnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeevx(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_double, lda: c_int, wr: *mut c_double, wi: *mut c_double, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double, abnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeevx(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float, abnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float) -> c_int;
    pub fn LAPACKE_zgeevx(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double, abnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double) -> c_int;

    pub fn LAPACKE_sgehrd(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dgehrd(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_cgehrd(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zgehrd(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_sgejsv(matrix_order: c_int, joba: c_char, jobu: c_char, jobv: c_char, jobr: c_char, jobt: c_char, jobp: c_char, m: c_int, n: c_int, a: *mut c_float, lda: c_int, sva: *mut c_float, u: *mut c_float, ldu: c_int, v: *mut c_float, ldv: c_int, stat: *mut c_float, istat: *mut c_int) -> c_int;
    pub fn LAPACKE_dgejsv(matrix_order: c_int, joba: c_char, jobu: c_char, jobv: c_char, jobr: c_char, jobt: c_char, jobp: c_char, m: c_int, n: c_int, a: *mut c_double, lda: c_int, sva: *mut c_double, u: *mut c_double, ldu: c_int, v: *mut c_double, ldv: c_int, stat: *mut c_double, istat: *mut c_int) -> c_int;

    pub fn LAPACKE_sgelq2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dgelq2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_cgelq2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zgelq2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_sgelqf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dgelqf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_cgelqf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zgelqf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_sgels(matrix_order: c_int, trans: c_char, m: c_int, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgels(matrix_order: c_int, trans: c_char, m: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgels(matrix_order: c_int, trans: c_char, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgels(matrix_order: c_int, trans: c_char, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sgelsd(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, s: *mut c_float, rcond: c_float, rank: *mut c_int) -> c_int;
    pub fn LAPACKE_dgelsd(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, s: *mut c_double, rcond: c_double, rank: *mut c_int) -> c_int;
    pub fn LAPACKE_cgelsd(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, s: *mut c_float, rcond: c_float, rank: *mut c_int) -> c_int;
    pub fn LAPACKE_zgelsd(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, s: *mut c_double, rcond: c_double, rank: *mut c_int) -> c_int;

    pub fn LAPACKE_sgelss(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, s: *mut c_float, rcond: c_float, rank: *mut c_int) -> c_int;
    pub fn LAPACKE_dgelss(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, s: *mut c_double, rcond: c_double, rank: *mut c_int) -> c_int;
    pub fn LAPACKE_cgelss(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, s: *mut c_float, rcond: c_float, rank: *mut c_int) -> c_int;
    pub fn LAPACKE_zgelss(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, s: *mut c_double, rcond: c_double, rank: *mut c_int) -> c_int;

    pub fn LAPACKE_sgelsy(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, jpvt: *mut c_int, rcond: c_float, rank: *mut c_int) -> c_int;
    pub fn LAPACKE_dgelsy(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, jpvt: *mut c_int, rcond: c_double, rank: *mut c_int) -> c_int;
    pub fn LAPACKE_cgelsy(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, jpvt: *mut c_int, rcond: c_float, rank: *mut c_int) -> c_int;
    pub fn LAPACKE_zgelsy(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, jpvt: *mut c_int, rcond: c_double, rank: *mut c_int) -> c_int;

    pub fn LAPACKE_sgeqlf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeqlf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeqlf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zgeqlf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_sgeqp3(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, jpvt: *mut c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeqp3(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, jpvt: *mut c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeqp3(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, jpvt: *mut c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zgeqp3(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, jpvt: *mut c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_sgeqpf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, jpvt: *mut c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeqpf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, jpvt: *mut c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeqpf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, jpvt: *mut c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zgeqpf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, jpvt: *mut c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_sgeqr2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeqr2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeqr2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zgeqr2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_sgeqrf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeqrf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeqrf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zgeqrf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_sgeqrfp(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeqrfp(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeqrfp(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zgeqrfp(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_sgerfs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, ipiv: *const c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dgerfs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, ipiv: *const c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cgerfs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zgerfs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_sgerfsx(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_dgerfsx(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;
    pub fn LAPACKE_cgerfsx(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_zgerfsx(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;

    pub fn LAPACKE_sgerqf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dgerqf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_cgerqf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zgerqf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_sgesdd(matrix_order: c_int, jobz: c_char, m: c_int, n: c_int, a: *mut c_float, lda: c_int, s: *mut c_float, u: *mut c_float, ldu: c_int, vt: *mut c_float, ldvt: c_int) -> c_int;
    pub fn LAPACKE_dgesdd(matrix_order: c_int, jobz: c_char, m: c_int, n: c_int, a: *mut c_double, lda: c_int, s: *mut c_double, u: *mut c_double, ldu: c_int, vt: *mut c_double, ldvt: c_int) -> c_int;
    pub fn LAPACKE_cgesdd(matrix_order: c_int, jobz: c_char, m: c_int, n: c_int, a: *mut c_void, lda: c_int, s: *mut c_float, u: *mut c_void, ldu: c_int, vt: *mut c_void, ldvt: c_int) -> c_int;
    pub fn LAPACKE_zgesdd(matrix_order: c_int, jobz: c_char, m: c_int, n: c_int, a: *mut c_void, lda: c_int, s: *mut c_double, u: *mut c_void, ldu: c_int, vt: *mut c_void, ldvt: c_int) -> c_int;

    pub fn LAPACKE_sgesv(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgesv(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgesv(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgesv(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_dsgesv(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, iter: *mut c_int) -> c_int;
    pub fn LAPACKE_zcgesv(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, iter: *mut c_int) -> c_int;

    pub fn LAPACKE_sgesvd(matrix_order: c_int, jobu: c_char, jobvt: c_char, m: c_int, n: c_int, a: *mut c_float, lda: c_int, s: *mut c_float, u: *mut c_float, ldu: c_int, vt: *mut c_float, ldvt: c_int, superb: *mut c_float) -> c_int;
    pub fn LAPACKE_dgesvd(matrix_order: c_int, jobu: c_char, jobvt: c_char, m: c_int, n: c_int, a: *mut c_double, lda: c_int, s: *mut c_double, u: *mut c_double, ldu: c_int, vt: *mut c_double, ldvt: c_int, superb: *mut c_double) -> c_int;
    pub fn LAPACKE_cgesvd(matrix_order: c_int, jobu: c_char, jobvt: c_char, m: c_int, n: c_int, a: *mut c_void, lda: c_int, s: *mut c_float, u: *mut c_void, ldu: c_int, vt: *mut c_void, ldvt: c_int, superb: *mut c_float) -> c_int;
    pub fn LAPACKE_zgesvd(matrix_order: c_int, jobu: c_char, jobvt: c_char, m: c_int, n: c_int, a: *mut c_void, lda: c_int, s: *mut c_double, u: *mut c_void, ldu: c_int, vt: *mut c_void, ldvt: c_int, superb: *mut c_double) -> c_int;

    pub fn LAPACKE_sgesvj(matrix_order: c_int, joba: c_char, jobu: c_char, jobv: c_char, m: c_int, n: c_int, a: *mut c_float, lda: c_int, sva: *mut c_float, mv: c_int, v: *mut c_float, ldv: c_int, stat: *mut c_float) -> c_int;
    pub fn LAPACKE_dgesvj(matrix_order: c_int, joba: c_char, jobu: c_char, jobv: c_char, m: c_int, n: c_int, a: *mut c_double, lda: c_int, sva: *mut c_double, mv: c_int, v: *mut c_double, ldv: c_int, stat: *mut c_double) -> c_int;

    pub fn LAPACKE_sgesvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, af: *mut c_float, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, rpivot: *mut c_float) -> c_int;
    pub fn LAPACKE_dgesvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, af: *mut c_double, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, rpivot: *mut c_double) -> c_int;
    pub fn LAPACKE_cgesvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, rpivot: *mut c_float) -> c_int;
    pub fn LAPACKE_zgesvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, rpivot: *mut c_double) -> c_int;

    pub fn LAPACKE_sgesvxx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, af: *mut c_float, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_dgesvxx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, af: *mut c_double, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;
    pub fn LAPACKE_cgesvxx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_zgesvxx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;

    pub fn LAPACKE_sgetf2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_dgetf2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_cgetf2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zgetf2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_sgetrf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_dgetrf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_cgetrf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zgetrf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_sgetri(matrix_order: c_int, n: c_int, a: *mut c_float, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_dgetri(matrix_order: c_int, n: c_int, a: *mut c_double, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_cgetri(matrix_order: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_zgetri(matrix_order: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;

    pub fn LAPACKE_sgetrs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, ipiv: *const c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgetrs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, ipiv: *const c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgetrs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgetrs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sggbak(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, lscale: *const c_float, rscale: *const c_float, m: c_int, v: *mut c_float, ldv: c_int) -> c_int;
    pub fn LAPACKE_dggbak(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, lscale: *const c_double, rscale: *const c_double, m: c_int, v: *mut c_double, ldv: c_int) -> c_int;
    pub fn LAPACKE_cggbak(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, lscale: *const c_float, rscale: *const c_float, m: c_int, v: *mut c_void, ldv: c_int) -> c_int;
    pub fn LAPACKE_zggbak(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, lscale: *const c_double, rscale: *const c_double, m: c_int, v: *mut c_void, ldv: c_int) -> c_int;

    pub fn LAPACKE_sggbal(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float) -> c_int;
    pub fn LAPACKE_dggbal(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double) -> c_int;
    pub fn LAPACKE_cggbal(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float) -> c_int;
    pub fn LAPACKE_zggbal(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double) -> c_int;

    pub fn LAPACKE_sgges(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_S_SELECT3, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, sdim: *mut c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vsl: *mut c_float, ldvsl: c_int, vsr: *mut c_float, ldvsr: c_int) -> c_int;
    pub fn LAPACKE_dgges(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_D_SELECT3, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, sdim: *mut c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vsl: *mut c_double, ldvsl: c_int, vsr: *mut c_double, ldvsr: c_int) -> c_int;
    pub fn LAPACKE_cgges(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_C_SELECT2, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: c_int, vsr: *mut c_void, ldvsr: c_int) -> c_int;
    pub fn LAPACKE_zgges(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_Z_SELECT2, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: c_int, vsr: *mut c_void, ldvsr: c_int) -> c_int;

    pub fn LAPACKE_sggesx(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_S_SELECT3, sense: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, sdim: *mut c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vsl: *mut c_float, ldvsl: c_int, vsr: *mut c_float, ldvsr: c_int, rconde: *mut c_float, rcondv: *mut c_float) -> c_int;
    pub fn LAPACKE_dggesx(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_D_SELECT3, sense: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, sdim: *mut c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vsl: *mut c_double, ldvsl: c_int, vsr: *mut c_double, ldvsr: c_int, rconde: *mut c_double, rcondv: *mut c_double) -> c_int;
    pub fn LAPACKE_cggesx(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_C_SELECT2, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: c_int, vsr: *mut c_void, ldvsr: c_int, rconde: *mut c_float, rcondv: *mut c_float) -> c_int;
    pub fn LAPACKE_zggesx(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_Z_SELECT2, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: c_int, vsr: *mut c_void, ldvsr: c_int, rconde: *mut c_double, rcondv: *mut c_double) -> c_int;

    pub fn LAPACKE_sggev(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int) -> c_int;
    pub fn LAPACKE_dggev(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int) -> c_int;
    pub fn LAPACKE_cggev(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int) -> c_int;
    pub fn LAPACKE_zggev(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int) -> c_int;

    pub fn LAPACKE_sggevx(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float, abnrm: *mut c_float, bbnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float) -> c_int;
    pub fn LAPACKE_dggevx(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double, abnrm: *mut c_double, bbnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double) -> c_int;
    pub fn LAPACKE_cggevx(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float, abnrm: *mut c_float, bbnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float) -> c_int;
    pub fn LAPACKE_zggevx(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double, abnrm: *mut c_double, bbnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double) -> c_int;

    pub fn LAPACKE_sggglm(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, d: *mut c_float, x: *mut c_float, y: *mut c_float) -> c_int;
    pub fn LAPACKE_dggglm(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, d: *mut c_double, x: *mut c_double, y: *mut c_double) -> c_int;
    pub fn LAPACKE_cggglm(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, d: *mut c_void, x: *mut c_void, y: *mut c_void) -> c_int;
    pub fn LAPACKE_zggglm(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, d: *mut c_void, x: *mut c_void, y: *mut c_void) -> c_int;

    pub fn LAPACKE_sgghrd(matrix_order: c_int, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, q: *mut c_float, ldq: c_int, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dgghrd(matrix_order: c_int, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, q: *mut c_double, ldq: c_int, z: *mut c_double, ldz: c_int) -> c_int;
    pub fn LAPACKE_cgghrd(matrix_order: c_int, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zgghrd(matrix_order: c_int, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_sgglse(matrix_order: c_int, m: c_int, n: c_int, p: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, c: *mut c_float, d: *mut c_float, x: *mut c_float) -> c_int;
    pub fn LAPACKE_dgglse(matrix_order: c_int, m: c_int, n: c_int, p: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, c: *mut c_double, d: *mut c_double, x: *mut c_double) -> c_int;
    pub fn LAPACKE_cgglse(matrix_order: c_int, m: c_int, n: c_int, p: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, c: *mut c_void, d: *mut c_void, x: *mut c_void) -> c_int;
    pub fn LAPACKE_zgglse(matrix_order: c_int, m: c_int, n: c_int, p: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, c: *mut c_void, d: *mut c_void, x: *mut c_void) -> c_int;

    pub fn LAPACKE_sggqrf(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_float, lda: c_int, taua: *mut c_float, b: *mut c_float, ldb: c_int, taub: *mut c_float) -> c_int;
    pub fn LAPACKE_dggqrf(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_double, lda: c_int, taua: *mut c_double, b: *mut c_double, ldb: c_int, taub: *mut c_double) -> c_int;
    pub fn LAPACKE_cggqrf(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_void, lda: c_int, taua: *mut c_void, b: *mut c_void, ldb: c_int, taub: *mut c_void) -> c_int;
    pub fn LAPACKE_zggqrf(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_void, lda: c_int, taua: *mut c_void, b: *mut c_void, ldb: c_int, taub: *mut c_void) -> c_int;

    pub fn LAPACKE_sggrqf(matrix_order: c_int, m: c_int, p: c_int, n: c_int, a: *mut c_float, lda: c_int, taua: *mut c_float, b: *mut c_float, ldb: c_int, taub: *mut c_float) -> c_int;
    pub fn LAPACKE_dggrqf(matrix_order: c_int, m: c_int, p: c_int, n: c_int, a: *mut c_double, lda: c_int, taua: *mut c_double, b: *mut c_double, ldb: c_int, taub: *mut c_double) -> c_int;
    pub fn LAPACKE_cggrqf(matrix_order: c_int, m: c_int, p: c_int, n: c_int, a: *mut c_void, lda: c_int, taua: *mut c_void, b: *mut c_void, ldb: c_int, taub: *mut c_void) -> c_int;
    pub fn LAPACKE_zggrqf(matrix_order: c_int, m: c_int, p: c_int, n: c_int, a: *mut c_void, lda: c_int, taua: *mut c_void, b: *mut c_void, ldb: c_int, taub: *mut c_void) -> c_int;

    pub fn LAPACKE_sggsvd(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, n: c_int, p: c_int, k: *mut c_int, l: *mut c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, alpha: *mut c_float, beta: *mut c_float, u: *mut c_float, ldu: c_int, v: *mut c_float, ldv: c_int, q: *mut c_float, ldq: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dggsvd(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, n: c_int, p: c_int, k: *mut c_int, l: *mut c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, alpha: *mut c_double, beta: *mut c_double, u: *mut c_double, ldu: c_int, v: *mut c_double, ldv: c_int, q: *mut c_double, ldq: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cggsvd(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, n: c_int, p: c_int, k: *mut c_int, l: *mut c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_float, beta: *mut c_float, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_zggsvd(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, n: c_int, p: c_int, k: *mut c_int, l: *mut c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_double, beta: *mut c_double, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int, iwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sggsvp(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, tola: c_float, tolb: c_float, k: *mut c_int, l: *mut c_int, u: *mut c_float, ldu: c_int, v: *mut c_float, ldv: c_int, q: *mut c_float, ldq: c_int) -> c_int;
    pub fn LAPACKE_dggsvp(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, tola: c_double, tolb: c_double, k: *mut c_int, l: *mut c_int, u: *mut c_double, ldu: c_int, v: *mut c_double, ldv: c_int, q: *mut c_double, ldq: c_int) -> c_int;
    pub fn LAPACKE_cggsvp(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, tola: c_float, tolb: c_float, k: *mut c_int, l: *mut c_int, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int) -> c_int;
    pub fn LAPACKE_zggsvp(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, tola: c_double, tolb: c_double, k: *mut c_int, l: *mut c_int, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int) -> c_int;

    pub fn LAPACKE_sgtcon(norm: c_char, n: c_int, dl: *const c_float, d: *const c_float, du: *const c_float, du2: *const c_float, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dgtcon(norm: c_char, n: c_int, dl: *const c_double, d: *const c_double, du: *const c_double, du2: *const c_double, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_cgtcon(norm: c_char, n: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_zgtcon(norm: c_char, n: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_sgtrfs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_float, d: *const c_float, du: *const c_float, dlf: *const c_float, df: *const c_float, duf: *const c_float, du2: *const c_float, ipiv: *const c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dgtrfs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_double, d: *const c_double, du: *const c_double, dlf: *const c_double, df: *const c_double, duf: *const c_double, du2: *const c_double, ipiv: *const c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cgtrfs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *const c_void, df: *const c_void, duf: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zgtrfs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *const c_void, df: *const c_void, duf: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_sgtsv(matrix_order: c_int, n: c_int, nrhs: c_int, dl: *mut c_float, d: *mut c_float, du: *mut c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgtsv(matrix_order: c_int, n: c_int, nrhs: c_int, dl: *mut c_double, d: *mut c_double, du: *mut c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgtsv(matrix_order: c_int, n: c_int, nrhs: c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgtsv(matrix_order: c_int, n: c_int, nrhs: c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sgtsvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_float, d: *const c_float, du: *const c_float, dlf: *mut c_float, df: *mut c_float, duf: *mut c_float, du2: *mut c_float, ipiv: *mut c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dgtsvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_double, d: *const c_double, du: *const c_double, dlf: *mut c_double, df: *mut c_double, duf: *mut c_double, du2: *mut c_double, ipiv: *mut c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cgtsvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *mut c_void, df: *mut c_void, duf: *mut c_void, du2: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zgtsvx(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *mut c_void, df: *mut c_void, duf: *mut c_void, du2: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_sgttrf(n: c_int, dl: *mut c_float, d: *mut c_float, du: *mut c_float, du2: *mut c_float, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_dgttrf(n: c_int, dl: *mut c_double, d: *mut c_double, du: *mut c_double, du2: *mut c_double, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_cgttrf(n: c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, du2: *mut c_void, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zgttrf(n: c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, du2: *mut c_void, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_sgttrs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_float, d: *const c_float, du: *const c_float, du2: *const c_float, ipiv: *const c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgttrs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_double, d: *const c_double, du: *const c_double, du2: *const c_double, ipiv: *const c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgttrs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgttrs(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_chbev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, w: *mut c_float, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zhbev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, w: *mut c_double, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_chbevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, w: *mut c_float, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zhbevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, w: *mut c_double, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_chbevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, q: *mut c_void, ldq: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zhbevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, q: *mut c_void, ldq: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_chbgst(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *const c_void, ldbb: c_int, x: *mut c_void, ldx: c_int) -> c_int;
    pub fn LAPACKE_zhbgst(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *const c_void, ldbb: c_int, x: *mut c_void, ldx: c_int) -> c_int;

    pub fn LAPACKE_chbgv(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, w: *mut c_float, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zhbgv(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, w: *mut c_double, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_chbgvd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, w: *mut c_float, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zhbgvd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, w: *mut c_double, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_chbgvx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, q: *mut c_void, ldq: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zhbgvx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, q: *mut c_void, ldq: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_chbtrd(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, d: *mut c_float, e: *mut c_float, q: *mut c_void, ldq: c_int) -> c_int;
    pub fn LAPACKE_zhbtrd(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, d: *mut c_double, e: *mut c_double, q: *mut c_void, ldq: c_int) -> c_int;

    pub fn LAPACKE_checon(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_zhecon(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_cheequb(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zheequb(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_cheev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_float) -> c_int;
    pub fn LAPACKE_zheev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_double) -> c_int;

    pub fn LAPACKE_cheevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_float) -> c_int;
    pub fn LAPACKE_zheevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_double) -> c_int;

    pub fn LAPACKE_cheevr(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, isuppz: *mut c_int) -> c_int;
    pub fn LAPACKE_zheevr(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, isuppz: *mut c_int) -> c_int;

    pub fn LAPACKE_cheevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zheevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_chegst(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *const c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zhegst(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *const c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_chegv(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, w: *mut c_float) -> c_int;
    pub fn LAPACKE_zhegv(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, w: *mut c_double) -> c_int;

    pub fn LAPACKE_chegvd(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, w: *mut c_float) -> c_int;
    pub fn LAPACKE_zhegvd(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, w: *mut c_double) -> c_int;

    pub fn LAPACKE_chegvx(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zhegvx(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_cherfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zherfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_cherfsx(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, s: *const c_float, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_zherfsx(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, s: *const c_double, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;

    pub fn LAPACKE_chesv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zhesv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_chesvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zhesvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_chesvxx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_zhesvxx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;

    pub fn LAPACKE_chetrd(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, d: *mut c_float, e: *mut c_float, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zhetrd(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, d: *mut c_double, e: *mut c_double, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_chetrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zhetrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_chetri(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_zhetri(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;

    pub fn LAPACKE_chetrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zhetrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_chfrk(matrix_order: c_int, transr: c_char, uplo: c_char, trans: c_char, n: c_int, k: c_int, alpha: c_float, a: *const c_void, lda: c_int, beta: c_float, c: *mut c_void) -> c_int;
    pub fn LAPACKE_zhfrk(matrix_order: c_int, transr: c_char, uplo: c_char, trans: c_char, n: c_int, k: c_int, alpha: c_double, a: *const c_void, lda: c_int, beta: c_double, c: *mut c_void) -> c_int;

    pub fn LAPACKE_shgeqz(matrix_order: c_int, job: c_char, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_float, ldh: c_int, t: *mut c_float, ldt: c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, q: *mut c_float, ldq: c_int, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dhgeqz(matrix_order: c_int, job: c_char, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_double, ldh: c_int, t: *mut c_double, ldt: c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, q: *mut c_double, ldq: c_int, z: *mut c_double, ldz: c_int) -> c_int;
    pub fn LAPACKE_chgeqz(matrix_order: c_int, job: c_char, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_void, ldh: c_int, t: *mut c_void, ldt: c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zhgeqz(matrix_order: c_int, job: c_char, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_void, ldh: c_int, t: *mut c_void, ldt: c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_chpcon(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_zhpcon(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_chpev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zhpev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_chpevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zhpevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_chpevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_void, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zhpevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_void, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_chpgst(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, ap: *mut c_void, bp: *const c_void) -> c_int;
    pub fn LAPACKE_zhpgst(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, ap: *mut c_void, bp: *const c_void) -> c_int;

    pub fn LAPACKE_chpgv(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zhpgv(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_chpgvd(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zhpgvd(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_chpgvx(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zhpgvx(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_chprfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zhprfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_chpsv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zhpsv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_chpsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zhpsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_chptrd(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, d: *mut c_float, e: *mut c_float, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zhptrd(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, d: *mut c_double, e: *mut c_double, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_chptrf(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zhptrf(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_chptri(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_zhptri(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *const c_int) -> c_int;

    pub fn LAPACKE_chptrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zhptrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_shsein(matrix_order: c_int, job: c_char, eigsrc: c_char, initv: c_char, select: *mut c_int, n: c_int, h: *const c_float, ldh: c_int, wr: *mut c_float, wi: *const c_float, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, mm: c_int, m: *mut c_int, ifaill: *mut c_int, ifailr: *mut c_int) -> c_int;
    pub fn LAPACKE_dhsein(matrix_order: c_int, job: c_char, eigsrc: c_char, initv: c_char, select: *mut c_int, n: c_int, h: *const c_double, ldh: c_int, wr: *mut c_double, wi: *const c_double, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, mm: c_int, m: *mut c_int, ifaill: *mut c_int, ifailr: *mut c_int) -> c_int;
    pub fn LAPACKE_chsein(matrix_order: c_int, job: c_char, eigsrc: c_char, initv: c_char, select: *const c_int, n: c_int, h: *const c_void, ldh: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int, ifaill: *mut c_int, ifailr: *mut c_int) -> c_int;
    pub fn LAPACKE_zhsein(matrix_order: c_int, job: c_char, eigsrc: c_char, initv: c_char, select: *const c_int, n: c_int, h: *const c_void, ldh: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int, ifaill: *mut c_int, ifailr: *mut c_int) -> c_int;

    pub fn LAPACKE_shseqr(matrix_order: c_int, job: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_float, ldh: c_int, wr: *mut c_float, wi: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dhseqr(matrix_order: c_int, job: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_double, ldh: c_int, wr: *mut c_double, wi: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;
    pub fn LAPACKE_chseqr(matrix_order: c_int, job: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_void, ldh: c_int, w: *mut c_void, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zhseqr(matrix_order: c_int, job: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_void, ldh: c_int, w: *mut c_void, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_clacgv(n: c_int, x: *mut c_void, incx: c_int) -> c_int;
    pub fn LAPACKE_zlacgv(n: c_int, x: *mut c_void, incx: c_int) -> c_int;

    pub fn LAPACKE_slacn2(n: c_int, v: *mut c_float, x: *mut c_float, isgn: *mut c_int, est: *mut c_float, kase: *mut c_int, isave: *mut c_int) -> c_int;
    pub fn LAPACKE_dlacn2(n: c_int, v: *mut c_double, x: *mut c_double, isgn: *mut c_int, est: *mut c_double, kase: *mut c_int, isave: *mut c_int) -> c_int;
    pub fn LAPACKE_clacn2(n: c_int, v: *mut c_void, x: *mut c_void, est: *mut c_float, kase: *mut c_int, isave: *mut c_int) -> c_int;
    pub fn LAPACKE_zlacn2(n: c_int, v: *mut c_void, x: *mut c_void, est: *mut c_double, kase: *mut c_int, isave: *mut c_int) -> c_int;

    pub fn LAPACKE_slacpy(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_float, lda: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dlacpy(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_double, lda: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_clacpy(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zlacpy(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_clacp2(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_float, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zlacp2(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_double, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_zlag2c(matrix_order: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, sa: *mut c_void, ldsa: c_int) -> c_int;

    pub fn LAPACKE_slag2d(matrix_order: c_int, m: c_int, n: c_int, sa: *const c_float, ldsa: c_int, a: *mut c_double, lda: c_int) -> c_int;

    pub fn LAPACKE_dlag2s(matrix_order: c_int, m: c_int, n: c_int, a: *const c_double, lda: c_int, sa: *mut c_float, ldsa: c_int) -> c_int;

    pub fn LAPACKE_clag2z(matrix_order: c_int, m: c_int, n: c_int, sa: *const c_void, ldsa: c_int, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_slagge(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, d: *const c_float, a: *mut c_float, lda: c_int, iseed: *mut c_int) -> c_int;
    pub fn LAPACKE_dlagge(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, d: *const c_double, a: *mut c_double, lda: c_int, iseed: *mut c_int) -> c_int;
    pub fn LAPACKE_clagge(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, d: *const c_float, a: *mut c_void, lda: c_int, iseed: *mut c_int) -> c_int;
    pub fn LAPACKE_zlagge(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, d: *const c_double, a: *mut c_void, lda: c_int, iseed: *mut c_int) -> c_int;

    pub fn LAPACKE_slamch(cmach: c_char) -> c_float;
    pub fn LAPACKE_dlamch(cmach: c_char) -> c_double;

    pub fn LAPACKE_slange(matrix_order: c_int, norm: c_char, m: c_int, n: c_int, a: *const c_float, lda: c_int) -> c_float;
    pub fn LAPACKE_dlange(matrix_order: c_int, norm: c_char, m: c_int, n: c_int, a: *const c_double, lda: c_int) -> c_double;
    pub fn LAPACKE_clange(matrix_order: c_int, norm: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int) -> c_float;
    pub fn LAPACKE_zlange(matrix_order: c_int, norm: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int) -> c_double;

    pub fn LAPACKE_clanhe(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int) -> c_float;
    pub fn LAPACKE_zlanhe(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int) -> c_double;

    pub fn LAPACKE_slansy(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_float, lda: c_int) -> c_float;
    pub fn LAPACKE_dlansy(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_double, lda: c_int) -> c_double;
    pub fn LAPACKE_clansy(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int) -> c_float;
    pub fn LAPACKE_zlansy(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int) -> c_double;

    pub fn LAPACKE_slantr(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, m: c_int, n: c_int, a: *const c_float, lda: c_int) -> c_float;
    pub fn LAPACKE_dlantr(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, m: c_int, n: c_int, a: *const c_double, lda: c_int) -> c_double;
    pub fn LAPACKE_clantr(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int) -> c_float;
    pub fn LAPACKE_zlantr(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int) -> c_double;

    pub fn LAPACKE_slarfb(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, v: *const c_float, ldv: c_int, t: *const c_float, ldt: c_int, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dlarfb(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, v: *const c_double, ldv: c_int, t: *const c_double, ldt: c_int, c: *mut c_double, ldc: c_int) -> c_int;
    pub fn LAPACKE_clarfb(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zlarfb(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_slarfg(n: c_int, alpha: *mut c_float, x: *mut c_float, incx: c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dlarfg(n: c_int, alpha: *mut c_double, x: *mut c_double, incx: c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_clarfg(n: c_int, alpha: *mut c_void, x: *mut c_void, incx: c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zlarfg(n: c_int, alpha: *mut c_void, x: *mut c_void, incx: c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_slarft(matrix_order: c_int, direct: c_char, storev: c_char, n: c_int, k: c_int, v: *const c_float, ldv: c_int, tau: *const c_float, t: *mut c_float, ldt: c_int) -> c_int;
    pub fn LAPACKE_dlarft(matrix_order: c_int, direct: c_char, storev: c_char, n: c_int, k: c_int, v: *const c_double, ldv: c_int, tau: *const c_double, t: *mut c_double, ldt: c_int) -> c_int;
    pub fn LAPACKE_clarft(matrix_order: c_int, direct: c_char, storev: c_char, n: c_int, k: c_int, v: *const c_void, ldv: c_int, tau: *const c_void, t: *mut c_void, ldt: c_int) -> c_int;
    pub fn LAPACKE_zlarft(matrix_order: c_int, direct: c_char, storev: c_char, n: c_int, k: c_int, v: *const c_void, ldv: c_int, tau: *const c_void, t: *mut c_void, ldt: c_int) -> c_int;

    pub fn LAPACKE_slarfx(matrix_order: c_int, side: c_char, m: c_int, n: c_int, v: *const c_float, tau: c_float, c: *mut c_float, ldc: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dlarfx(matrix_order: c_int, side: c_char, m: c_int, n: c_int, v: *const c_double, tau: c_double, c: *mut c_double, ldc: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_clarfx(matrix_order: c_int, side: c_char, m: c_int, n: c_int, v: *const c_void, tau: c_void, c: *mut c_void, ldc: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zlarfx(matrix_order: c_int, side: c_char, m: c_int, n: c_int, v: *const c_void, tau: c_void, c: *mut c_void, ldc: c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_slarnv(idist: c_int, iseed: *mut c_int, n: c_int, x: *mut c_float) -> c_int;
    pub fn LAPACKE_dlarnv(idist: c_int, iseed: *mut c_int, n: c_int, x: *mut c_double) -> c_int;
    pub fn LAPACKE_clarnv(idist: c_int, iseed: *mut c_int, n: c_int, x: *mut c_void) -> c_int;
    pub fn LAPACKE_zlarnv(idist: c_int, iseed: *mut c_int, n: c_int, x: *mut c_void) -> c_int;

    pub fn LAPACKE_slaset(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, alpha: c_float, beta: c_float, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dlaset(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, alpha: c_double, beta: c_double, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_claset(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, alpha: c_void, beta: c_void, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_zlaset(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, alpha: c_void, beta: c_void, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_slasrt(id: c_char, n: c_int, d: *mut c_float) -> c_int;
    pub fn LAPACKE_dlasrt(id: c_char, n: c_int, d: *mut c_double) -> c_int;

    pub fn LAPACKE_slaswp(matrix_order: c_int, n: c_int, a: *mut c_float, lda: c_int, k1: c_int, k2: c_int, ipiv: *const c_int, incx: c_int) -> c_int;
    pub fn LAPACKE_dlaswp(matrix_order: c_int, n: c_int, a: *mut c_double, lda: c_int, k1: c_int, k2: c_int, ipiv: *const c_int, incx: c_int) -> c_int;
    pub fn LAPACKE_claswp(matrix_order: c_int, n: c_int, a: *mut c_void, lda: c_int, k1: c_int, k2: c_int, ipiv: *const c_int, incx: c_int) -> c_int;
    pub fn LAPACKE_zlaswp(matrix_order: c_int, n: c_int, a: *mut c_void, lda: c_int, k1: c_int, k2: c_int, ipiv: *const c_int, incx: c_int) -> c_int;

    pub fn LAPACKE_slatms(matrix_order: c_int, m: c_int, n: c_int, dist: c_char, iseed: *mut c_int, sym: c_char, d: *mut c_float, mode: c_int, cond: c_float, dmax: c_float, kl: c_int, ku: c_int, pack: c_char, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dlatms(matrix_order: c_int, m: c_int, n: c_int, dist: c_char, iseed: *mut c_int, sym: c_char, d: *mut c_double, mode: c_int, cond: c_double, dmax: c_double, kl: c_int, ku: c_int, pack: c_char, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_clatms(matrix_order: c_int, m: c_int, n: c_int, dist: c_char, iseed: *mut c_int, sym: c_char, d: *mut c_float, mode: c_int, cond: c_float, dmax: c_float, kl: c_int, ku: c_int, pack: c_char, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_zlatms(matrix_order: c_int, m: c_int, n: c_int, dist: c_char, iseed: *mut c_int, sym: c_char, d: *mut c_double, mode: c_int, cond: c_double, dmax: c_double, kl: c_int, ku: c_int, pack: c_char, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_slauum(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dlauum(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_clauum(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_zlauum(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_sopgtr(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_float, tau: *const c_float, q: *mut c_float, ldq: c_int) -> c_int;
    pub fn LAPACKE_dopgtr(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_double, tau: *const c_double, q: *mut c_double, ldq: c_int) -> c_int;

    pub fn LAPACKE_sopmtr(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, ap: *const c_float, tau: *const c_float, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dopmtr(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, ap: *const c_double, tau: *const c_double, c: *mut c_double, ldc: c_int) -> c_int;

    pub fn LAPACKE_sorgbr(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, k: c_int, a: *mut c_float, lda: c_int, tau: *const c_float) -> c_int;
    pub fn LAPACKE_dorgbr(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, k: c_int, a: *mut c_double, lda: c_int, tau: *const c_double) -> c_int;

    pub fn LAPACKE_sorghr(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_float, lda: c_int, tau: *const c_float) -> c_int;
    pub fn LAPACKE_dorghr(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_double, lda: c_int, tau: *const c_double) -> c_int;

    pub fn LAPACKE_sorglq(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_float, lda: c_int, tau: *const c_float) -> c_int;
    pub fn LAPACKE_dorglq(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_double, lda: c_int, tau: *const c_double) -> c_int;

    pub fn LAPACKE_sorgql(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_float, lda: c_int, tau: *const c_float) -> c_int;
    pub fn LAPACKE_dorgql(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_double, lda: c_int, tau: *const c_double) -> c_int;

    pub fn LAPACKE_sorgqr(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_float, lda: c_int, tau: *const c_float) -> c_int;
    pub fn LAPACKE_dorgqr(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_double, lda: c_int, tau: *const c_double) -> c_int;

    pub fn LAPACKE_sorgrq(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_float, lda: c_int, tau: *const c_float) -> c_int;
    pub fn LAPACKE_dorgrq(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_double, lda: c_int, tau: *const c_double) -> c_int;

    pub fn LAPACKE_sorgtr(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, tau: *const c_float) -> c_int;
    pub fn LAPACKE_dorgtr(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, tau: *const c_double) -> c_int;

    pub fn LAPACKE_sormbr(matrix_order: c_int, vect: c_char, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dormbr(matrix_order: c_int, vect: c_char, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int) -> c_int;

    pub fn LAPACKE_sormhr(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dormhr(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int) -> c_int;

    pub fn LAPACKE_sormlq(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dormlq(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int) -> c_int;

    pub fn LAPACKE_sormql(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dormql(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int) -> c_int;

    pub fn LAPACKE_sormqr(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dormqr(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int) -> c_int;

    pub fn LAPACKE_sormrq(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dormrq(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int) -> c_int;

    pub fn LAPACKE_sormrz(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dormrz(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int) -> c_int;

    pub fn LAPACKE_sormtr(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dormtr(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int) -> c_int;

    pub fn LAPACKE_spbcon(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_float, ldab: c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dpbcon(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_double, ldab: c_int, anorm: c_double, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_cpbcon(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_zpbcon(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, anorm: c_double, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_spbequ(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_float, ldab: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dpbequ(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_double, ldab: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cpbequ(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zpbequ(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_spbrfs(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, afb: *const c_float, ldafb: c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dpbrfs(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, afb: *const c_double, ldafb: c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cpbrfs(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zpbrfs(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_spbstf(matrix_order: c_int, uplo: c_char, n: c_int, kb: c_int, bb: *mut c_float, ldbb: c_int) -> c_int;
    pub fn LAPACKE_dpbstf(matrix_order: c_int, uplo: c_char, n: c_int, kb: c_int, bb: *mut c_double, ldbb: c_int) -> c_int;
    pub fn LAPACKE_cpbstf(matrix_order: c_int, uplo: c_char, n: c_int, kb: c_int, bb: *mut c_void, ldbb: c_int) -> c_int;
    pub fn LAPACKE_zpbstf(matrix_order: c_int, uplo: c_char, n: c_int, kb: c_int, bb: *mut c_void, ldbb: c_int) -> c_int;

    pub fn LAPACKE_spbsv(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_float, ldab: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpbsv(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_double, ldab: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpbsv(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpbsv(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_spbsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_float, ldab: c_int, afb: *mut c_float, ldafb: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dpbsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_double, ldab: c_int, afb: *mut c_double, ldafb: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cpbsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zpbsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_spbtrf(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_float, ldab: c_int) -> c_int;
    pub fn LAPACKE_dpbtrf(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_double, ldab: c_int) -> c_int;
    pub fn LAPACKE_cpbtrf(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int) -> c_int;
    pub fn LAPACKE_zpbtrf(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int) -> c_int;

    pub fn LAPACKE_spbtrs(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpbtrs(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpbtrs(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpbtrs(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_spftrf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_float) -> c_int;
    pub fn LAPACKE_dpftrf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_double) -> c_int;
    pub fn LAPACKE_cpftrf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_void) -> c_int;
    pub fn LAPACKE_zpftrf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_void) -> c_int;

    pub fn LAPACKE_spftri(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_float) -> c_int;
    pub fn LAPACKE_dpftri(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_double) -> c_int;
    pub fn LAPACKE_cpftri(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_void) -> c_int;
    pub fn LAPACKE_zpftri(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_void) -> c_int;

    pub fn LAPACKE_spftrs(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpftrs(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpftrs(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpftrs(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_spocon(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_float, lda: c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dpocon(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_double, lda: c_int, anorm: c_double, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_cpocon(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_zpocon(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, anorm: c_double, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_spoequ(matrix_order: c_int, n: c_int, a: *const c_float, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dpoequ(matrix_order: c_int, n: c_int, a: *const c_double, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cpoequ(matrix_order: c_int, n: c_int, a: *const c_void, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zpoequ(matrix_order: c_int, n: c_int, a: *const c_void, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_spoequb(matrix_order: c_int, n: c_int, a: *const c_float, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dpoequb(matrix_order: c_int, n: c_int, a: *const c_double, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cpoequb(matrix_order: c_int, n: c_int, a: *const c_void, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zpoequb(matrix_order: c_int, n: c_int, a: *const c_void, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_sporfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dporfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cporfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zporfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_sporfsx(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, s: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_dporfsx(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, s: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;
    pub fn LAPACKE_cporfsx(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, s: *const c_float, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_zporfsx(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, s: *const c_double, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;

    pub fn LAPACKE_sposv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dposv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cposv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zposv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_dsposv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, iter: *mut c_int) -> c_int;
    pub fn LAPACKE_zcposv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, iter: *mut c_int) -> c_int;

    pub fn LAPACKE_sposvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, af: *mut c_float, ldaf: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dposvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, af: *mut c_double, ldaf: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cposvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zposvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_sposvxx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, af: *mut c_float, ldaf: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_dposvxx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, af: *mut c_double, ldaf: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;
    pub fn LAPACKE_cposvxx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_zposvxx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;

    pub fn LAPACKE_spotrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dpotrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_cpotrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_zpotrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_spotri(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dpotri(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_cpotri(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_zpotri(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_spotrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpotrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpotrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpotrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sppcon(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_float, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dppcon(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_double, anorm: c_double, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_cppcon(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_zppcon(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, anorm: c_double, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_sppequ(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_float, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dppequ(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_double, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cppequ(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zppequ(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_spprfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_float, afp: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dpprfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_double, afp: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cpprfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zpprfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_sppsv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dppsv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cppsv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zppsv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sppsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_float, afp: *mut c_float, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dppsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_double, afp: *mut c_double, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cppsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, afp: *mut c_void, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zppsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, afp: *mut c_void, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_spptrf(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_float) -> c_int;
    pub fn LAPACKE_dpptrf(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_double) -> c_int;
    pub fn LAPACKE_cpptrf(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void) -> c_int;
    pub fn LAPACKE_zpptrf(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void) -> c_int;

    pub fn LAPACKE_spptri(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_float) -> c_int;
    pub fn LAPACKE_dpptri(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_double) -> c_int;
    pub fn LAPACKE_cpptri(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void) -> c_int;
    pub fn LAPACKE_zpptri(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void) -> c_int;

    pub fn LAPACKE_spptrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpptrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpptrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpptrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_spstrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, piv: *mut c_int, rank: *mut c_int, tol: c_float) -> c_int;
    pub fn LAPACKE_dpstrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, piv: *mut c_int, rank: *mut c_int, tol: c_double) -> c_int;
    pub fn LAPACKE_cpstrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, piv: *mut c_int, rank: *mut c_int, tol: c_float) -> c_int;
    pub fn LAPACKE_zpstrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, piv: *mut c_int, rank: *mut c_int, tol: c_double) -> c_int;

    pub fn LAPACKE_sptcon(n: c_int, d: *const c_float, e: *const c_float, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dptcon(n: c_int, d: *const c_double, e: *const c_double, anorm: c_double, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_cptcon(n: c_int, d: *const c_float, e: *const c_void, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_zptcon(n: c_int, d: *const c_double, e: *const c_void, anorm: c_double, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_spteqr(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dpteqr(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;
    pub fn LAPACKE_cpteqr(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zpteqr(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_sptrfs(matrix_order: c_int, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_float, df: *const c_float, ef: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dptrfs(matrix_order: c_int, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_double, df: *const c_double, ef: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cptrfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_void, df: *const c_float, ef: *const c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zptrfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_void, df: *const c_double, ef: *const c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_sptsv(matrix_order: c_int, n: c_int, nrhs: c_int, d: *mut c_float, e: *mut c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dptsv(matrix_order: c_int, n: c_int, nrhs: c_int, d: *mut c_double, e: *mut c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cptsv(matrix_order: c_int, n: c_int, nrhs: c_int, d: *mut c_float, e: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zptsv(matrix_order: c_int, n: c_int, nrhs: c_int, d: *mut c_double, e: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sptsvx(matrix_order: c_int, fact: c_char, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_float, df: *mut c_float, ef: *mut c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dptsvx(matrix_order: c_int, fact: c_char, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_double, df: *mut c_double, ef: *mut c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cptsvx(matrix_order: c_int, fact: c_char, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_void, df: *mut c_float, ef: *mut c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zptsvx(matrix_order: c_int, fact: c_char, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_void, df: *mut c_double, ef: *mut c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_spttrf(n: c_int, d: *mut c_float, e: *mut c_float) -> c_int;
    pub fn LAPACKE_dpttrf(n: c_int, d: *mut c_double, e: *mut c_double) -> c_int;
    pub fn LAPACKE_cpttrf(n: c_int, d: *mut c_float, e: *mut c_void) -> c_int;
    pub fn LAPACKE_zpttrf(n: c_int, d: *mut c_double, e: *mut c_void) -> c_int;

    pub fn LAPACKE_spttrs(matrix_order: c_int, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpttrs(matrix_order: c_int, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpttrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpttrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_ssbev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_float, ldab: c_int, w: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dsbev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_double, ldab: c_int, w: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;

    pub fn LAPACKE_ssbevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_float, ldab: c_int, w: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dsbevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_double, ldab: c_int, w: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;

    pub fn LAPACKE_ssbevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_float, ldab: c_int, q: *mut c_float, ldq: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dsbevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_double, ldab: c_int, q: *mut c_double, ldq: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssbgst(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_float, ldab: c_int, bb: *const c_float, ldbb: c_int, x: *mut c_float, ldx: c_int) -> c_int;
    pub fn LAPACKE_dsbgst(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_double, ldab: c_int, bb: *const c_double, ldbb: c_int, x: *mut c_double, ldx: c_int) -> c_int;

    pub fn LAPACKE_ssbgv(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_float, ldab: c_int, bb: *mut c_float, ldbb: c_int, w: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dsbgv(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_double, ldab: c_int, bb: *mut c_double, ldbb: c_int, w: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;

    pub fn LAPACKE_ssbgvd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_float, ldab: c_int, bb: *mut c_float, ldbb: c_int, w: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dsbgvd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_double, ldab: c_int, bb: *mut c_double, ldbb: c_int, w: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;

    pub fn LAPACKE_ssbgvx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_float, ldab: c_int, bb: *mut c_float, ldbb: c_int, q: *mut c_float, ldq: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dsbgvx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_double, ldab: c_int, bb: *mut c_double, ldbb: c_int, q: *mut c_double, ldq: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssbtrd(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_float, ldab: c_int, d: *mut c_float, e: *mut c_float, q: *mut c_float, ldq: c_int) -> c_int;
    pub fn LAPACKE_dsbtrd(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_double, ldab: c_int, d: *mut c_double, e: *mut c_double, q: *mut c_double, ldq: c_int) -> c_int;

    pub fn LAPACKE_ssfrk(matrix_order: c_int, transr: c_char, uplo: c_char, trans: c_char, n: c_int, k: c_int, alpha: c_float, a: *const c_float, lda: c_int, beta: c_float, c: *mut c_float) -> c_int;
    pub fn LAPACKE_dsfrk(matrix_order: c_int, transr: c_char, uplo: c_char, trans: c_char, n: c_int, k: c_int, alpha: c_double, a: *const c_double, lda: c_int, beta: c_double, c: *mut c_double) -> c_int;

    pub fn LAPACKE_sspcon(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_float, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dspcon(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_double, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_cspcon(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_zspcon(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_sspev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dspev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;

    pub fn LAPACKE_sspevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dspevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;

    pub fn LAPACKE_sspevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dspevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_sspgst(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, ap: *mut c_float, bp: *const c_float) -> c_int;
    pub fn LAPACKE_dspgst(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, ap: *mut c_double, bp: *const c_double) -> c_int;

    pub fn LAPACKE_sspgv(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dspgv(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_double, bp: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;

    pub fn LAPACKE_sspgvd(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dspgvd(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_double, bp: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;

    pub fn LAPACKE_sspgvx(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_float, bp: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dspgvx(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_double, bp: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssprfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_float, afp: *const c_float, ipiv: *const c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dsprfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_double, afp: *const c_double, ipiv: *const c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_csprfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zsprfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_sspsv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_float, ipiv: *mut c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dspsv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_double, ipiv: *mut c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cspsv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zspsv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sspsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_float, afp: *mut c_float, ipiv: *mut c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dspsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_double, afp: *mut c_double, ipiv: *mut c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_cspsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zspsvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_ssptrd(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_float, d: *mut c_float, e: *mut c_float, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dsptrd(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_double, d: *mut c_double, e: *mut c_double, tau: *mut c_double) -> c_int;

    pub fn LAPACKE_ssptrf(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_float, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_dsptrf(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_double, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_csptrf(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zsptrf(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_ssptri(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_float, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_dsptri(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_double, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_csptri(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_zsptri(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *const c_int) -> c_int;

    pub fn LAPACKE_ssptrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_float, ipiv: *const c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dsptrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_double, ipiv: *const c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_csptrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zsptrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sstebz(range: c_char, order: c_char, n: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, d: *const c_float, e: *const c_float, m: *mut c_int, nsplit: *mut c_int, w: *mut c_float, iblock: *mut c_int, isplit: *mut c_int) -> c_int;
    pub fn LAPACKE_dstebz(range: c_char, order: c_char, n: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, d: *const c_double, e: *const c_double, m: *mut c_int, nsplit: *mut c_int, w: *mut c_double, iblock: *mut c_int, isplit: *mut c_int) -> c_int;

    pub fn LAPACKE_sstedc(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dstedc(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;
    pub fn LAPACKE_cstedc(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zstedc(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_sstegr(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, isuppz: *mut c_int) -> c_int;
    pub fn LAPACKE_dstegr(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, isuppz: *mut c_int) -> c_int;
    pub fn LAPACKE_cstegr(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, isuppz: *mut c_int) -> c_int;
    pub fn LAPACKE_zstegr(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, isuppz: *mut c_int) -> c_int;

    pub fn LAPACKE_sstein(matrix_order: c_int, n: c_int, d: *const c_float, e: *const c_float, m: c_int, w: *const c_float, iblock: *const c_int, isplit: *const c_int, z: *mut c_float, ldz: c_int, ifailv: *mut c_int) -> c_int;
    pub fn LAPACKE_dstein(matrix_order: c_int, n: c_int, d: *const c_double, e: *const c_double, m: c_int, w: *const c_double, iblock: *const c_int, isplit: *const c_int, z: *mut c_double, ldz: c_int, ifailv: *mut c_int) -> c_int;
    pub fn LAPACKE_cstein(matrix_order: c_int, n: c_int, d: *const c_float, e: *const c_float, m: c_int, w: *const c_float, iblock: *const c_int, isplit: *const c_int, z: *mut c_void, ldz: c_int, ifailv: *mut c_int) -> c_int;
    pub fn LAPACKE_zstein(matrix_order: c_int, n: c_int, d: *const c_double, e: *const c_double, m: c_int, w: *const c_double, iblock: *const c_int, isplit: *const c_int, z: *mut c_void, ldz: c_int, ifailv: *mut c_int) -> c_int;

    pub fn LAPACKE_sstemr(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, nzc: c_int, isuppz: *mut c_int, tryrac: *mut c_int) -> c_int;
    pub fn LAPACKE_dstemr(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, nzc: c_int, isuppz: *mut c_int, tryrac: *mut c_int) -> c_int;
    pub fn LAPACKE_cstemr(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, nzc: c_int, isuppz: *mut c_int, tryrac: *mut c_int) -> c_int;
    pub fn LAPACKE_zstemr(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, nzc: c_int, isuppz: *mut c_int, tryrac: *mut c_int) -> c_int;

    pub fn LAPACKE_ssteqr(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dsteqr(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;
    pub fn LAPACKE_csteqr(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zsteqr(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_ssterf(n: c_int, d: *mut c_float, e: *mut c_float) -> c_int;
    pub fn LAPACKE_dsterf(n: c_int, d: *mut c_double, e: *mut c_double) -> c_int;

    pub fn LAPACKE_sstev(matrix_order: c_int, jobz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dstev(matrix_order: c_int, jobz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;

    pub fn LAPACKE_sstevd(matrix_order: c_int, jobz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dstevd(matrix_order: c_int, jobz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: c_int) -> c_int;

    pub fn LAPACKE_sstevr(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, isuppz: *mut c_int) -> c_int;
    pub fn LAPACKE_dstevr(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, isuppz: *mut c_int) -> c_int;

    pub fn LAPACKE_sstevx(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dstevx(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssycon(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_float, lda: c_int, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dsycon(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_double, lda: c_int, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_csycon(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_zsycon(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_ssyequb(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_float, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dsyequb(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_double, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_csyequb(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zsyequb(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_ssyev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, w: *mut c_float) -> c_int;
    pub fn LAPACKE_dsyev(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, w: *mut c_double) -> c_int;

    pub fn LAPACKE_ssyevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, w: *mut c_float) -> c_int;
    pub fn LAPACKE_dsyevd(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, w: *mut c_double) -> c_int;

    pub fn LAPACKE_ssyevr(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, isuppz: *mut c_int) -> c_int;
    pub fn LAPACKE_dsyevr(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, isuppz: *mut c_int) -> c_int;

    pub fn LAPACKE_ssyevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dsyevx(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssygst(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *const c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dsygst(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *const c_double, ldb: c_int) -> c_int;

    pub fn LAPACKE_ssygv(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, w: *mut c_float) -> c_int;
    pub fn LAPACKE_dsygv(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, w: *mut c_double) -> c_int;

    pub fn LAPACKE_ssygvd(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, w: *mut c_float) -> c_int;
    pub fn LAPACKE_dsygvd(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, w: *mut c_double) -> c_int;

    pub fn LAPACKE_ssygvx(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dsygvx(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssyrfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, ipiv: *const c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dsyrfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, ipiv: *const c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_csyrfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zsyrfs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_ssyrfsx(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, ipiv: *const c_int, s: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_dsyrfsx(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, ipiv: *const c_int, s: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;
    pub fn LAPACKE_csyrfsx(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, s: *const c_float, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_zsyrfsx(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, s: *const c_double, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;

    pub fn LAPACKE_ssysv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dsysv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_csysv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zsysv(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_ssysvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *mut c_float, ldaf: c_int, ipiv: *mut c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dsysvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *mut c_double, ldaf: c_int, ipiv: *mut c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_csysvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_zsysvx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_ssysvxx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, af: *mut c_float, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_dsysvxx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, af: *mut c_double, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;
    pub fn LAPACKE_csysvxx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float) -> c_int;
    pub fn LAPACKE_zsysvxx(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double) -> c_int;

    pub fn LAPACKE_ssytrd(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, d: *mut c_float, e: *mut c_float, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dsytrd(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, d: *mut c_double, e: *mut c_double, tau: *mut c_double) -> c_int;

    pub fn LAPACKE_ssytrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_dsytrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_csytrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zsytrf(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_ssytri(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_dsytri(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_csytri(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_zsytri(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;

    pub fn LAPACKE_ssytrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, ipiv: *const c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dsytrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, ipiv: *const c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_csytrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zsytrs(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_stbcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, kd: c_int, ab: *const c_float, ldab: c_int, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dtbcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, kd: c_int, ab: *const c_double, ldab: c_int, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_ctbcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_ztbcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_stbrfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, b: *const c_float, ldb: c_int, x: *const c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dtbrfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, b: *const c_double, ldb: c_int, x: *const c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_ctbrfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_ztbrfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_stbtrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dtbtrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_ctbtrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_ztbtrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_stfsm(matrix_order: c_int, transr: c_char, side: c_char, uplo: c_char, trans: c_char, diag: c_char, m: c_int, n: c_int, alpha: c_float, a: *const c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dtfsm(matrix_order: c_int, transr: c_char, side: c_char, uplo: c_char, trans: c_char, diag: c_char, m: c_int, n: c_int, alpha: c_double, a: *const c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_ctfsm(matrix_order: c_int, transr: c_char, side: c_char, uplo: c_char, trans: c_char, diag: c_char, m: c_int, n: c_int, alpha: c_void, a: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_ztfsm(matrix_order: c_int, transr: c_char, side: c_char, uplo: c_char, trans: c_char, diag: c_char, m: c_int, n: c_int, alpha: c_void, a: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_stftri(matrix_order: c_int, transr: c_char, uplo: c_char, diag: c_char, n: c_int, a: *mut c_float) -> c_int;
    pub fn LAPACKE_dtftri(matrix_order: c_int, transr: c_char, uplo: c_char, diag: c_char, n: c_int, a: *mut c_double) -> c_int;
    pub fn LAPACKE_ctftri(matrix_order: c_int, transr: c_char, uplo: c_char, diag: c_char, n: c_int, a: *mut c_void) -> c_int;
    pub fn LAPACKE_ztftri(matrix_order: c_int, transr: c_char, uplo: c_char, diag: c_char, n: c_int, a: *mut c_void) -> c_int;

    pub fn LAPACKE_stfttp(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_float, ap: *mut c_float) -> c_int;
    pub fn LAPACKE_dtfttp(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_double, ap: *mut c_double) -> c_int;
    pub fn LAPACKE_ctfttp(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_void, ap: *mut c_void) -> c_int;
    pub fn LAPACKE_ztfttp(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_void, ap: *mut c_void) -> c_int;

    pub fn LAPACKE_stfttr(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_float, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dtfttr(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_double, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_ctfttr(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_void, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_ztfttr(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_void, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_stgevc(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, s: *const c_float, lds: c_int, p: *const c_float, ldp: c_int, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_dtgevc(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, s: *const c_double, lds: c_int, p: *const c_double, ldp: c_int, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_ctgevc(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, s: *const c_void, lds: c_int, p: *const c_void, ldp: c_int, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_ztgevc(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, s: *const c_void, lds: c_int, p: *const c_void, ldp: c_int, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int) -> c_int;

    pub fn LAPACKE_stgexc(matrix_order: c_int, wantq: c_int, wantz: c_int, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, q: *mut c_float, ldq: c_int, z: *mut c_float, ldz: c_int, ifst: *mut c_int, ilst: *mut c_int) -> c_int;
    pub fn LAPACKE_dtgexc(matrix_order: c_int, wantq: c_int, wantz: c_int, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, q: *mut c_double, ldq: c_int, z: *mut c_double, ldz: c_int, ifst: *mut c_int, ilst: *mut c_int) -> c_int;
    pub fn LAPACKE_ctgexc(matrix_order: c_int, wantq: c_int, wantz: c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int, ifst: c_int, ilst: c_int) -> c_int;
    pub fn LAPACKE_ztgexc(matrix_order: c_int, wantq: c_int, wantz: c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int, ifst: c_int, ilst: c_int) -> c_int;

    pub fn LAPACKE_stgsen(matrix_order: c_int, ijob: c_int, wantq: c_int, wantz: c_int, select: *const c_int, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, q: *mut c_float, ldq: c_int, z: *mut c_float, ldz: c_int, m: *mut c_int, pl: *mut c_float, pr: *mut c_float, dif: *mut c_float) -> c_int;
    pub fn LAPACKE_dtgsen(matrix_order: c_int, ijob: c_int, wantq: c_int, wantz: c_int, select: *const c_int, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, q: *mut c_double, ldq: c_int, z: *mut c_double, ldz: c_int, m: *mut c_int, pl: *mut c_double, pr: *mut c_double, dif: *mut c_double) -> c_int;
    pub fn LAPACKE_ctgsen(matrix_order: c_int, ijob: c_int, wantq: c_int, wantz: c_int, select: *const c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int, m: *mut c_int, pl: *mut c_float, pr: *mut c_float, dif: *mut c_float) -> c_int;
    pub fn LAPACKE_ztgsen(matrix_order: c_int, ijob: c_int, wantq: c_int, wantz: c_int, select: *const c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int, m: *mut c_int, pl: *mut c_double, pr: *mut c_double, dif: *mut c_double) -> c_int;

    pub fn LAPACKE_stgsja(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, k: c_int, l: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, tola: c_float, tolb: c_float, alpha: *mut c_float, beta: *mut c_float, u: *mut c_float, ldu: c_int, v: *mut c_float, ldv: c_int, q: *mut c_float, ldq: c_int, ncycle: *mut c_int) -> c_int;
    pub fn LAPACKE_dtgsja(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, k: c_int, l: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, tola: c_double, tolb: c_double, alpha: *mut c_double, beta: *mut c_double, u: *mut c_double, ldu: c_int, v: *mut c_double, ldv: c_int, q: *mut c_double, ldq: c_int, ncycle: *mut c_int) -> c_int;
    pub fn LAPACKE_ctgsja(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, k: c_int, l: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, tola: c_float, tolb: c_float, alpha: *mut c_float, beta: *mut c_float, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int, ncycle: *mut c_int) -> c_int;
    pub fn LAPACKE_ztgsja(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, k: c_int, l: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, tola: c_double, tolb: c_double, alpha: *mut c_double, beta: *mut c_double, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int, ncycle: *mut c_int) -> c_int;

    pub fn LAPACKE_stgsna(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, a: *const c_float, lda: c_int, b: *const c_float, ldb: c_int, vl: *const c_float, ldvl: c_int, vr: *const c_float, ldvr: c_int, s: *mut c_float, dif: *mut c_float, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_dtgsna(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, a: *const c_double, lda: c_int, b: *const c_double, ldb: c_int, vl: *const c_double, ldvl: c_int, vr: *const c_double, ldvr: c_int, s: *mut c_double, dif: *mut c_double, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_ctgsna(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, vl: *const c_void, ldvl: c_int, vr: *const c_void, ldvr: c_int, s: *mut c_float, dif: *mut c_float, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_ztgsna(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, vl: *const c_void, ldvl: c_int, vr: *const c_void, ldvr: c_int, s: *mut c_double, dif: *mut c_double, mm: c_int, m: *mut c_int) -> c_int;

    pub fn LAPACKE_stgsyl(matrix_order: c_int, trans: c_char, ijob: c_int, m: c_int, n: c_int, a: *const c_float, lda: c_int, b: *const c_float, ldb: c_int, c: *mut c_float, ldc: c_int, d: *const c_float, ldd: c_int, e: *const c_float, lde: c_int, f: *mut c_float, ldf: c_int, scale: *mut c_float, dif: *mut c_float) -> c_int;
    pub fn LAPACKE_dtgsyl(matrix_order: c_int, trans: c_char, ijob: c_int, m: c_int, n: c_int, a: *const c_double, lda: c_int, b: *const c_double, ldb: c_int, c: *mut c_double, ldc: c_int, d: *const c_double, ldd: c_int, e: *const c_double, lde: c_int, f: *mut c_double, ldf: c_int, scale: *mut c_double, dif: *mut c_double) -> c_int;
    pub fn LAPACKE_ctgsyl(matrix_order: c_int, trans: c_char, ijob: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, c: *mut c_void, ldc: c_int, d: *const c_void, ldd: c_int, e: *const c_void, lde: c_int, f: *mut c_void, ldf: c_int, scale: *mut c_float, dif: *mut c_float) -> c_int;
    pub fn LAPACKE_ztgsyl(matrix_order: c_int, trans: c_char, ijob: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, c: *mut c_void, ldc: c_int, d: *const c_void, ldd: c_int, e: *const c_void, lde: c_int, f: *mut c_void, ldf: c_int, scale: *mut c_double, dif: *mut c_double) -> c_int;

    pub fn LAPACKE_stpcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, ap: *const c_float, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dtpcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, ap: *const c_double, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_ctpcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, ap: *const c_void, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_ztpcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, ap: *const c_void, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_stprfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_float, b: *const c_float, ldb: c_int, x: *const c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dtprfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_double, b: *const c_double, ldb: c_int, x: *const c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_ctprfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_ztprfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_stptri(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, ap: *mut c_float) -> c_int;
    pub fn LAPACKE_dtptri(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, ap: *mut c_double) -> c_int;
    pub fn LAPACKE_ctptri(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, ap: *mut c_void) -> c_int;
    pub fn LAPACKE_ztptri(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, ap: *mut c_void) -> c_int;

    pub fn LAPACKE_stptrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dtptrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_ctptrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_ztptrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_stpttf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, ap: *const c_float, arf: *mut c_float) -> c_int;
    pub fn LAPACKE_dtpttf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, ap: *const c_double, arf: *mut c_double) -> c_int;
    pub fn LAPACKE_ctpttf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, ap: *const c_void, arf: *mut c_void) -> c_int;
    pub fn LAPACKE_ztpttf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, ap: *const c_void, arf: *mut c_void) -> c_int;

    pub fn LAPACKE_stpttr(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_float, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dtpttr(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_double, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_ctpttr(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_ztpttr(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_strcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, a: *const c_float, lda: c_int, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_dtrcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, a: *const c_double, lda: c_int, rcond: *mut c_double) -> c_int;
    pub fn LAPACKE_ctrcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, a: *const c_void, lda: c_int, rcond: *mut c_float) -> c_int;
    pub fn LAPACKE_ztrcon(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, a: *const c_void, lda: c_int, rcond: *mut c_double) -> c_int;

    pub fn LAPACKE_strevc(matrix_order: c_int, side: c_char, howmny: c_char, select: *mut c_int, n: c_int, t: *const c_float, ldt: c_int, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_dtrevc(matrix_order: c_int, side: c_char, howmny: c_char, select: *mut c_int, n: c_int, t: *const c_double, ldt: c_int, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_ctrevc(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *mut c_void, ldt: c_int, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_ztrevc(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *mut c_void, ldt: c_int, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int) -> c_int;

    pub fn LAPACKE_strexc(matrix_order: c_int, compq: c_char, n: c_int, t: *mut c_float, ldt: c_int, q: *mut c_float, ldq: c_int, ifst: *mut c_int, ilst: *mut c_int) -> c_int;
    pub fn LAPACKE_dtrexc(matrix_order: c_int, compq: c_char, n: c_int, t: *mut c_double, ldt: c_int, q: *mut c_double, ldq: c_int, ifst: *mut c_int, ilst: *mut c_int) -> c_int;
    pub fn LAPACKE_ctrexc(matrix_order: c_int, compq: c_char, n: c_int, t: *mut c_void, ldt: c_int, q: *mut c_void, ldq: c_int, ifst: c_int, ilst: c_int) -> c_int;
    pub fn LAPACKE_ztrexc(matrix_order: c_int, compq: c_char, n: c_int, t: *mut c_void, ldt: c_int, q: *mut c_void, ldq: c_int, ifst: c_int, ilst: c_int) -> c_int;

    pub fn LAPACKE_strrfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, b: *const c_float, ldb: c_int, x: *const c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_dtrrfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, b: *const c_double, ldb: c_int, x: *const c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;
    pub fn LAPACKE_ctrrfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float) -> c_int;
    pub fn LAPACKE_ztrrfs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double) -> c_int;

    pub fn LAPACKE_strsen(matrix_order: c_int, job: c_char, compq: c_char, select: *const c_int, n: c_int, t: *mut c_float, ldt: c_int, q: *mut c_float, ldq: c_int, wr: *mut c_float, wi: *mut c_float, m: *mut c_int, s: *mut c_float, sep: *mut c_float) -> c_int;
    pub fn LAPACKE_dtrsen(matrix_order: c_int, job: c_char, compq: c_char, select: *const c_int, n: c_int, t: *mut c_double, ldt: c_int, q: *mut c_double, ldq: c_int, wr: *mut c_double, wi: *mut c_double, m: *mut c_int, s: *mut c_double, sep: *mut c_double) -> c_int;
    pub fn LAPACKE_ctrsen(matrix_order: c_int, job: c_char, compq: c_char, select: *const c_int, n: c_int, t: *mut c_void, ldt: c_int, q: *mut c_void, ldq: c_int, w: *mut c_void, m: *mut c_int, s: *mut c_float, sep: *mut c_float) -> c_int;
    pub fn LAPACKE_ztrsen(matrix_order: c_int, job: c_char, compq: c_char, select: *const c_int, n: c_int, t: *mut c_void, ldt: c_int, q: *mut c_void, ldq: c_int, w: *mut c_void, m: *mut c_int, s: *mut c_double, sep: *mut c_double) -> c_int;

    pub fn LAPACKE_strsna(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *const c_float, ldt: c_int, vl: *const c_float, ldvl: c_int, vr: *const c_float, ldvr: c_int, s: *mut c_float, sep: *mut c_float, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_dtrsna(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *const c_double, ldt: c_int, vl: *const c_double, ldvl: c_int, vr: *const c_double, ldvr: c_int, s: *mut c_double, sep: *mut c_double, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_ctrsna(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *const c_void, ldt: c_int, vl: *const c_void, ldvl: c_int, vr: *const c_void, ldvr: c_int, s: *mut c_float, sep: *mut c_float, mm: c_int, m: *mut c_int) -> c_int;
    pub fn LAPACKE_ztrsna(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *const c_void, ldt: c_int, vl: *const c_void, ldvl: c_int, vr: *const c_void, ldvr: c_int, s: *mut c_double, sep: *mut c_double, mm: c_int, m: *mut c_int) -> c_int;

    pub fn LAPACKE_strsyl(matrix_order: c_int, trana: c_char, tranb: c_char, isgn: c_int, m: c_int, n: c_int, a: *const c_float, lda: c_int, b: *const c_float, ldb: c_int, c: *mut c_float, ldc: c_int, scale: *mut c_float) -> c_int;
    pub fn LAPACKE_dtrsyl(matrix_order: c_int, trana: c_char, tranb: c_char, isgn: c_int, m: c_int, n: c_int, a: *const c_double, lda: c_int, b: *const c_double, ldb: c_int, c: *mut c_double, ldc: c_int, scale: *mut c_double) -> c_int;
    pub fn LAPACKE_ctrsyl(matrix_order: c_int, trana: c_char, tranb: c_char, isgn: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, c: *mut c_void, ldc: c_int, scale: *mut c_float) -> c_int;
    pub fn LAPACKE_ztrsyl(matrix_order: c_int, trana: c_char, tranb: c_char, isgn: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, c: *mut c_void, ldc: c_int, scale: *mut c_double) -> c_int;

    pub fn LAPACKE_strtri(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dtrtri(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_ctrtri(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_ztrtri(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_strtrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dtrtrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_ctrtrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_ztrtrs(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_strttf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *const c_float, lda: c_int, arf: *mut c_float) -> c_int;
    pub fn LAPACKE_dtrttf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *const c_double, lda: c_int, arf: *mut c_double) -> c_int;
    pub fn LAPACKE_ctrttf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, arf: *mut c_void) -> c_int;
    pub fn LAPACKE_ztrttf(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, arf: *mut c_void) -> c_int;

    pub fn LAPACKE_strttp(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_float, lda: c_int, ap: *mut c_float) -> c_int;
    pub fn LAPACKE_dtrttp(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_double, lda: c_int, ap: *mut c_double) -> c_int;
    pub fn LAPACKE_ctrttp(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ap: *mut c_void) -> c_int;
    pub fn LAPACKE_ztrttp(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ap: *mut c_void) -> c_int;

    pub fn LAPACKE_stzrzf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dtzrzf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_ctzrzf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_ztzrzf(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_cungbr(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;
    pub fn LAPACKE_zungbr(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;

    pub fn LAPACKE_cunghr(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;
    pub fn LAPACKE_zunghr(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;

    pub fn LAPACKE_cunglq(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;
    pub fn LAPACKE_zunglq(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;

    pub fn LAPACKE_cungql(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;
    pub fn LAPACKE_zungql(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;

    pub fn LAPACKE_cungqr(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;
    pub fn LAPACKE_zungqr(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;

    pub fn LAPACKE_cungrq(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;
    pub fn LAPACKE_zungrq(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;

    pub fn LAPACKE_cungtr(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;
    pub fn LAPACKE_zungtr(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, tau: *const c_void) -> c_int;

    pub fn LAPACKE_cunmbr(matrix_order: c_int, vect: c_char, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zunmbr(matrix_order: c_int, vect: c_char, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_cunmhr(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zunmhr(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_cunmlq(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zunmlq(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_cunmql(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zunmql(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_cunmqr(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zunmqr(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_cunmrq(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zunmrq(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_cunmrz(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zunmrz(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_cunmtr(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zunmtr(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_cupgtr(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, tau: *const c_void, q: *mut c_void, ldq: c_int) -> c_int;
    pub fn LAPACKE_zupgtr(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, tau: *const c_void, q: *mut c_void, ldq: c_int) -> c_int;

    pub fn LAPACKE_cupmtr(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, ap: *const c_void, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zupmtr(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, ap: *const c_void, tau: *const c_void, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_claghe(matrix_order: c_int, n: c_int, k: c_int, d: *const c_float, a: *mut c_void, lda: c_int, iseed: *mut c_int) -> c_int;
    pub fn LAPACKE_zlaghe(matrix_order: c_int, n: c_int, k: c_int, d: *const c_double, a: *mut c_void, lda: c_int, iseed: *mut c_int) -> c_int;

    pub fn LAPACKE_slagsy(matrix_order: c_int, n: c_int, k: c_int, d: *const c_float, a: *mut c_float, lda: c_int, iseed: *mut c_int) -> c_int;
    pub fn LAPACKE_dlagsy(matrix_order: c_int, n: c_int, k: c_int, d: *const c_double, a: *mut c_double, lda: c_int, iseed: *mut c_int) -> c_int;
    pub fn LAPACKE_clagsy(matrix_order: c_int, n: c_int, k: c_int, d: *const c_float, a: *mut c_void, lda: c_int, iseed: *mut c_int) -> c_int;
    pub fn LAPACKE_zlagsy(matrix_order: c_int, n: c_int, k: c_int, d: *const c_double, a: *mut c_void, lda: c_int, iseed: *mut c_int) -> c_int;

    pub fn LAPACKE_slapmr(matrix_order: c_int, forwrd: c_int, m: c_int, n: c_int, x: *mut c_float, ldx: c_int, k: *mut c_int) -> c_int;
    pub fn LAPACKE_dlapmr(matrix_order: c_int, forwrd: c_int, m: c_int, n: c_int, x: *mut c_double, ldx: c_int, k: *mut c_int) -> c_int;
    pub fn LAPACKE_clapmr(matrix_order: c_int, forwrd: c_int, m: c_int, n: c_int, x: *mut c_void, ldx: c_int, k: *mut c_int) -> c_int;
    pub fn LAPACKE_zlapmr(matrix_order: c_int, forwrd: c_int, m: c_int, n: c_int, x: *mut c_void, ldx: c_int, k: *mut c_int) -> c_int;

    pub fn LAPACKE_slapy2(x: c_float, y: c_float) -> c_float;
    pub fn LAPACKE_dlapy2(x: c_double, y: c_double) -> c_double;

    pub fn LAPACKE_slapy3(x: c_float, y: c_float, z: c_float) -> c_float;
    pub fn LAPACKE_dlapy3(x: c_double, y: c_double, z: c_double) -> c_double;

    pub fn LAPACKE_slartgp(f: c_float, g: c_float, cs: *mut c_float, sn: *mut c_float, r: *mut c_float) -> c_int;
    pub fn LAPACKE_dlartgp(f: c_double, g: c_double, cs: *mut c_double, sn: *mut c_double, r: *mut c_double) -> c_int;

    pub fn LAPACKE_slartgs(x: c_float, y: c_float, sigma: c_float, cs: *mut c_float, sn: *mut c_float) -> c_int;
    pub fn LAPACKE_dlartgs(x: c_double, y: c_double, sigma: c_double, cs: *mut c_double, sn: *mut c_double) -> c_int;

    // LAPACK 3.3.0
    pub fn LAPACKE_cbbcsd(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, m: c_int, p: c_int, q: c_int, theta: *mut c_float, phi: *mut c_float, u1: *mut c_void, ldu1: c_int, u2: *mut c_void, ldu2: c_int, v1t: *mut c_void, ldv1t: c_int, v2t: *mut c_void, ldv2t: c_int, b11d: *mut c_float, b11e: *mut c_float, b12d: *mut c_float, b12e: *mut c_float, b21d: *mut c_float, b21e: *mut c_float, b22d: *mut c_float, b22e: *mut c_float) -> c_int;
    pub fn LAPACKE_cheswapr(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_chetri2(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_chetri2x(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, nb: c_int) -> c_int;
    pub fn LAPACKE_chetrs2(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_csyconv(matrix_order: c_int, uplo: c_char, way: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_csyswapr(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_csytri2(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_csytri2x(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, nb: c_int) -> c_int;
    pub fn LAPACKE_csytrs2(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_cunbdb(matrix_order: c_int, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_void, ldx11: c_int, x12: *mut c_void, ldx12: c_int, x21: *mut c_void, ldx21: c_int, x22: *mut c_void, ldx22: c_int, theta: *mut c_float, phi: *mut c_float, taup1: *mut c_void, taup2: *mut c_void, tauq1: *mut c_void, tauq2: *mut c_void) -> c_int;
    pub fn LAPACKE_cuncsd(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_void, ldx11: c_int, x12: *mut c_void, ldx12: c_int, x21: *mut c_void, ldx21: c_int, x22: *mut c_void, ldx22: c_int, theta: *mut c_float, u1: *mut c_void, ldu1: c_int, u2: *mut c_void, ldu2: c_int, v1t: *mut c_void, ldv1t: c_int, v2t: *mut c_void, ldv2t: c_int) -> c_int;
    pub fn LAPACKE_dbbcsd(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, m: c_int, p: c_int, q: c_int, theta: *mut c_double, phi: *mut c_double, u1: *mut c_double, ldu1: c_int, u2: *mut c_double, ldu2: c_int, v1t: *mut c_double, ldv1t: c_int, v2t: *mut c_double, ldv2t: c_int, b11d: *mut c_double, b11e: *mut c_double, b12d: *mut c_double, b12e: *mut c_double, b21d: *mut c_double, b21e: *mut c_double, b22d: *mut c_double, b22e: *mut c_double) -> c_int;
    pub fn LAPACKE_dorbdb(matrix_order: c_int, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_double, ldx11: c_int, x12: *mut c_double, ldx12: c_int, x21: *mut c_double, ldx21: c_int, x22: *mut c_double, ldx22: c_int, theta: *mut c_double, phi: *mut c_double, taup1: *mut c_double, taup2: *mut c_double, tauq1: *mut c_double, tauq2: *mut c_double) -> c_int;
    pub fn LAPACKE_dorcsd(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_double, ldx11: c_int, x12: *mut c_double, ldx12: c_int, x21: *mut c_double, ldx21: c_int, x22: *mut c_double, ldx22: c_int, theta: *mut c_double, u1: *mut c_double, ldu1: c_int, u2: *mut c_double, ldu2: c_int, v1t: *mut c_double, ldv1t: c_int, v2t: *mut c_double, ldv2t: c_int) -> c_int;
    pub fn LAPACKE_dsyconv(matrix_order: c_int, uplo: c_char, way: c_char, n: c_int, a: *mut c_double, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_dsyswapr(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_dsytri2(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_dsytri2x(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, ipiv: *const c_int, nb: c_int) -> c_int;
    pub fn LAPACKE_dsytrs2(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, ipiv: *const c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_sbbcsd(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, m: c_int, p: c_int, q: c_int, theta: *mut c_float, phi: *mut c_float, u1: *mut c_float, ldu1: c_int, u2: *mut c_float, ldu2: c_int, v1t: *mut c_float, ldv1t: c_int, v2t: *mut c_float, ldv2t: c_int, b11d: *mut c_float, b11e: *mut c_float, b12d: *mut c_float, b12e: *mut c_float, b21d: *mut c_float, b21e: *mut c_float, b22d: *mut c_float, b22e: *mut c_float) -> c_int;
    pub fn LAPACKE_sorbdb(matrix_order: c_int, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_float, ldx11: c_int, x12: *mut c_float, ldx12: c_int, x21: *mut c_float, ldx21: c_int, x22: *mut c_float, ldx22: c_int, theta: *mut c_float, phi: *mut c_float, taup1: *mut c_float, taup2: *mut c_float, tauq1: *mut c_float, tauq2: *mut c_float) -> c_int;
    pub fn LAPACKE_sorcsd(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_float, ldx11: c_int, x12: *mut c_float, ldx12: c_int, x21: *mut c_float, ldx21: c_int, x22: *mut c_float, ldx22: c_int, theta: *mut c_float, u1: *mut c_float, ldu1: c_int, u2: *mut c_float, ldu2: c_int, v1t: *mut c_float, ldv1t: c_int, v2t: *mut c_float, ldv2t: c_int) -> c_int;
    pub fn LAPACKE_ssyconv(matrix_order: c_int, uplo: c_char, way: c_char, n: c_int, a: *mut c_float, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_ssyswapr(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_ssytri2(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_ssytri2x(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, ipiv: *const c_int, nb: c_int) -> c_int;
    pub fn LAPACKE_ssytrs2(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, ipiv: *const c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_zbbcsd(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, m: c_int, p: c_int, q: c_int, theta: *mut c_double, phi: *mut c_double, u1: *mut c_void, ldu1: c_int, u2: *mut c_void, ldu2: c_int, v1t: *mut c_void, ldv1t: c_int, v2t: *mut c_void, ldv2t: c_int, b11d: *mut c_double, b11e: *mut c_double, b12d: *mut c_double, b12e: *mut c_double, b21d: *mut c_double, b21e: *mut c_double, b22d: *mut c_double, b22e: *mut c_double) -> c_int;
    pub fn LAPACKE_zheswapr(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_zhetri2(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_zhetri2x(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, nb: c_int) -> c_int;
    pub fn LAPACKE_zhetrs2(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zsyconv(matrix_order: c_int, uplo: c_char, way: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_zsyswapr(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_zsytri2(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int) -> c_int;
    pub fn LAPACKE_zsytri2x(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, nb: c_int) -> c_int;
    pub fn LAPACKE_zsytrs2(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zunbdb(matrix_order: c_int, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_void, ldx11: c_int, x12: *mut c_void, ldx12: c_int, x21: *mut c_void, ldx21: c_int, x22: *mut c_void, ldx22: c_int, theta: *mut c_double, phi: *mut c_double, taup1: *mut c_void, taup2: *mut c_void, tauq1: *mut c_void, tauq2: *mut c_void) -> c_int;
    pub fn LAPACKE_zuncsd(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_void, ldx11: c_int, x12: *mut c_void, ldx12: c_int, x21: *mut c_void, ldx21: c_int, x22: *mut c_void, ldx22: c_int, theta: *mut c_double, u1: *mut c_void, ldu1: c_int, u2: *mut c_void, ldu2: c_int, v1t: *mut c_void, ldv1t: c_int, v2t: *mut c_void, ldv2t: c_int) -> c_int;

    // LAPACK 3.4.0
    pub fn LAPACKE_sgemqrt(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, nb: c_int, v: *const c_float, ldv: c_int, t: *const c_float, ldt: c_int, c: *mut c_float, ldc: c_int) -> c_int;
    pub fn LAPACKE_dgemqrt(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, nb: c_int, v: *const c_double, ldv: c_int, t: *const c_double, ldt: c_int, c: *mut c_double, ldc: c_int) -> c_int;
    pub fn LAPACKE_cgemqrt(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, nb: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, c: *mut c_void, ldc: c_int) -> c_int;
    pub fn LAPACKE_zgemqrt(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, nb: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, c: *mut c_void, ldc: c_int) -> c_int;

    pub fn LAPACKE_sgeqrt(matrix_order: c_int, m: c_int, n: c_int, nb: c_int, a: *mut c_float, lda: c_int, t: *mut c_float, ldt: c_int) -> c_int;
    pub fn LAPACKE_dgeqrt(matrix_order: c_int, m: c_int, n: c_int, nb: c_int, a: *mut c_double, lda: c_int, t: *mut c_double, ldt: c_int) -> c_int;
    pub fn LAPACKE_cgeqrt(matrix_order: c_int, m: c_int, n: c_int, nb: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int) -> c_int;
    pub fn LAPACKE_zgeqrt(matrix_order: c_int, m: c_int, n: c_int, nb: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int) -> c_int;

    pub fn LAPACKE_sgeqrt2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, t: *mut c_float, ldt: c_int) -> c_int;
    pub fn LAPACKE_dgeqrt2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, t: *mut c_double, ldt: c_int) -> c_int;
    pub fn LAPACKE_cgeqrt2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int) -> c_int;
    pub fn LAPACKE_zgeqrt2(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int) -> c_int;

    pub fn LAPACKE_sgeqrt3(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, t: *mut c_float, ldt: c_int) -> c_int;
    pub fn LAPACKE_dgeqrt3(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, t: *mut c_double, ldt: c_int) -> c_int;
    pub fn LAPACKE_cgeqrt3(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int) -> c_int;
    pub fn LAPACKE_zgeqrt3(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int) -> c_int;

    pub fn LAPACKE_stpmqrt(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, nb: c_int, v: *const c_float, ldv: c_int, t: *const c_float, ldt: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dtpmqrt(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, nb: c_int, v: *const c_double, ldv: c_int, t: *const c_double, ldt: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_ctpmqrt(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, nb: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_ztpmqrt(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, nb: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_stpqrt(matrix_order: c_int, m: c_int, n: c_int, l: c_int, nb: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, t: *mut c_float, ldt: c_int) -> c_int;
    pub fn LAPACKE_dtpqrt(matrix_order: c_int, m: c_int, n: c_int, l: c_int, nb: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, t: *mut c_double, ldt: c_int) -> c_int;
    pub fn LAPACKE_ctpqrt(matrix_order: c_int, m: c_int, n: c_int, l: c_int, nb: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, t: *mut c_void, ldt: c_int) -> c_int;
    pub fn LAPACKE_ztpqrt(matrix_order: c_int, m: c_int, n: c_int, l: c_int, nb: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, t: *mut c_void, ldt: c_int) -> c_int;

    pub fn LAPACKE_stpqrt2(matrix_order: c_int, m: c_int, n: c_int, l: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, t: *mut c_float, ldt: c_int) -> c_int;
    pub fn LAPACKE_dtpqrt2(matrix_order: c_int, m: c_int, n: c_int, l: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, t: *mut c_double, ldt: c_int) -> c_int;
    pub fn LAPACKE_ctpqrt2(matrix_order: c_int, m: c_int, n: c_int, l: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, t: *mut c_void, ldt: c_int) -> c_int;
    pub fn LAPACKE_ztpqrt2(matrix_order: c_int, m: c_int, n: c_int, l: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, t: *mut c_void, ldt: c_int) -> c_int;

    pub fn LAPACKE_stprfb(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, l: c_int, v: *const c_float, ldv: c_int, t: *const c_float, ldt: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dtprfb(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, l: c_int, v: *const c_double, ldv: c_int, t: *const c_double, ldt: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_ctprfb(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, l: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_ztprfb(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, l: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    // LAPACK 3.X.X
    pub fn LAPACKE_ssysv_rook(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dsysv_rook(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_csysv_rook(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zsysv_rook(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_csyr(matrix_order: c_int, uplo: c_char, n: c_int, alpha: c_void, x: *const c_void, incx: c_int, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_zsyr(matrix_order: c_int, uplo: c_char, n: c_int, alpha: c_void, x: *const c_void, incx: c_int, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_ilaver(vers_major: *const c_int, vers_minor: *const c_int, vers_patch: *const c_int);
}

// Middle-level C interface
extern "C" {
    pub fn LAPACKE_sbdsdc_work(matrix_order: c_int, uplo: c_char, compq: c_char, n: c_int, d: *mut c_float, e: *mut c_float, u: *mut c_float, ldu: c_int, vt: *mut c_float, ldvt: c_int, q: *mut c_float, iq: *mut c_int, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dbdsdc_work(matrix_order: c_int, uplo: c_char, compq: c_char, n: c_int, d: *mut c_double, e: *mut c_double, u: *mut c_double, ldu: c_int, vt: *mut c_double, ldvt: c_int, q: *mut c_double, iq: *mut c_int, work: *mut c_double, iwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sbdsqr_work(matrix_order: c_int, uplo: c_char, n: c_int, ncvt: c_int, nru: c_int, ncc: c_int, d: *mut c_float, e: *mut c_float, vt: *mut c_float, ldvt: c_int, u: *mut c_float, ldu: c_int, c: *mut c_float, ldc: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dbdsqr_work(matrix_order: c_int, uplo: c_char, n: c_int, ncvt: c_int, nru: c_int, ncc: c_int, d: *mut c_double, e: *mut c_double, vt: *mut c_double, ldvt: c_int, u: *mut c_double, ldu: c_int, c: *mut c_double, ldc: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cbdsqr_work(matrix_order: c_int, uplo: c_char, n: c_int, ncvt: c_int, nru: c_int, ncc: c_int, d: *mut c_float, e: *mut c_float, vt: *mut c_void, ldvt: c_int, u: *mut c_void, ldu: c_int, c: *mut c_void, ldc: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_zbdsqr_work(matrix_order: c_int, uplo: c_char, n: c_int, ncvt: c_int, nru: c_int, ncc: c_int, d: *mut c_double, e: *mut c_double, vt: *mut c_void, ldvt: c_int, u: *mut c_void, ldu: c_int, c: *mut c_void, ldc: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_sdisna_work(job: c_char, m: c_int, n: c_int, d: *const c_float, sep: *mut c_float) -> c_int;
    pub fn LAPACKE_ddisna_work(job: c_char, m: c_int, n: c_int, d: *const c_double, sep: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbbrd_work(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, ncc: c_int, kl: c_int, ku: c_int, ab: *mut c_float, ldab: c_int, d: *mut c_float, e: *mut c_float, q: *mut c_float, ldq: c_int, pt: *mut c_float, ldpt: c_int, c: *mut c_float, ldc: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dgbbrd_work(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, ncc: c_int, kl: c_int, ku: c_int, ab: *mut c_double, ldab: c_int, d: *mut c_double, e: *mut c_double, q: *mut c_double, ldq: c_int, pt: *mut c_double, ldpt: c_int, c: *mut c_double, ldc: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cgbbrd_work(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, ncc: c_int, kl: c_int, ku: c_int, ab: *mut c_void, ldab: c_int, d: *mut c_float, e: *mut c_float, q: *mut c_void, ldq: c_int, pt: *mut c_void, ldpt: c_int, c: *mut c_void, ldc: c_int, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbbrd_work(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, ncc: c_int, kl: c_int, ku: c_int, ab: *mut c_void, ldab: c_int, d: *mut c_double, e: *mut c_double, q: *mut c_void, ldq: c_int, pt: *mut c_void, ldpt: c_int, c: *mut c_void, ldc: c_int, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbcon_work(matrix_order: c_int, norm: c_char, n: c_int, kl: c_int, ku: c_int, ab: *const c_float, ldab: c_int, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgbcon_work(matrix_order: c_int, norm: c_char, n: c_int, kl: c_int, ku: c_int, ab: *const c_double, ldab: c_int, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgbcon_work(matrix_order: c_int, norm: c_char, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbcon_work(matrix_order: c_int, norm: c_char, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbequ_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_float, ldab: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dgbequ_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_double, ldab: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cgbequ_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbequ_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbequb_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_float, ldab: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dgbequb_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_double, ldab: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cgbequb_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbequb_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *const c_void, ldab: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbrfs_work(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, afb: *const c_float, ldafb: c_int, ipiv: *const c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgbrfs_work(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, afb: *const c_double, ldafb: c_int, ipiv: *const c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgbrfs_work(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbrfs_work(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbrfsx_work(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, afb: *const c_float, ldafb: c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgbrfsx_work(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, afb: *const c_double, ldafb: c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgbrfsx_work(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbrfsx_work(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbsv_work(matrix_order: c_int, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_float, ldab: c_int, ipiv: *mut c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgbsv_work(matrix_order: c_int, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_double, ldab: c_int, ipiv: *mut c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgbsv_work(matrix_order: c_int, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgbsv_work(matrix_order: c_int, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sgbsvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_float, ldab: c_int, afb: *mut c_float, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgbsvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_double, ldab: c_int, afb: *mut c_double, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgbsvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbsvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbsvxx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_float, ldab: c_int, afb: *mut c_float, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgbsvxx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_double, ldab: c_int, afb: *mut c_double, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgbsvxx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgbsvxx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgbtrf_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *mut c_float, ldab: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_dgbtrf_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *mut c_double, ldab: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_cgbtrf_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *mut c_void, ldab: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zgbtrf_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, ab: *mut c_void, ldab: c_int, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_sgbtrs_work(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, ipiv: *const c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgbtrs_work(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, ipiv: *const c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgbtrs_work(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgbtrs_work(matrix_order: c_int, trans: c_char, n: c_int, kl: c_int, ku: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sgebak_work(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, scale: *const c_float, m: c_int, v: *mut c_float, ldv: c_int) -> c_int;
    pub fn LAPACKE_dgebak_work(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, scale: *const c_double, m: c_int, v: *mut c_double, ldv: c_int) -> c_int;
    pub fn LAPACKE_cgebak_work(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, scale: *const c_float, m: c_int, v: *mut c_void, ldv: c_int) -> c_int;
    pub fn LAPACKE_zgebak_work(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, scale: *const c_double, m: c_int, v: *mut c_void, ldv: c_int) -> c_int;

    pub fn LAPACKE_sgebal_work(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_float, lda: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float) -> c_int;
    pub fn LAPACKE_dgebal_work(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_double, lda: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double) -> c_int;
    pub fn LAPACKE_cgebal_work(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_void, lda: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float) -> c_int;
    pub fn LAPACKE_zgebal_work(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_void, lda: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double) -> c_int;

    pub fn LAPACKE_sgebrd_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, d: *mut c_float, e: *mut c_float, tauq: *mut c_float, taup: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgebrd_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, d: *mut c_double, e: *mut c_double, tauq: *mut c_double, taup: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgebrd_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, d: *mut c_float, e: *mut c_float, tauq: *mut c_void, taup: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zgebrd_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, d: *mut c_double, e: *mut c_double, tauq: *mut c_void, taup: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sgecon_work(matrix_order: c_int, norm: c_char, n: c_int, a: *const c_float, lda: c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgecon_work(matrix_order: c_int, norm: c_char, n: c_int, a: *const c_double, lda: c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgecon_work(matrix_order: c_int, norm: c_char, n: c_int, a: *const c_void, lda: c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgecon_work(matrix_order: c_int, norm: c_char, n: c_int, a: *const c_void, lda: c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgeequ_work(matrix_order: c_int, m: c_int, n: c_int, a: *const c_float, lda: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeequ_work(matrix_order: c_int, m: c_int, n: c_int, a: *const c_double, lda: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeequ_work(matrix_order: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zgeequ_work(matrix_order: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_sgeequb_work(matrix_order: c_int, m: c_int, n: c_int, a: *const c_float, lda: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeequb_work(matrix_order: c_int, m: c_int, n: c_int, a: *const c_double, lda: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeequb_work(matrix_order: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, r: *mut c_float, c: *mut c_float, rowcnd: *mut c_float, colcnd: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zgeequb_work(matrix_order: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, r: *mut c_double, c: *mut c_double, rowcnd: *mut c_double, colcnd: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_sgees_work(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_S_SELECT2, n: c_int, a: *mut c_float, lda: c_int, sdim: *mut c_int, wr: *mut c_float, wi: *mut c_float, vs: *mut c_float, ldvs: c_int, work: *mut c_float, lwork: c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgees_work(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_D_SELECT2, n: c_int, a: *mut c_double, lda: c_int, sdim: *mut c_int, wr: *mut c_double, wi: *mut c_double, vs: *mut c_double, ldvs: c_int, work: *mut c_double, lwork: c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgees_work(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_C_SELECT1, n: c_int, a: *mut c_void, lda: c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_zgees_work(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_Z_SELECT1, n: c_int, a: *mut c_void, lda: c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, bwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sgeesx_work(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_S_SELECT2, sense: c_char, n: c_int, a: *mut c_float, lda: c_int, sdim: *mut c_int, wr: *mut c_float, wi: *mut c_float, vs: *mut c_float, ldvs: c_int, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgeesx_work(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_D_SELECT2, sense: c_char, n: c_int, a: *mut c_double, lda: c_int, sdim: *mut c_int, wr: *mut c_double, wi: *mut c_double, vs: *mut c_double, ldvs: c_int, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgeesx_work(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_C_SELECT1, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: c_int, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_void, lwork: c_int, rwork: *mut c_float, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_zgeesx_work(matrix_order: c_int, jobvs: c_char, sort: c_char, select: LAPACK_Z_SELECT1, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, sdim: *mut c_int, w: *mut c_void, vs: *mut c_void, ldvs: c_int, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_void, lwork: c_int, rwork: *mut c_double, bwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sgeev_work(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_float, lda: c_int, wr: *mut c_float, wi: *mut c_float, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgeev_work(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_double, lda: c_int, wr: *mut c_double, wi: *mut c_double, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgeev_work(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgeev_work(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgeevx_work(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_float, lda: c_int, wr: *mut c_float, wi: *mut c_float, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float, abnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgeevx_work(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_double, lda: c_int, wr: *mut c_double, wi: *mut c_double, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double, abnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgeevx_work(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_float, abnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgeevx_work(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, scale: *mut c_double, abnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgehrd_work(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgehrd_work(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgehrd_work(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zgehrd_work(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sgejsv_work(matrix_order: c_int, joba: c_char, jobu: c_char, jobv: c_char, jobr: c_char, jobt: c_char, jobp: c_char, m: c_int, n: c_int, a: *mut c_float, lda: c_int, sva: *mut c_float, u: *mut c_float, ldu: c_int, v: *mut c_float, ldv: c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgejsv_work(matrix_order: c_int, joba: c_char, jobu: c_char, jobv: c_char, jobr: c_char, jobt: c_char, jobp: c_char, m: c_int, n: c_int, a: *mut c_double, lda: c_int, sva: *mut c_double, u: *mut c_double, ldu: c_int, v: *mut c_double, ldv: c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sgelq2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dgelq2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cgelq2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zgelq2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void) -> c_int;

    pub fn LAPACKE_sgelqf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgelqf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgelqf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zgelqf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sgels_work(matrix_order: c_int, trans: c_char, m: c_int, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgels_work(matrix_order: c_int, trans: c_char, m: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgels_work(matrix_order: c_int, trans: c_char, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zgels_work(matrix_order: c_int, trans: c_char, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sgelsd_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, s: *mut c_float, rcond: c_float, rank: *mut c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgelsd_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, s: *mut c_double, rcond: c_double, rank: *mut c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgelsd_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, s: *mut c_float, rcond: c_float, rank: *mut c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_zgelsd_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, s: *mut c_double, rcond: c_double, rank: *mut c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, iwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sgelss_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, s: *mut c_float, rcond: c_float, rank: *mut c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgelss_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, s: *mut c_double, rcond: c_double, rank: *mut c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgelss_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, s: *mut c_float, rcond: c_float, rank: *mut c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgelss_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, s: *mut c_double, rcond: c_double, rank: *mut c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgelsy_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, jpvt: *mut c_int, rcond: c_float, rank: *mut c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgelsy_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, jpvt: *mut c_int, rcond: c_double, rank: *mut c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgelsy_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, jpvt: *mut c_int, rcond: c_float, rank: *mut c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgelsy_work(matrix_order: c_int, m: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, jpvt: *mut c_int, rcond: c_double, rank: *mut c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgeqlf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgeqlf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgeqlf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zgeqlf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sgeqp3_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, jpvt: *mut c_int, tau: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgeqp3_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, jpvt: *mut c_int, tau: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgeqp3_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, jpvt: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgeqp3_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, jpvt: *mut c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgeqpf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, jpvt: *mut c_int, tau: *mut c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeqpf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, jpvt: *mut c_int, tau: *mut c_double, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeqpf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, jpvt: *mut c_int, tau: *mut c_void, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgeqpf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, jpvt: *mut c_int, tau: *mut c_void, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgeqr2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeqr2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeqr2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zgeqr2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void) -> c_int;

    pub fn LAPACKE_sgeqrf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgeqrf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgeqrf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zgeqrf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sgeqrfp_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgeqrfp_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgeqrfp_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zgeqrfp_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sgerfs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, ipiv: *const c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgerfs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, ipiv: *const c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgerfs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgerfs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgerfsx_work(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgerfsx_work(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgerfsx_work(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, r: *const c_float, c: *const c_float, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgerfsx_work(matrix_order: c_int, trans: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, r: *const c_double, c: *const c_double, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgerqf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgerqf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgerqf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zgerqf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sgesdd_work(matrix_order: c_int, jobz: c_char, m: c_int, n: c_int, a: *mut c_float, lda: c_int, s: *mut c_float, u: *mut c_float, ldu: c_int, vt: *mut c_float, ldvt: c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgesdd_work(matrix_order: c_int, jobz: c_char, m: c_int, n: c_int, a: *mut c_double, lda: c_int, s: *mut c_double, u: *mut c_double, ldu: c_int, vt: *mut c_double, ldvt: c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgesdd_work(matrix_order: c_int, jobz: c_char, m: c_int, n: c_int, a: *mut c_void, lda: c_int, s: *mut c_float, u: *mut c_void, ldu: c_int, vt: *mut c_void, ldvt: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_zgesdd_work(matrix_order: c_int, jobz: c_char, m: c_int, n: c_int, a: *mut c_void, lda: c_int, s: *mut c_double, u: *mut c_void, ldu: c_int, vt: *mut c_void, ldvt: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, iwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sgesv_work(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgesv_work(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgesv_work(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgesv_work(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_dsgesv_work(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, work: *mut c_double, swork: *mut c_float, iter: *mut c_int) -> c_int;
    pub fn LAPACKE_zcgesv_work(matrix_order: c_int, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, work: *mut c_void, swork: *mut c_void, rwork: *mut c_double, iter: *mut c_int) -> c_int;

    pub fn LAPACKE_sgesvd_work(matrix_order: c_int, jobu: c_char, jobvt: c_char, m: c_int, n: c_int, a: *mut c_float, lda: c_int, s: *mut c_float, u: *mut c_float, ldu: c_int, vt: *mut c_float, ldvt: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgesvd_work(matrix_order: c_int, jobu: c_char, jobvt: c_char, m: c_int, n: c_int, a: *mut c_double, lda: c_int, s: *mut c_double, u: *mut c_double, ldu: c_int, vt: *mut c_double, ldvt: c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgesvd_work(matrix_order: c_int, jobu: c_char, jobvt: c_char, m: c_int, n: c_int, a: *mut c_void, lda: c_int, s: *mut c_float, u: *mut c_void, ldu: c_int, vt: *mut c_void, ldvt: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgesvd_work(matrix_order: c_int, jobu: c_char, jobvt: c_char, m: c_int, n: c_int, a: *mut c_void, lda: c_int, s: *mut c_double, u: *mut c_void, ldu: c_int, vt: *mut c_void, ldvt: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgesvj_work(matrix_order: c_int, joba: c_char, jobu: c_char, jobv: c_char, m: c_int, n: c_int, a: *mut c_float, lda: c_int, sva: *mut c_float, mv: c_int, v: *mut c_float, ldv: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgesvj_work(matrix_order: c_int, joba: c_char, jobu: c_char, jobv: c_char, m: c_int, n: c_int, a: *mut c_double, lda: c_int, sva: *mut c_double, mv: c_int, v: *mut c_double, ldv: c_int, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sgesvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, af: *mut c_float, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgesvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, af: *mut c_double, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgesvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgesvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgesvxx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, af: *mut c_float, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgesvxx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, af: *mut c_double, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgesvxx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_float, c: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgesvxx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, r: *mut c_double, c: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgetf2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_dgetf2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_cgetf2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zgetf2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_sgetrf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_dgetrf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_cgetrf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zgetrf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_sgetri_work(matrix_order: c_int, n: c_int, a: *mut c_float, lda: c_int, ipiv: *const c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgetri_work(matrix_order: c_int, n: c_int, a: *mut c_double, lda: c_int, ipiv: *const c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgetri_work(matrix_order: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zgetri_work(matrix_order: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sgetrs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, ipiv: *const c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgetrs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, ipiv: *const c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgetrs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgetrs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sggbak_work(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, lscale: *const c_float, rscale: *const c_float, m: c_int, v: *mut c_float, ldv: c_int) -> c_int;
    pub fn LAPACKE_dggbak_work(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, lscale: *const c_double, rscale: *const c_double, m: c_int, v: *mut c_double, ldv: c_int) -> c_int;
    pub fn LAPACKE_cggbak_work(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, lscale: *const c_float, rscale: *const c_float, m: c_int, v: *mut c_void, ldv: c_int) -> c_int;
    pub fn LAPACKE_zggbak_work(matrix_order: c_int, job: c_char, side: c_char, n: c_int, ilo: c_int, ihi: c_int, lscale: *const c_double, rscale: *const c_double, m: c_int, v: *mut c_void, ldv: c_int) -> c_int;

    pub fn LAPACKE_sggbal_work(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dggbal_work(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cggbal_work(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_zggbal_work(matrix_order: c_int, job: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double, work: *mut c_double) -> c_int;

    pub fn LAPACKE_sgges_work(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_S_SELECT3, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, sdim: *mut c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vsl: *mut c_float, ldvsl: c_int, vsr: *mut c_float, ldvsr: c_int, work: *mut c_float, lwork: c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgges_work(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_D_SELECT3, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, sdim: *mut c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vsl: *mut c_double, ldvsl: c_int, vsr: *mut c_double, ldvsr: c_int, work: *mut c_double, lwork: c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgges_work(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_C_SELECT2, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: c_int, vsr: *mut c_void, ldvsr: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_zgges_work(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_Z_SELECT2, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: c_int, vsr: *mut c_void, ldvsr: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, bwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sggesx_work(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_S_SELECT3, sense: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, sdim: *mut c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vsl: *mut c_float, ldvsl: c_int, vsr: *mut c_float, ldvsr: c_int, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dggesx_work(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_D_SELECT3, sense: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, sdim: *mut c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vsl: *mut c_double, ldvsl: c_int, vsr: *mut c_double, ldvsr: c_int, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cggesx_work(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_C_SELECT2, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: c_int, vsr: *mut c_void, ldvsr: c_int, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_void, lwork: c_int, rwork: *mut c_float, iwork: *mut c_int, liwork: c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_zggesx_work(matrix_order: c_int, jobvsl: c_char, jobvsr: c_char, sort: c_char, selctg: LAPACK_Z_SELECT2, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, sdim: *mut c_int, alpha: *mut c_void, beta: *mut c_void, vsl: *mut c_void, ldvsl: c_int, vsr: *mut c_void, ldvsr: c_int, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_void, lwork: c_int, rwork: *mut c_double, iwork: *mut c_int, liwork: c_int, bwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sggev_work(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dggev_work(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cggev_work(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zggev_work(matrix_order: c_int, jobvl: c_char, jobvr: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sggevx_work(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float, abnrm: *mut c_float, bbnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_float, lwork: c_int, iwork: *mut c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dggevx_work(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double, abnrm: *mut c_double, bbnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_double, lwork: c_int, iwork: *mut c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cggevx_work(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_float, rscale: *mut c_float, abnrm: *mut c_float, bbnrm: *mut c_float, rconde: *mut c_float, rcondv: *mut c_float, work: *mut c_void, lwork: c_int, rwork: *mut c_float, iwork: *mut c_int, bwork: *mut c_int) -> c_int;
    pub fn LAPACKE_zggevx_work(matrix_order: c_int, balanc: c_char, jobvl: c_char, jobvr: c_char, sense: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, ilo: *mut c_int, ihi: *mut c_int, lscale: *mut c_double, rscale: *mut c_double, abnrm: *mut c_double, bbnrm: *mut c_double, rconde: *mut c_double, rcondv: *mut c_double, work: *mut c_void, lwork: c_int, rwork: *mut c_double, iwork: *mut c_int, bwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sggglm_work(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, d: *mut c_float, x: *mut c_float, y: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dggglm_work(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, d: *mut c_double, x: *mut c_double, y: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cggglm_work(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, d: *mut c_void, x: *mut c_void, y: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zggglm_work(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, d: *mut c_void, x: *mut c_void, y: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sgghrd_work(matrix_order: c_int, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, q: *mut c_float, ldq: c_int, z: *mut c_float, ldz: c_int) -> c_int;
    pub fn LAPACKE_dgghrd_work(matrix_order: c_int, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, q: *mut c_double, ldq: c_int, z: *mut c_double, ldz: c_int) -> c_int;
    pub fn LAPACKE_cgghrd_work(matrix_order: c_int, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int) -> c_int;
    pub fn LAPACKE_zgghrd_work(matrix_order: c_int, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int) -> c_int;

    pub fn LAPACKE_sgglse_work(matrix_order: c_int, m: c_int, n: c_int, p: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, c: *mut c_float, d: *mut c_float, x: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dgglse_work(matrix_order: c_int, m: c_int, n: c_int, p: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, c: *mut c_double, d: *mut c_double, x: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cgglse_work(matrix_order: c_int, m: c_int, n: c_int, p: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, c: *mut c_void, d: *mut c_void, x: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zgglse_work(matrix_order: c_int, m: c_int, n: c_int, p: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, c: *mut c_void, d: *mut c_void, x: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sggqrf_work(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_float, lda: c_int, taua: *mut c_float, b: *mut c_float, ldb: c_int, taub: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dggqrf_work(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_double, lda: c_int, taua: *mut c_double, b: *mut c_double, ldb: c_int, taub: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cggqrf_work(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_void, lda: c_int, taua: *mut c_void, b: *mut c_void, ldb: c_int, taub: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zggqrf_work(matrix_order: c_int, n: c_int, m: c_int, p: c_int, a: *mut c_void, lda: c_int, taua: *mut c_void, b: *mut c_void, ldb: c_int, taub: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sggrqf_work(matrix_order: c_int, m: c_int, p: c_int, n: c_int, a: *mut c_float, lda: c_int, taua: *mut c_float, b: *mut c_float, ldb: c_int, taub: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dggrqf_work(matrix_order: c_int, m: c_int, p: c_int, n: c_int, a: *mut c_double, lda: c_int, taua: *mut c_double, b: *mut c_double, ldb: c_int, taub: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_cggrqf_work(matrix_order: c_int, m: c_int, p: c_int, n: c_int, a: *mut c_void, lda: c_int, taua: *mut c_void, b: *mut c_void, ldb: c_int, taub: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zggrqf_work(matrix_order: c_int, m: c_int, p: c_int, n: c_int, a: *mut c_void, lda: c_int, taua: *mut c_void, b: *mut c_void, ldb: c_int, taub: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_sggsvd_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, n: c_int, p: c_int, k: *mut c_int, l: *mut c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, alpha: *mut c_float, beta: *mut c_float, u: *mut c_float, ldu: c_int, v: *mut c_float, ldv: c_int, q: *mut c_float, ldq: c_int, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dggsvd_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, n: c_int, p: c_int, k: *mut c_int, l: *mut c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, alpha: *mut c_double, beta: *mut c_double, u: *mut c_double, ldu: c_int, v: *mut c_double, ldv: c_int, q: *mut c_double, ldq: c_int, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cggsvd_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, n: c_int, p: c_int, k: *mut c_int, l: *mut c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_float, beta: *mut c_float, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int, work: *mut c_void, rwork: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_zggsvd_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, n: c_int, p: c_int, k: *mut c_int, l: *mut c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_double, beta: *mut c_double, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int, work: *mut c_void, rwork: *mut c_double, iwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sggsvp_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, tola: c_float, tolb: c_float, k: *mut c_int, l: *mut c_int, u: *mut c_float, ldu: c_int, v: *mut c_float, ldv: c_int, q: *mut c_float, ldq: c_int, iwork: *mut c_int, tau: *mut c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dggsvp_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, tola: c_double, tolb: c_double, k: *mut c_int, l: *mut c_int, u: *mut c_double, ldu: c_int, v: *mut c_double, ldv: c_int, q: *mut c_double, ldq: c_int, iwork: *mut c_int, tau: *mut c_double, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cggsvp_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, tola: c_float, tolb: c_float, k: *mut c_int, l: *mut c_int, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int, iwork: *mut c_int, rwork: *mut c_float, tau: *mut c_void, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zggsvp_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, tola: c_double, tolb: c_double, k: *mut c_int, l: *mut c_int, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int, iwork: *mut c_int, rwork: *mut c_double, tau: *mut c_void, work: *mut c_void) -> c_int;

    pub fn LAPACKE_sgtcon_work(norm: c_char, n: c_int, dl: *const c_float, d: *const c_float, du: *const c_float, du2: *const c_float, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgtcon_work(norm: c_char, n: c_int, dl: *const c_double, d: *const c_double, du: *const c_double, du2: *const c_double, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgtcon_work(norm: c_char, n: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zgtcon_work(norm: c_char, n: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_void) -> c_int;

    pub fn LAPACKE_sgtrfs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_float, d: *const c_float, du: *const c_float, dlf: *const c_float, df: *const c_float, duf: *const c_float, du2: *const c_float, ipiv: *const c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgtrfs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_double, d: *const c_double, du: *const c_double, dlf: *const c_double, df: *const c_double, duf: *const c_double, du2: *const c_double, ipiv: *const c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgtrfs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *const c_void, df: *const c_void, duf: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgtrfs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *const c_void, df: *const c_void, duf: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgtsv_work(matrix_order: c_int, n: c_int, nrhs: c_int, dl: *mut c_float, d: *mut c_float, du: *mut c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgtsv_work(matrix_order: c_int, n: c_int, nrhs: c_int, dl: *mut c_double, d: *mut c_double, du: *mut c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgtsv_work(matrix_order: c_int, n: c_int, nrhs: c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgtsv_work(matrix_order: c_int, n: c_int, nrhs: c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sgtsvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_float, d: *const c_float, du: *const c_float, dlf: *mut c_float, df: *mut c_float, duf: *mut c_float, du2: *mut c_float, ipiv: *mut c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dgtsvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_double, d: *const c_double, du: *const c_double, dlf: *mut c_double, df: *mut c_double, duf: *mut c_double, du2: *mut c_double, ipiv: *mut c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cgtsvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *mut c_void, df: *mut c_void, duf: *mut c_void, du2: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zgtsvx_work(matrix_order: c_int, fact: c_char, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, dlf: *mut c_void, df: *mut c_void, duf: *mut c_void, du2: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sgttrf_work(n: c_int, dl: *mut c_float, d: *mut c_float, du: *mut c_float, du2: *mut c_float, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_dgttrf_work(n: c_int, dl: *mut c_double, d: *mut c_double, du: *mut c_double, du2: *mut c_double, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_cgttrf_work(n: c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, du2: *mut c_void, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zgttrf_work(n: c_int, dl: *mut c_void, d: *mut c_void, du: *mut c_void, du2: *mut c_void, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_sgttrs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_float, d: *const c_float, du: *const c_float, du2: *const c_float, ipiv: *const c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dgttrs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_double, d: *const c_double, du: *const c_double, du2: *const c_double, ipiv: *const c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cgttrs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zgttrs_work(matrix_order: c_int, trans: c_char, n: c_int, nrhs: c_int, dl: *const c_void, d: *const c_void, du: *const c_void, du2: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_chbev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zhbev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chbevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_zhbevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_chbevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, q: *mut c_void, ldq: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zhbevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, q: *mut c_void, ldq: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_chbgst_work(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *const c_void, ldbb: c_int, x: *mut c_void, ldx: c_int, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zhbgst_work(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *const c_void, ldbb: c_int, x: *mut c_void, ldx: c_int, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chbgv_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zhbgv_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chbgvd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_zhbgvd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_chbgvx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, q: *mut c_void, ldq: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zhbgvx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_void, ldab: c_int, bb: *mut c_void, ldbb: c_int, q: *mut c_void, ldq: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_chbtrd_work(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, d: *mut c_float, e: *mut c_float, q: *mut c_void, ldq: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zhbtrd_work(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int, d: *mut c_double, e: *mut c_double, q: *mut c_void, ldq: c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_checon_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zhecon_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_void) -> c_int;

    pub fn LAPACKE_cheequb_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zheequb_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, work: *mut c_void) -> c_int;

    pub fn LAPACKE_cheev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_float, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zheev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_double, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_cheevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_float, work: *mut c_void, lwork: c_int, rwork: *mut c_float, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_zheevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, w: *mut c_double, work: *mut c_void, lwork: c_int, rwork: *mut c_double, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_cheevr_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, isuppz: *mut c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_zheevr_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, isuppz: *mut c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_cheevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zheevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_chegst_work(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *const c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zhegst_work(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *const c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_chegv_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, w: *mut c_float, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zhegv_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, w: *mut c_double, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chegvd_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, w: *mut c_float, work: *mut c_void, lwork: c_int, rwork: *mut c_float, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_zhegvd_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, w: *mut c_double, work: *mut c_void, lwork: c_int, rwork: *mut c_double, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_chegvx_work(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zhegvx_work(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_cherfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zherfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_cherfsx_work(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, s: *const c_float, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zherfsx_work(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, s: *const c_double, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chesv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zhesv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_chesvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zhesvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chesvxx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zhesvxx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chetrd_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, d: *mut c_float, e: *mut c_float, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zhetrd_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, d: *mut c_double, e: *mut c_double, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_chetrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zhetrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_chetri_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zhetri_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_chetrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zhetrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_chfrk_work(matrix_order: c_int, transr: c_char, uplo: c_char, trans: c_char, n: c_int, k: c_int, alpha: c_float, a: *const c_void, lda: c_int, beta: c_float, c: *mut c_void) -> c_int;
    pub fn LAPACKE_zhfrk_work(matrix_order: c_int, transr: c_char, uplo: c_char, trans: c_char, n: c_int, k: c_int, alpha: c_double, a: *const c_void, lda: c_int, beta: c_double, c: *mut c_void) -> c_int;

    pub fn LAPACKE_shgeqz_work(matrix_order: c_int, job: c_char, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_float, ldh: c_int, t: *mut c_float, ldt: c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, q: *mut c_float, ldq: c_int, z: *mut c_float, ldz: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dhgeqz_work(matrix_order: c_int, job: c_char, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_double, ldh: c_int, t: *mut c_double, ldt: c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, q: *mut c_double, ldq: c_int, z: *mut c_double, ldz: c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_chgeqz_work(matrix_order: c_int, job: c_char, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_void, ldh: c_int, t: *mut c_void, ldt: c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zhgeqz_work(matrix_order: c_int, job: c_char, compq: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_void, ldh: c_int, t: *mut c_void, ldt: c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chpcon_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zhpcon_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_void) -> c_int;

    pub fn LAPACKE_chpev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zhpev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chpevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_zhpevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_chpevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_void, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zhpevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_void, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_chpgst_work(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, ap: *mut c_void, bp: *const c_void) -> c_int;
    pub fn LAPACKE_zhpgst_work(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, ap: *mut c_void, bp: *const c_void) -> c_int;

    pub fn LAPACKE_chpgv_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zhpgv_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chpgvd_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_zhpgvd_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_chpgvx_work(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_float, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_zhpgvx_work(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_void, bp: *mut c_void, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, rwork: *mut c_double, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_chprfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zhprfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chpsv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zhpsv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_chpsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zhpsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_chptrd_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, d: *mut c_float, e: *mut c_float, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zhptrd_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, d: *mut c_double, e: *mut c_double, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_chptrf_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zhptrf_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_chptri_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *const c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zhptri_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *const c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_chptrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zhptrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_shsein_work(matrix_order: c_int, job: c_char, eigsrc: c_char, initv: c_char, select: *mut c_int, n: c_int, h: *const c_float, ldh: c_int, wr: *mut c_float, wi: *const c_float, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_float, ifaill: *mut c_int, ifailr: *mut c_int) -> c_int;
    pub fn LAPACKE_dhsein_work(matrix_order: c_int, job: c_char, eigsrc: c_char, initv: c_char, select: *mut c_int, n: c_int, h: *const c_double, ldh: c_int, wr: *mut c_double, wi: *const c_double, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_double, ifaill: *mut c_int, ifailr: *mut c_int) -> c_int;
    pub fn LAPACKE_chsein_work(matrix_order: c_int, job: c_char, eigsrc: c_char, initv: c_char, select: *const c_int, n: c_int, h: *const c_void, ldh: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_float, ifaill: *mut c_int, ifailr: *mut c_int) -> c_int;
    pub fn LAPACKE_zhsein_work(matrix_order: c_int, job: c_char, eigsrc: c_char, initv: c_char, select: *const c_int, n: c_int, h: *const c_void, ldh: c_int, w: *mut c_void, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_double, ifaill: *mut c_int, ifailr: *mut c_int) -> c_int;

    pub fn LAPACKE_shseqr_work(matrix_order: c_int, job: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_float, ldh: c_int, wr: *mut c_float, wi: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dhseqr_work(matrix_order: c_int, job: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_double, ldh: c_int, wr: *mut c_double, wi: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_chseqr_work(matrix_order: c_int, job: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_void, ldh: c_int, w: *mut c_void, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zhseqr_work(matrix_order: c_int, job: c_char, compz: c_char, n: c_int, ilo: c_int, ihi: c_int, h: *mut c_void, ldh: c_int, w: *mut c_void, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_clacgv_work(n: c_int, x: *mut c_void, incx: c_int) -> c_int;
    pub fn LAPACKE_zlacgv_work(n: c_int, x: *mut c_void, incx: c_int) -> c_int;

    pub fn LAPACKE_slacn2_work(n: c_int, v: *mut c_float, x: *mut c_float, isgn: *mut c_int, est: *mut c_float, kase: *mut c_int, isave: *mut c_int) -> c_int;
    pub fn LAPACKE_dlacn2_work(n: c_int, v: *mut c_double, x: *mut c_double, isgn: *mut c_int, est: *mut c_double, kase: *mut c_int, isave: *mut c_int) -> c_int;
    pub fn LAPACKE_clacn2_work(n: c_int, v: *mut c_void, x: *mut c_void, est: *mut c_float, kase: *mut c_int, isave: *mut c_int) -> c_int;
    pub fn LAPACKE_zlacn2_work(n: c_int, v: *mut c_void, x: *mut c_void, est: *mut c_double, kase: *mut c_int, isave: *mut c_int) -> c_int;

    pub fn LAPACKE_slacpy_work(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_float, lda: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dlacpy_work(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_double, lda: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_clacpy_work(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zlacpy_work(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_clacp2_work(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_float, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zlacp2_work(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, a: *const c_double, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_zlag2c_work(matrix_order: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, sa: *mut c_void, ldsa: c_int) -> c_int;

    pub fn LAPACKE_slag2d_work(matrix_order: c_int, m: c_int, n: c_int, sa: *const c_float, ldsa: c_int, a: *mut c_double, lda: c_int) -> c_int;

    pub fn LAPACKE_dlag2s_work(matrix_order: c_int, m: c_int, n: c_int, a: *const c_double, lda: c_int, sa: *mut c_float, ldsa: c_int) -> c_int;

    pub fn LAPACKE_clag2z_work(matrix_order: c_int, m: c_int, n: c_int, sa: *const c_void, ldsa: c_int, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_slagge_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, d: *const c_float, a: *mut c_float, lda: c_int, iseed: *mut c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dlagge_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, d: *const c_double, a: *mut c_double, lda: c_int, iseed: *mut c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_clagge_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, d: *const c_float, a: *mut c_void, lda: c_int, iseed: *mut c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zlagge_work(matrix_order: c_int, m: c_int, n: c_int, kl: c_int, ku: c_int, d: *const c_double, a: *mut c_void, lda: c_int, iseed: *mut c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_claghe_work(matrix_order: c_int, n: c_int, k: c_int, d: *const c_float, a: *mut c_void, lda: c_int, iseed: *mut c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zlaghe_work(matrix_order: c_int, n: c_int, k: c_int, d: *const c_double, a: *mut c_void, lda: c_int, iseed: *mut c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_slagsy_work(matrix_order: c_int, n: c_int, k: c_int, d: *const c_float, a: *mut c_float, lda: c_int, iseed: *mut c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dlagsy_work(matrix_order: c_int, n: c_int, k: c_int, d: *const c_double, a: *mut c_double, lda: c_int, iseed: *mut c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_clagsy_work(matrix_order: c_int, n: c_int, k: c_int, d: *const c_float, a: *mut c_void, lda: c_int, iseed: *mut c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zlagsy_work(matrix_order: c_int, n: c_int, k: c_int, d: *const c_double, a: *mut c_void, lda: c_int, iseed: *mut c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_slapmr_work(matrix_order: c_int, forwrd: c_int, m: c_int, n: c_int, x: *mut c_float, ldx: c_int, k: *mut c_int) -> c_int;
    pub fn LAPACKE_dlapmr_work(matrix_order: c_int, forwrd: c_int, m: c_int, n: c_int, x: *mut c_double, ldx: c_int, k: *mut c_int) -> c_int;
    pub fn LAPACKE_clapmr_work(matrix_order: c_int, forwrd: c_int, m: c_int, n: c_int, x: *mut c_void, ldx: c_int, k: *mut c_int) -> c_int;
    pub fn LAPACKE_zlapmr_work(matrix_order: c_int, forwrd: c_int, m: c_int, n: c_int, x: *mut c_void, ldx: c_int, k: *mut c_int) -> c_int;

    pub fn LAPACKE_slartgp_work(f: c_float, g: c_float, cs: *mut c_float, sn: *mut c_float, r: *mut c_float) -> c_int;
    pub fn LAPACKE_dlartgp_work(f: c_double, g: c_double, cs: *mut c_double, sn: *mut c_double, r: *mut c_double) -> c_int;

    pub fn LAPACKE_slartgs_work(x: c_float, y: c_float, sigma: c_float, cs: *mut c_float, sn: *mut c_float) -> c_int;
    pub fn LAPACKE_dlartgs_work(x: c_double, y: c_double, sigma: c_double, cs: *mut c_double, sn: *mut c_double) -> c_int;

    pub fn LAPACKE_slapy2_work(x: c_float, y: c_float) -> c_float;
    pub fn LAPACKE_dlapy2_work(x: c_double, y: c_double) -> c_double;

    pub fn LAPACKE_slapy3_work(x: c_float, y: c_float, z: c_float) -> c_float;
    pub fn LAPACKE_dlapy3_work(x: c_double, y: c_double, z: c_double) -> c_double;

    pub fn LAPACKE_slamch_work(cmach: c_char) -> c_float;
    pub fn LAPACKE_dlamch_work(cmach: c_char) -> c_double;

    pub fn LAPACKE_slange_work(matrix_order: c_int, norm: c_char, m: c_int, n: c_int, a: *const c_float, lda: c_int, work: *mut c_float) -> c_float;
    pub fn LAPACKE_dlange_work(matrix_order: c_int, norm: c_char, m: c_int, n: c_int, a: *const c_double, lda: c_int, work: *mut c_double) -> c_double;
    pub fn LAPACKE_clange_work(matrix_order: c_int, norm: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, work: *mut c_float) -> c_float;
    pub fn LAPACKE_zlange_work(matrix_order: c_int, norm: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, work: *mut c_double) -> c_double;

    pub fn LAPACKE_clanhe_work(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, work: *mut c_float) -> c_float;
    pub fn LAPACKE_zlanhe_work(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, work: *mut c_double) -> c_double;

    pub fn LAPACKE_slansy_work(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_float, lda: c_int, work: *mut c_float) -> c_float;
    pub fn LAPACKE_dlansy_work(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_double, lda: c_int, work: *mut c_double) -> c_double;
    pub fn LAPACKE_clansy_work(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, work: *mut c_float) -> c_float;
    pub fn LAPACKE_zlansy_work(matrix_order: c_int, norm: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, work: *mut c_double) -> c_double;

    pub fn LAPACKE_slantr_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, m: c_int, n: c_int, a: *const c_float, lda: c_int, work: *mut c_float) -> c_float;
    pub fn LAPACKE_dlantr_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, m: c_int, n: c_int, a: *const c_double, lda: c_int, work: *mut c_double) -> c_double;
    pub fn LAPACKE_clantr_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, work: *mut c_float) -> c_float;
    pub fn LAPACKE_zlantr_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, work: *mut c_double) -> c_double;

    pub fn LAPACKE_slarfb_work(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, v: *const c_float, ldv: c_int, t: *const c_float, ldt: c_int, c: *mut c_float, ldc: c_int, work: *mut c_float, ldwork: c_int) -> c_int;
    pub fn LAPACKE_dlarfb_work(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, v: *const c_double, ldv: c_int, t: *const c_double, ldt: c_int, c: *mut c_double, ldc: c_int, work: *mut c_double, ldwork: c_int) -> c_int;
    pub fn LAPACKE_clarfb_work(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, c: *mut c_void, ldc: c_int, work: *mut c_void, ldwork: c_int) -> c_int;
    pub fn LAPACKE_zlarfb_work(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, c: *mut c_void, ldc: c_int, work: *mut c_void, ldwork: c_int) -> c_int;

    pub fn LAPACKE_slarfg_work(n: c_int, alpha: *mut c_float, x: *mut c_float, incx: c_int, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dlarfg_work(n: c_int, alpha: *mut c_double, x: *mut c_double, incx: c_int, tau: *mut c_double) -> c_int;
    pub fn LAPACKE_clarfg_work(n: c_int, alpha: *mut c_void, x: *mut c_void, incx: c_int, tau: *mut c_void) -> c_int;
    pub fn LAPACKE_zlarfg_work(n: c_int, alpha: *mut c_void, x: *mut c_void, incx: c_int, tau: *mut c_void) -> c_int;

    pub fn LAPACKE_slarft_work(matrix_order: c_int, direct: c_char, storev: c_char, n: c_int, k: c_int, v: *const c_float, ldv: c_int, tau: *const c_float, t: *mut c_float, ldt: c_int) -> c_int;
    pub fn LAPACKE_dlarft_work(matrix_order: c_int, direct: c_char, storev: c_char, n: c_int, k: c_int, v: *const c_double, ldv: c_int, tau: *const c_double, t: *mut c_double, ldt: c_int) -> c_int;
    pub fn LAPACKE_clarft_work(matrix_order: c_int, direct: c_char, storev: c_char, n: c_int, k: c_int, v: *const c_void, ldv: c_int, tau: *const c_void, t: *mut c_void, ldt: c_int) -> c_int;
    pub fn LAPACKE_zlarft_work(matrix_order: c_int, direct: c_char, storev: c_char, n: c_int, k: c_int, v: *const c_void, ldv: c_int, tau: *const c_void, t: *mut c_void, ldt: c_int) -> c_int;

    pub fn LAPACKE_slarfx_work(matrix_order: c_int, side: c_char, m: c_int, n: c_int, v: *const c_float, tau: c_float, c: *mut c_float, ldc: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dlarfx_work(matrix_order: c_int, side: c_char, m: c_int, n: c_int, v: *const c_double, tau: c_double, c: *mut c_double, ldc: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_clarfx_work(matrix_order: c_int, side: c_char, m: c_int, n: c_int, v: *const c_void, tau: c_void, c: *mut c_void, ldc: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zlarfx_work(matrix_order: c_int, side: c_char, m: c_int, n: c_int, v: *const c_void, tau: c_void, c: *mut c_void, ldc: c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_slarnv_work(idist: c_int, iseed: *mut c_int, n: c_int, x: *mut c_float) -> c_int;
    pub fn LAPACKE_dlarnv_work(idist: c_int, iseed: *mut c_int, n: c_int, x: *mut c_double) -> c_int;
    pub fn LAPACKE_clarnv_work(idist: c_int, iseed: *mut c_int, n: c_int, x: *mut c_void) -> c_int;
    pub fn LAPACKE_zlarnv_work(idist: c_int, iseed: *mut c_int, n: c_int, x: *mut c_void) -> c_int;

    pub fn LAPACKE_slaset_work(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, alpha: c_float, beta: c_float, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dlaset_work(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, alpha: c_double, beta: c_double, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_claset_work(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, alpha: c_void, beta: c_void, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_zlaset_work(matrix_order: c_int, uplo: c_char, m: c_int, n: c_int, alpha: c_void, beta: c_void, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_slasrt_work(id: c_char, n: c_int, d: *mut c_float) -> c_int;
    pub fn LAPACKE_dlasrt_work(id: c_char, n: c_int, d: *mut c_double) -> c_int;

    pub fn LAPACKE_slaswp_work(matrix_order: c_int, n: c_int, a: *mut c_float, lda: c_int, k1: c_int, k2: c_int, ipiv: *const c_int, incx: c_int) -> c_int;
    pub fn LAPACKE_dlaswp_work(matrix_order: c_int, n: c_int, a: *mut c_double, lda: c_int, k1: c_int, k2: c_int, ipiv: *const c_int, incx: c_int) -> c_int;
    pub fn LAPACKE_claswp_work(matrix_order: c_int, n: c_int, a: *mut c_void, lda: c_int, k1: c_int, k2: c_int, ipiv: *const c_int, incx: c_int) -> c_int;
    pub fn LAPACKE_zlaswp_work(matrix_order: c_int, n: c_int, a: *mut c_void, lda: c_int, k1: c_int, k2: c_int, ipiv: *const c_int, incx: c_int) -> c_int;

    pub fn LAPACKE_slatms_work(matrix_order: c_int, m: c_int, n: c_int, dist: c_char, iseed: *mut c_int, sym: c_char, d: *mut c_float, mode: c_int, cond: c_float, dmax: c_float, kl: c_int, ku: c_int, pack: c_char, a: *mut c_float, lda: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dlatms_work(matrix_order: c_int, m: c_int, n: c_int, dist: c_char, iseed: *mut c_int, sym: c_char, d: *mut c_double, mode: c_int, cond: c_double, dmax: c_double, kl: c_int, ku: c_int, pack: c_char, a: *mut c_double, lda: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_clatms_work(matrix_order: c_int, m: c_int, n: c_int, dist: c_char, iseed: *mut c_int, sym: c_char, d: *mut c_float, mode: c_int, cond: c_float, dmax: c_float, kl: c_int, ku: c_int, pack: c_char, a: *mut c_void, lda: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zlatms_work(matrix_order: c_int, m: c_int, n: c_int, dist: c_char, iseed: *mut c_int, sym: c_char, d: *mut c_double, mode: c_int, cond: c_double, dmax: c_double, kl: c_int, ku: c_int, pack: c_char, a: *mut c_void, lda: c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_slauum_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dlauum_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_clauum_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_zlauum_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_sopgtr_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_float, tau: *const c_float, q: *mut c_float, ldq: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dopgtr_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_double, tau: *const c_double, q: *mut c_double, ldq: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_sopmtr_work(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, ap: *const c_float, tau: *const c_float, c: *mut c_float, ldc: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dopmtr_work(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, ap: *const c_double, tau: *const c_double, c: *mut c_double, ldc: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_sorgbr_work(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, k: c_int, a: *mut c_float, lda: c_int, tau: *const c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dorgbr_work(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, k: c_int, a: *mut c_double, lda: c_int, tau: *const c_double, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sorghr_work(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_float, lda: c_int, tau: *const c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dorghr_work(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_double, lda: c_int, tau: *const c_double, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sorglq_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_float, lda: c_int, tau: *const c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dorglq_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_double, lda: c_int, tau: *const c_double, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sorgql_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_float, lda: c_int, tau: *const c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dorgql_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_double, lda: c_int, tau: *const c_double, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sorgqr_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_float, lda: c_int, tau: *const c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dorgqr_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_double, lda: c_int, tau: *const c_double, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sorgrq_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_float, lda: c_int, tau: *const c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dorgrq_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_double, lda: c_int, tau: *const c_double, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sorgtr_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, tau: *const c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dorgtr_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, tau: *const c_double, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sormbr_work(matrix_order: c_int, vect: c_char, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dormbr_work(matrix_order: c_int, vect: c_char, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sormhr_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dormhr_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sormlq_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dormlq_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sormql_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dormql_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sormqr_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dormqr_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sormrq_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dormrq_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sormrz_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dormrz_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_sormtr_work(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, a: *const c_float, lda: c_int, tau: *const c_float, c: *mut c_float, ldc: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dormtr_work(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, a: *const c_double, lda: c_int, tau: *const c_double, c: *mut c_double, ldc: c_int, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_spbcon_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_float, ldab: c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dpbcon_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_double, ldab: c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cpbcon_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zpbcon_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_spbequ_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_float, ldab: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dpbequ_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_double, ldab: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cpbequ_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zpbequ_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_spbrfs_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, afb: *const c_float, ldafb: c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dpbrfs_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, afb: *const c_double, ldafb: c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cpbrfs_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zpbrfs_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, afb: *const c_void, ldafb: c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_spbstf_work(matrix_order: c_int, uplo: c_char, n: c_int, kb: c_int, bb: *mut c_float, ldbb: c_int) -> c_int;
    pub fn LAPACKE_dpbstf_work(matrix_order: c_int, uplo: c_char, n: c_int, kb: c_int, bb: *mut c_double, ldbb: c_int) -> c_int;
    pub fn LAPACKE_cpbstf_work(matrix_order: c_int, uplo: c_char, n: c_int, kb: c_int, bb: *mut c_void, ldbb: c_int) -> c_int;
    pub fn LAPACKE_zpbstf_work(matrix_order: c_int, uplo: c_char, n: c_int, kb: c_int, bb: *mut c_void, ldbb: c_int) -> c_int;

    pub fn LAPACKE_spbsv_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_float, ldab: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpbsv_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_double, ldab: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpbsv_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpbsv_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_spbsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_float, ldab: c_int, afb: *mut c_float, ldafb: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dpbsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_double, ldab: c_int, afb: *mut c_double, ldafb: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cpbsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zpbsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *mut c_void, ldab: c_int, afb: *mut c_void, ldafb: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_spbtrf_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_float, ldab: c_int) -> c_int;
    pub fn LAPACKE_dpbtrf_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_double, ldab: c_int) -> c_int;
    pub fn LAPACKE_cpbtrf_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int) -> c_int;
    pub fn LAPACKE_zpbtrf_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_void, ldab: c_int) -> c_int;

    pub fn LAPACKE_spbtrs_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpbtrs_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpbtrs_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpbtrs_work(matrix_order: c_int, uplo: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_spftrf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_float) -> c_int;
    pub fn LAPACKE_dpftrf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_double) -> c_int;
    pub fn LAPACKE_cpftrf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_void) -> c_int;
    pub fn LAPACKE_zpftrf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_void) -> c_int;

    pub fn LAPACKE_spftri_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_float) -> c_int;
    pub fn LAPACKE_dpftri_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_double) -> c_int;
    pub fn LAPACKE_cpftri_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_void) -> c_int;
    pub fn LAPACKE_zpftri_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *mut c_void) -> c_int;

    pub fn LAPACKE_spftrs_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpftrs_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpftrs_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpftrs_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_spocon_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_float, lda: c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dpocon_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_double, lda: c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cpocon_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zpocon_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_spoequ_work(matrix_order: c_int, n: c_int, a: *const c_float, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dpoequ_work(matrix_order: c_int, n: c_int, a: *const c_double, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cpoequ_work(matrix_order: c_int, n: c_int, a: *const c_void, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zpoequ_work(matrix_order: c_int, n: c_int, a: *const c_void, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_spoequb_work(matrix_order: c_int, n: c_int, a: *const c_float, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dpoequb_work(matrix_order: c_int, n: c_int, a: *const c_double, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cpoequb_work(matrix_order: c_int, n: c_int, a: *const c_void, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zpoequb_work(matrix_order: c_int, n: c_int, a: *const c_void, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_sporfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dporfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cporfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zporfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sporfsx_work(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, s: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dporfsx_work(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, s: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cporfsx_work(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, s: *const c_float, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zporfsx_work(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, s: *const c_double, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sposv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dposv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cposv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zposv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_dsposv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, work: *mut c_double, swork: *mut c_float, iter: *mut c_int) -> c_int;
    pub fn LAPACKE_zcposv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, work: *mut c_void, swork: *mut c_void, rwork: *mut c_double, iter: *mut c_int) -> c_int;

    pub fn LAPACKE_sposvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, af: *mut c_float, ldaf: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dposvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, af: *mut c_double, ldaf: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cposvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zposvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sposvxx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, af: *mut c_float, ldaf: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dposvxx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, af: *mut c_double, ldaf: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cposvxx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zposvxx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_spotrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dpotrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_cpotrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_zpotrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_spotri_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dpotri_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_cpotri_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_zpotri_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_spotrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpotrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpotrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpotrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sppcon_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_float, anorm: c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dppcon_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_double, anorm: c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cppcon_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, anorm: c_float, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zppcon_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, anorm: c_double, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sppequ_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_float, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_dppequ_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_double, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;
    pub fn LAPACKE_cppequ_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, s: *mut c_float, scond: *mut c_float, amax: *mut c_float) -> c_int;
    pub fn LAPACKE_zppequ_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, s: *mut c_double, scond: *mut c_double, amax: *mut c_double) -> c_int;

    pub fn LAPACKE_spprfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_float, afp: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dpprfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_double, afp: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cpprfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zpprfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sppsv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dppsv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cppsv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zppsv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sppsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_float, afp: *mut c_float, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dppsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_double, afp: *mut c_double, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cppsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, afp: *mut c_void, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zppsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, afp: *mut c_void, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_spptrf_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_float) -> c_int;
    pub fn LAPACKE_dpptrf_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_double) -> c_int;
    pub fn LAPACKE_cpptrf_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void) -> c_int;
    pub fn LAPACKE_zpptrf_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void) -> c_int;

    pub fn LAPACKE_spptri_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_float) -> c_int;
    pub fn LAPACKE_dpptri_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_double) -> c_int;
    pub fn LAPACKE_cpptri_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void) -> c_int;
    pub fn LAPACKE_zpptri_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void) -> c_int;

    pub fn LAPACKE_spptrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpptrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpptrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpptrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_spstrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, piv: *mut c_int, rank: *mut c_int, tol: c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dpstrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, piv: *mut c_int, rank: *mut c_int, tol: c_double, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cpstrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, piv: *mut c_int, rank: *mut c_int, tol: c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_zpstrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, piv: *mut c_int, rank: *mut c_int, tol: c_double, work: *mut c_double) -> c_int;

    pub fn LAPACKE_sptcon_work(n: c_int, d: *const c_float, e: *const c_float, anorm: c_float, rcond: *mut c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dptcon_work(n: c_int, d: *const c_double, e: *const c_double, anorm: c_double, rcond: *mut c_double, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cptcon_work(n: c_int, d: *const c_float, e: *const c_void, anorm: c_float, rcond: *mut c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_zptcon_work(n: c_int, d: *const c_double, e: *const c_void, anorm: c_double, rcond: *mut c_double, work: *mut c_double) -> c_int;

    pub fn LAPACKE_spteqr_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dpteqr_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cpteqr_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_zpteqr_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_sptrfs_work(matrix_order: c_int, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_float, df: *const c_float, ef: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dptrfs_work(matrix_order: c_int, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_double, df: *const c_double, ef: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cptrfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_void, df: *const c_float, ef: *const c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zptrfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_void, df: *const c_double, ef: *const c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sptsv_work(matrix_order: c_int, n: c_int, nrhs: c_int, d: *mut c_float, e: *mut c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dptsv_work(matrix_order: c_int, n: c_int, nrhs: c_int, d: *mut c_double, e: *mut c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cptsv_work(matrix_order: c_int, n: c_int, nrhs: c_int, d: *mut c_float, e: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zptsv_work(matrix_order: c_int, n: c_int, nrhs: c_int, d: *mut c_double, e: *mut c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sptsvx_work(matrix_order: c_int, fact: c_char, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_float, df: *mut c_float, ef: *mut c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dptsvx_work(matrix_order: c_int, fact: c_char, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_double, df: *mut c_double, ef: *mut c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cptsvx_work(matrix_order: c_int, fact: c_char, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_void, df: *mut c_float, ef: *mut c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zptsvx_work(matrix_order: c_int, fact: c_char, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_void, df: *mut c_double, ef: *mut c_void, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_spttrf_work(n: c_int, d: *mut c_float, e: *mut c_float) -> c_int;
    pub fn LAPACKE_dpttrf_work(n: c_int, d: *mut c_double, e: *mut c_double) -> c_int;
    pub fn LAPACKE_cpttrf_work(n: c_int, d: *mut c_float, e: *mut c_void) -> c_int;
    pub fn LAPACKE_zpttrf_work(n: c_int, d: *mut c_double, e: *mut c_void) -> c_int;

    pub fn LAPACKE_spttrs_work(matrix_order: c_int, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dpttrs_work(matrix_order: c_int, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cpttrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, d: *const c_float, e: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zpttrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, d: *const c_double, e: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_ssbev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_float, ldab: c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dsbev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_double, ldab: c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_ssbevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_float, ldab: c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dsbevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_double, ldab: c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_ssbevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_float, ldab: c_int, q: *mut c_float, ldq: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dsbevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_double, ldab: c_int, q: *mut c_double, ldq: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssbgst_work(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_float, ldab: c_int, bb: *const c_float, ldbb: c_int, x: *mut c_float, ldx: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dsbgst_work(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_double, ldab: c_int, bb: *const c_double, ldbb: c_int, x: *mut c_double, ldx: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_ssbgv_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_float, ldab: c_int, bb: *mut c_float, ldbb: c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dsbgv_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_double, ldab: c_int, bb: *mut c_double, ldbb: c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_ssbgvd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_float, ldab: c_int, bb: *mut c_float, ldbb: c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dsbgvd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_double, ldab: c_int, bb: *mut c_double, ldbb: c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_ssbgvx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_float, ldab: c_int, bb: *mut c_float, ldbb: c_int, q: *mut c_float, ldq: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dsbgvx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ka: c_int, kb: c_int, ab: *mut c_double, ldab: c_int, bb: *mut c_double, ldbb: c_int, q: *mut c_double, ldq: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssbtrd_work(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_float, ldab: c_int, d: *mut c_float, e: *mut c_float, q: *mut c_float, ldq: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dsbtrd_work(matrix_order: c_int, vect: c_char, uplo: c_char, n: c_int, kd: c_int, ab: *mut c_double, ldab: c_int, d: *mut c_double, e: *mut c_double, q: *mut c_double, ldq: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_ssfrk_work(matrix_order: c_int, transr: c_char, uplo: c_char, trans: c_char, n: c_int, k: c_int, alpha: c_float, a: *const c_float, lda: c_int, beta: c_float, c: *mut c_float) -> c_int;
    pub fn LAPACKE_dsfrk_work(matrix_order: c_int, transr: c_char, uplo: c_char, trans: c_char, n: c_int, k: c_int, alpha: c_double, a: *const c_double, lda: c_int, beta: c_double, c: *mut c_double) -> c_int;

    pub fn LAPACKE_sspcon_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_float, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dspcon_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_double, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cspcon_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zspcon_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_void) -> c_int;

    pub fn LAPACKE_sspev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dspev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_sspevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dspevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_sspevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dspevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_sspgst_work(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, ap: *mut c_float, bp: *const c_float) -> c_int;
    pub fn LAPACKE_dspgst_work(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, ap: *mut c_double, bp: *const c_double) -> c_int;

    pub fn LAPACKE_sspgv_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dspgv_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_double, bp: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_sspgvd_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_float, bp: *mut c_float, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dspgvd_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, ap: *mut c_double, bp: *mut c_double, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_sspgvx_work(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_float, bp: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dspgvx_work(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, ap: *mut c_double, bp: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssprfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_float, afp: *const c_float, ipiv: *const c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dsprfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_double, afp: *const c_double, ipiv: *const c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_csprfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zsprfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *const c_void, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_sspsv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_float, ipiv: *mut c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dspsv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_double, ipiv: *mut c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_cspsv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zspsv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *mut c_void, ipiv: *mut c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sspsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_float, afp: *mut c_float, ipiv: *mut c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dspsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_double, afp: *mut c_double, ipiv: *mut c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_cspsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zspsvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, afp: *mut c_void, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_ssptrd_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_float, d: *mut c_float, e: *mut c_float, tau: *mut c_float) -> c_int;
    pub fn LAPACKE_dsptrd_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_double, d: *mut c_double, e: *mut c_double, tau: *mut c_double) -> c_int;

    pub fn LAPACKE_ssptrf_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_float, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_dsptrf_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_double, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_csptrf_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *mut c_int) -> c_int;
    pub fn LAPACKE_zsptrf_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *mut c_int) -> c_int;

    pub fn LAPACKE_ssptri_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_float, ipiv: *const c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dsptri_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_double, ipiv: *const c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_csptri_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *const c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zsptri_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *mut c_void, ipiv: *const c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_ssptrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_float, ipiv: *const c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dsptrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_double, ipiv: *const c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_csptrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zsptrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, ap: *const c_void, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_sstebz_work(range: c_char, order: c_char, n: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, d: *const c_float, e: *const c_float, m: *mut c_int, nsplit: *mut c_int, w: *mut c_float, iblock: *mut c_int, isplit: *mut c_int, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dstebz_work(range: c_char, order: c_char, n: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, d: *const c_double, e: *const c_double, m: *mut c_int, nsplit: *mut c_int, w: *mut c_double, iblock: *mut c_int, isplit: *mut c_int, work: *mut c_double, iwork: *mut c_int) -> c_int;

    pub fn LAPACKE_sstedc_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dstedc_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_cstedc_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_zstedc_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, lrwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_sstegr_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, isuppz: *mut c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dstegr_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, isuppz: *mut c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_cstegr_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, isuppz: *mut c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_zstegr_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, isuppz: *mut c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_sstein_work(matrix_order: c_int, n: c_int, d: *const c_float, e: *const c_float, m: c_int, w: *const c_float, iblock: *const c_int, isplit: *const c_int, z: *mut c_float, ldz: c_int, work: *mut c_float, iwork: *mut c_int, ifailv: *mut c_int) -> c_int;
    pub fn LAPACKE_dstein_work(matrix_order: c_int, n: c_int, d: *const c_double, e: *const c_double, m: c_int, w: *const c_double, iblock: *const c_int, isplit: *const c_int, z: *mut c_double, ldz: c_int, work: *mut c_double, iwork: *mut c_int, ifailv: *mut c_int) -> c_int;
    pub fn LAPACKE_cstein_work(matrix_order: c_int, n: c_int, d: *const c_float, e: *const c_float, m: c_int, w: *const c_float, iblock: *const c_int, isplit: *const c_int, z: *mut c_void, ldz: c_int, work: *mut c_float, iwork: *mut c_int, ifailv: *mut c_int) -> c_int;
    pub fn LAPACKE_zstein_work(matrix_order: c_int, n: c_int, d: *const c_double, e: *const c_double, m: c_int, w: *const c_double, iblock: *const c_int, isplit: *const c_int, z: *mut c_void, ldz: c_int, work: *mut c_double, iwork: *mut c_int, ifailv: *mut c_int) -> c_int;

    pub fn LAPACKE_sstemr_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, nzc: c_int, isuppz: *mut c_int, tryrac: *mut c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dstemr_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, nzc: c_int, isuppz: *mut c_int, tryrac: *mut c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_cstemr_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, m: *mut c_int, w: *mut c_float, z: *mut c_void, ldz: c_int, nzc: c_int, isuppz: *mut c_int, tryrac: *mut c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_zstemr_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, m: *mut c_int, w: *mut c_double, z: *mut c_void, ldz: c_int, nzc: c_int, isuppz: *mut c_int, tryrac: *mut c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_ssteqr_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dsteqr_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_csteqr_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_void, ldz: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_zsteqr_work(matrix_order: c_int, compz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_void, ldz: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_ssterf_work(n: c_int, d: *mut c_float, e: *mut c_float) -> c_int;
    pub fn LAPACKE_dsterf_work(n: c_int, d: *mut c_double, e: *mut c_double) -> c_int;

    pub fn LAPACKE_sstev_work(matrix_order: c_int, jobz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dstev_work(matrix_order: c_int, jobz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double) -> c_int;

    pub fn LAPACKE_sstevd_work(matrix_order: c_int, jobz: c_char, n: c_int, d: *mut c_float, e: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dstevd_work(matrix_order: c_int, jobz: c_char, n: c_int, d: *mut c_double, e: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_sstevr_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, isuppz: *mut c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dstevr_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, isuppz: *mut c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_sstevx_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_float, e: *mut c_float, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dstevx_work(matrix_order: c_int, jobz: c_char, range: c_char, n: c_int, d: *mut c_double, e: *mut c_double, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssycon_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_float, lda: c_int, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dsycon_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_double, lda: c_int, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_csycon_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, anorm: c_float, rcond: *mut c_float, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zsycon_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, anorm: c_double, rcond: *mut c_double, work: *mut c_void) -> c_int;

    pub fn LAPACKE_ssyequb_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_float, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dsyequb_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_double, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, work: *mut c_double) -> c_int;
    pub fn LAPACKE_csyequb_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, s: *mut c_float, scond: *mut c_float, amax: *mut c_float, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zsyequb_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, s: *mut c_double, scond: *mut c_double, amax: *mut c_double, work: *mut c_void) -> c_int;

    pub fn LAPACKE_ssyev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, w: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dsyev_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, w: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_ssyevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, w: *mut c_float, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dsyevd_work(matrix_order: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, w: *mut c_double, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_ssyevr_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, isuppz: *mut c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dsyevr_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, isuppz: *mut c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_ssyevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dsyevx_work(matrix_order: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssygst_work(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *const c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dsygst_work(matrix_order: c_int, itype: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *const c_double, ldb: c_int) -> c_int;

    pub fn LAPACKE_ssygv_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, w: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dsygv_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, w: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_ssygvd_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, w: *mut c_float, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dsygvd_work(matrix_order: c_int, itype: c_int, jobz: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, w: *mut c_double, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_ssygvx_work(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, vl: c_float, vu: c_float, il: c_int, iu: c_int, abstol: c_float, m: *mut c_int, w: *mut c_float, z: *mut c_float, ldz: c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int, ifail: *mut c_int) -> c_int;
    pub fn LAPACKE_dsygvx_work(matrix_order: c_int, itype: c_int, jobz: c_char, range: c_char, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, vl: c_double, vu: c_double, il: c_int, iu: c_int, abstol: c_double, m: *mut c_int, w: *mut c_double, z: *mut c_double, ldz: c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int, ifail: *mut c_int) -> c_int;

    pub fn LAPACKE_ssyrfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, ipiv: *const c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dsyrfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, ipiv: *const c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_csyrfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zsyrfs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_ssyrfsx_work(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *const c_float, ldaf: c_int, ipiv: *const c_int, s: *const c_float, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dsyrfsx_work(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *const c_double, ldaf: c_int, ipiv: *const c_int, s: *const c_double, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_csyrfsx_work(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, s: *const c_float, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zsyrfsx_work(matrix_order: c_int, uplo: c_char, equed: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *const c_void, ldaf: c_int, ipiv: *const c_int, s: *const c_double, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_ssysv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int, b: *mut c_float, ldb: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dsysv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int, b: *mut c_double, ldb: c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_csysv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zsysv_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_ssysvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, af: *mut c_float, ldaf: c_int, ipiv: *mut c_int, b: *const c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dsysvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, af: *mut c_double, ldaf: c_int, ipiv: *mut c_int, b: *const c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_csysvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, lwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zsysvx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, b: *const c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, lwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_ssysvxx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, af: *mut c_float, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_float, ldb: c_int, x: *mut c_float, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dsysvxx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, af: *mut c_double, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_double, ldb: c_int, x: *mut c_double, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_csysvxx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_float, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_float, rpvgrw: *mut c_float, berr: *mut c_float, n_err_bnds: c_int, err_bnds_norm: *mut c_float, err_bnds_comp: *mut c_float, nparams: c_int, params: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_zsysvxx_work(matrix_order: c_int, fact: c_char, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, af: *mut c_void, ldaf: c_int, ipiv: *mut c_int, equed: *mut c_char, s: *mut c_double, b: *mut c_void, ldb: c_int, x: *mut c_void, ldx: c_int, rcond: *mut c_double, rpvgrw: *mut c_double, berr: *mut c_double, n_err_bnds: c_int, err_bnds_norm: *mut c_double, err_bnds_comp: *mut c_double, nparams: c_int, params: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_ssytrd_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, d: *mut c_float, e: *mut c_float, tau: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dsytrd_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, d: *mut c_double, e: *mut c_double, tau: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;

    pub fn LAPACKE_ssytrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dsytrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_csytrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zsytrf_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_ssytri_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, ipiv: *const c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dsytri_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, ipiv: *const c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_csytri_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zsytri_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_ssytrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, ipiv: *const c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dsytrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, ipiv: *const c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_csytrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_zsytrs_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_stbcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, kd: c_int, ab: *const c_float, ldab: c_int, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dtbcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, kd: c_int, ab: *const c_double, ldab: c_int, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ctbcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_ztbcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, kd: c_int, ab: *const c_void, ldab: c_int, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_stbrfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, b: *const c_float, ldb: c_int, x: *const c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dtbrfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, b: *const c_double, ldb: c_int, x: *const c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ctbrfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_ztbrfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_stbtrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_float, ldab: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dtbtrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_double, ldab: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_ctbtrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_ztbtrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, kd: c_int, nrhs: c_int, ab: *const c_void, ldab: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_stfsm_work(matrix_order: c_int, transr: c_char, side: c_char, uplo: c_char, trans: c_char, diag: c_char, m: c_int, n: c_int, alpha: c_float, a: *const c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dtfsm_work(matrix_order: c_int, transr: c_char, side: c_char, uplo: c_char, trans: c_char, diag: c_char, m: c_int, n: c_int, alpha: c_double, a: *const c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_ctfsm_work(matrix_order: c_int, transr: c_char, side: c_char, uplo: c_char, trans: c_char, diag: c_char, m: c_int, n: c_int, alpha: c_void, a: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_ztfsm_work(matrix_order: c_int, transr: c_char, side: c_char, uplo: c_char, trans: c_char, diag: c_char, m: c_int, n: c_int, alpha: c_void, a: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_stftri_work(matrix_order: c_int, transr: c_char, uplo: c_char, diag: c_char, n: c_int, a: *mut c_float) -> c_int;
    pub fn LAPACKE_dtftri_work(matrix_order: c_int, transr: c_char, uplo: c_char, diag: c_char, n: c_int, a: *mut c_double) -> c_int;
    pub fn LAPACKE_ctftri_work(matrix_order: c_int, transr: c_char, uplo: c_char, diag: c_char, n: c_int, a: *mut c_void) -> c_int;
    pub fn LAPACKE_ztftri_work(matrix_order: c_int, transr: c_char, uplo: c_char, diag: c_char, n: c_int, a: *mut c_void) -> c_int;

    pub fn LAPACKE_stfttp_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_float, ap: *mut c_float) -> c_int;
    pub fn LAPACKE_dtfttp_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_double, ap: *mut c_double) -> c_int;
    pub fn LAPACKE_ctfttp_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_void, ap: *mut c_void) -> c_int;
    pub fn LAPACKE_ztfttp_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_void, ap: *mut c_void) -> c_int;

    pub fn LAPACKE_stfttr_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_float, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dtfttr_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_double, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_ctfttr_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_void, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_ztfttr_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, arf: *const c_void, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_stgevc_work(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, s: *const c_float, lds: c_int, p: *const c_float, ldp: c_int, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dtgevc_work(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, s: *const c_double, lds: c_int, p: *const c_double, ldp: c_int, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_ctgevc_work(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, s: *const c_void, lds: c_int, p: *const c_void, ldp: c_int, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_ztgevc_work(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, s: *const c_void, lds: c_int, p: *const c_void, ldp: c_int, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_stgexc_work(matrix_order: c_int, wantq: c_int, wantz: c_int, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, q: *mut c_float, ldq: c_int, z: *mut c_float, ldz: c_int, ifst: *mut c_int, ilst: *mut c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dtgexc_work(matrix_order: c_int, wantq: c_int, wantz: c_int, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, q: *mut c_double, ldq: c_int, z: *mut c_double, ldz: c_int, ifst: *mut c_int, ilst: *mut c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_ctgexc_work(matrix_order: c_int, wantq: c_int, wantz: c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int, ifst: c_int, ilst: c_int) -> c_int;
    pub fn LAPACKE_ztgexc_work(matrix_order: c_int, wantq: c_int, wantz: c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int, ifst: c_int, ilst: c_int) -> c_int;

    pub fn LAPACKE_stgsen_work(matrix_order: c_int, ijob: c_int, wantq: c_int, wantz: c_int, select: *const c_int, n: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, alphar: *mut c_float, alphai: *mut c_float, beta: *mut c_float, q: *mut c_float, ldq: c_int, z: *mut c_float, ldz: c_int, m: *mut c_int, pl: *mut c_float, pr: *mut c_float, dif: *mut c_float, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dtgsen_work(matrix_order: c_int, ijob: c_int, wantq: c_int, wantz: c_int, select: *const c_int, n: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, alphar: *mut c_double, alphai: *mut c_double, beta: *mut c_double, q: *mut c_double, ldq: c_int, z: *mut c_double, ldz: c_int, m: *mut c_int, pl: *mut c_double, pr: *mut c_double, dif: *mut c_double, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_ctgsen_work(matrix_order: c_int, ijob: c_int, wantq: c_int, wantz: c_int, select: *const c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int, m: *mut c_int, pl: *mut c_float, pr: *mut c_float, dif: *mut c_float, work: *mut c_void, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_ztgsen_work(matrix_order: c_int, ijob: c_int, wantq: c_int, wantz: c_int, select: *const c_int, n: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, alpha: *mut c_void, beta: *mut c_void, q: *mut c_void, ldq: c_int, z: *mut c_void, ldz: c_int, m: *mut c_int, pl: *mut c_double, pr: *mut c_double, dif: *mut c_double, work: *mut c_void, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;

    pub fn LAPACKE_stgsja_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, k: c_int, l: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, tola: c_float, tolb: c_float, alpha: *mut c_float, beta: *mut c_float, u: *mut c_float, ldu: c_int, v: *mut c_float, ldv: c_int, q: *mut c_float, ldq: c_int, work: *mut c_float, ncycle: *mut c_int) -> c_int;
    pub fn LAPACKE_dtgsja_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, k: c_int, l: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, tola: c_double, tolb: c_double, alpha: *mut c_double, beta: *mut c_double, u: *mut c_double, ldu: c_int, v: *mut c_double, ldv: c_int, q: *mut c_double, ldq: c_int, work: *mut c_double, ncycle: *mut c_int) -> c_int;
    pub fn LAPACKE_ctgsja_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, k: c_int, l: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, tola: c_float, tolb: c_float, alpha: *mut c_float, beta: *mut c_float, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int, work: *mut c_void, ncycle: *mut c_int) -> c_int;
    pub fn LAPACKE_ztgsja_work(matrix_order: c_int, jobu: c_char, jobv: c_char, jobq: c_char, m: c_int, p: c_int, n: c_int, k: c_int, l: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, tola: c_double, tolb: c_double, alpha: *mut c_double, beta: *mut c_double, u: *mut c_void, ldu: c_int, v: *mut c_void, ldv: c_int, q: *mut c_void, ldq: c_int, work: *mut c_void, ncycle: *mut c_int) -> c_int;

    pub fn LAPACKE_stgsna_work(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, a: *const c_float, lda: c_int, b: *const c_float, ldb: c_int, vl: *const c_float, ldvl: c_int, vr: *const c_float, ldvr: c_int, s: *mut c_float, dif: *mut c_float, mm: c_int, m: *mut c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dtgsna_work(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, a: *const c_double, lda: c_int, b: *const c_double, ldb: c_int, vl: *const c_double, ldvl: c_int, vr: *const c_double, ldvr: c_int, s: *mut c_double, dif: *mut c_double, mm: c_int, m: *mut c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ctgsna_work(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, vl: *const c_void, ldvl: c_int, vr: *const c_void, ldvr: c_int, s: *mut c_float, dif: *mut c_float, mm: c_int, m: *mut c_int, work: *mut c_void, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ztgsna_work(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, vl: *const c_void, ldvl: c_int, vr: *const c_void, ldvr: c_int, s: *mut c_double, dif: *mut c_double, mm: c_int, m: *mut c_int, work: *mut c_void, lwork: c_int, iwork: *mut c_int) -> c_int;

    pub fn LAPACKE_stgsyl_work(matrix_order: c_int, trans: c_char, ijob: c_int, m: c_int, n: c_int, a: *const c_float, lda: c_int, b: *const c_float, ldb: c_int, c: *mut c_float, ldc: c_int, d: *const c_float, ldd: c_int, e: *const c_float, lde: c_int, f: *mut c_float, ldf: c_int, scale: *mut c_float, dif: *mut c_float, work: *mut c_float, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dtgsyl_work(matrix_order: c_int, trans: c_char, ijob: c_int, m: c_int, n: c_int, a: *const c_double, lda: c_int, b: *const c_double, ldb: c_int, c: *mut c_double, ldc: c_int, d: *const c_double, ldd: c_int, e: *const c_double, lde: c_int, f: *mut c_double, ldf: c_int, scale: *mut c_double, dif: *mut c_double, work: *mut c_double, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ctgsyl_work(matrix_order: c_int, trans: c_char, ijob: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, c: *mut c_void, ldc: c_int, d: *const c_void, ldd: c_int, e: *const c_void, lde: c_int, f: *mut c_void, ldf: c_int, scale: *mut c_float, dif: *mut c_float, work: *mut c_void, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ztgsyl_work(matrix_order: c_int, trans: c_char, ijob: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, c: *mut c_void, ldc: c_int, d: *const c_void, ldd: c_int, e: *const c_void, lde: c_int, f: *mut c_void, ldf: c_int, scale: *mut c_double, dif: *mut c_double, work: *mut c_void, lwork: c_int, iwork: *mut c_int) -> c_int;

    pub fn LAPACKE_stpcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, ap: *const c_float, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dtpcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, ap: *const c_double, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ctpcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, ap: *const c_void, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_ztpcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, ap: *const c_void, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_stprfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_float, b: *const c_float, ldb: c_int, x: *const c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dtprfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_double, b: *const c_double, ldb: c_int, x: *const c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ctprfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_ztprfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_stptri_work(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, ap: *mut c_float) -> c_int;
    pub fn LAPACKE_dtptri_work(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, ap: *mut c_double) -> c_int;
    pub fn LAPACKE_ctptri_work(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, ap: *mut c_void) -> c_int;
    pub fn LAPACKE_ztptri_work(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, ap: *mut c_void) -> c_int;

    pub fn LAPACKE_stptrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_float, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dtptrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_double, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_ctptrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_ztptrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, ap: *const c_void, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_stpttf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, ap: *const c_float, arf: *mut c_float) -> c_int;
    pub fn LAPACKE_dtpttf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, ap: *const c_double, arf: *mut c_double) -> c_int;
    pub fn LAPACKE_ctpttf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, ap: *const c_void, arf: *mut c_void) -> c_int;
    pub fn LAPACKE_ztpttf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, ap: *const c_void, arf: *mut c_void) -> c_int;

    pub fn LAPACKE_stpttr_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_float, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dtpttr_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_double, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_ctpttr_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_ztpttr_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_strcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, a: *const c_float, lda: c_int, rcond: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dtrcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, a: *const c_double, lda: c_int, rcond: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ctrcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, a: *const c_void, lda: c_int, rcond: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_ztrcon_work(matrix_order: c_int, norm: c_char, uplo: c_char, diag: c_char, n: c_int, a: *const c_void, lda: c_int, rcond: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_strevc_work(matrix_order: c_int, side: c_char, howmny: c_char, select: *mut c_int, n: c_int, t: *const c_float, ldt: c_int, vl: *mut c_float, ldvl: c_int, vr: *mut c_float, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dtrevc_work(matrix_order: c_int, side: c_char, howmny: c_char, select: *mut c_int, n: c_int, t: *const c_double, ldt: c_int, vl: *mut c_double, ldvl: c_int, vr: *mut c_double, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_ctrevc_work(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *mut c_void, ldt: c_int, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_ztrevc_work(matrix_order: c_int, side: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *mut c_void, ldt: c_int, vl: *mut c_void, ldvl: c_int, vr: *mut c_void, ldvr: c_int, mm: c_int, m: *mut c_int, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_strexc_work(matrix_order: c_int, compq: c_char, n: c_int, t: *mut c_float, ldt: c_int, q: *mut c_float, ldq: c_int, ifst: *mut c_int, ilst: *mut c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dtrexc_work(matrix_order: c_int, compq: c_char, n: c_int, t: *mut c_double, ldt: c_int, q: *mut c_double, ldq: c_int, ifst: *mut c_int, ilst: *mut c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_ctrexc_work(matrix_order: c_int, compq: c_char, n: c_int, t: *mut c_void, ldt: c_int, q: *mut c_void, ldq: c_int, ifst: c_int, ilst: c_int) -> c_int;
    pub fn LAPACKE_ztrexc_work(matrix_order: c_int, compq: c_char, n: c_int, t: *mut c_void, ldt: c_int, q: *mut c_void, ldq: c_int, ifst: c_int, ilst: c_int) -> c_int;

    pub fn LAPACKE_strrfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, b: *const c_float, ldb: c_int, x: *const c_float, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_float, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dtrrfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, b: *const c_double, ldb: c_int, x: *const c_double, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_double, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ctrrfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_float, berr: *mut c_float, work: *mut c_void, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_ztrrfs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, x: *const c_void, ldx: c_int, ferr: *mut c_double, berr: *mut c_double, work: *mut c_void, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_strsen_work(matrix_order: c_int, job: c_char, compq: c_char, select: *const c_int, n: c_int, t: *mut c_float, ldt: c_int, q: *mut c_float, ldq: c_int, wr: *mut c_float, wi: *mut c_float, m: *mut c_int, s: *mut c_float, sep: *mut c_float, work: *mut c_float, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_dtrsen_work(matrix_order: c_int, job: c_char, compq: c_char, select: *const c_int, n: c_int, t: *mut c_double, ldt: c_int, q: *mut c_double, ldq: c_int, wr: *mut c_double, wi: *mut c_double, m: *mut c_int, s: *mut c_double, sep: *mut c_double, work: *mut c_double, lwork: c_int, iwork: *mut c_int, liwork: c_int) -> c_int;
    pub fn LAPACKE_ctrsen_work(matrix_order: c_int, job: c_char, compq: c_char, select: *const c_int, n: c_int, t: *mut c_void, ldt: c_int, q: *mut c_void, ldq: c_int, w: *mut c_void, m: *mut c_int, s: *mut c_float, sep: *mut c_float, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_ztrsen_work(matrix_order: c_int, job: c_char, compq: c_char, select: *const c_int, n: c_int, t: *mut c_void, ldt: c_int, q: *mut c_void, ldq: c_int, w: *mut c_void, m: *mut c_int, s: *mut c_double, sep: *mut c_double, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_strsna_work(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *const c_float, ldt: c_int, vl: *const c_float, ldvl: c_int, vr: *const c_float, ldvr: c_int, s: *mut c_float, sep: *mut c_float, mm: c_int, m: *mut c_int, work: *mut c_float, ldwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dtrsna_work(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *const c_double, ldt: c_int, vl: *const c_double, ldvl: c_int, vr: *const c_double, ldvr: c_int, s: *mut c_double, sep: *mut c_double, mm: c_int, m: *mut c_int, work: *mut c_double, ldwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ctrsna_work(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *const c_void, ldt: c_int, vl: *const c_void, ldvl: c_int, vr: *const c_void, ldvr: c_int, s: *mut c_float, sep: *mut c_float, mm: c_int, m: *mut c_int, work: *mut c_void, ldwork: c_int, rwork: *mut c_float) -> c_int;
    pub fn LAPACKE_ztrsna_work(matrix_order: c_int, job: c_char, howmny: c_char, select: *const c_int, n: c_int, t: *const c_void, ldt: c_int, vl: *const c_void, ldvl: c_int, vr: *const c_void, ldvr: c_int, s: *mut c_double, sep: *mut c_double, mm: c_int, m: *mut c_int, work: *mut c_void, ldwork: c_int, rwork: *mut c_double) -> c_int;

    pub fn LAPACKE_strsyl_work(matrix_order: c_int, trana: c_char, tranb: c_char, isgn: c_int, m: c_int, n: c_int, a: *const c_float, lda: c_int, b: *const c_float, ldb: c_int, c: *mut c_float, ldc: c_int, scale: *mut c_float) -> c_int;
    pub fn LAPACKE_dtrsyl_work(matrix_order: c_int, trana: c_char, tranb: c_char, isgn: c_int, m: c_int, n: c_int, a: *const c_double, lda: c_int, b: *const c_double, ldb: c_int, c: *mut c_double, ldc: c_int, scale: *mut c_double) -> c_int;
    pub fn LAPACKE_ctrsyl_work(matrix_order: c_int, trana: c_char, tranb: c_char, isgn: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, c: *mut c_void, ldc: c_int, scale: *mut c_float) -> c_int;
    pub fn LAPACKE_ztrsyl_work(matrix_order: c_int, trana: c_char, tranb: c_char, isgn: c_int, m: c_int, n: c_int, a: *const c_void, lda: c_int, b: *const c_void, ldb: c_int, c: *mut c_void, ldc: c_int, scale: *mut c_double) -> c_int;

    pub fn LAPACKE_strtri_work(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, a: *mut c_float, lda: c_int) -> c_int;
    pub fn LAPACKE_dtrtri_work(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, a: *mut c_double, lda: c_int) -> c_int;
    pub fn LAPACKE_ctrtri_work(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_ztrtri_work(matrix_order: c_int, uplo: c_char, diag: c_char, n: c_int, a: *mut c_void, lda: c_int) -> c_int;

    pub fn LAPACKE_strtrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, b: *mut c_float, ldb: c_int) -> c_int;
    pub fn LAPACKE_dtrtrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, b: *mut c_double, ldb: c_int) -> c_int;
    pub fn LAPACKE_ctrtrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;
    pub fn LAPACKE_ztrtrs_work(matrix_order: c_int, uplo: c_char, trans: c_char, diag: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, b: *mut c_void, ldb: c_int) -> c_int;

    pub fn LAPACKE_strttf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *const c_float, lda: c_int, arf: *mut c_float) -> c_int;
    pub fn LAPACKE_dtrttf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *const c_double, lda: c_int, arf: *mut c_double) -> c_int;
    pub fn LAPACKE_ctrttf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, arf: *mut c_void) -> c_int;
    pub fn LAPACKE_ztrttf_work(matrix_order: c_int, transr: c_char, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, arf: *mut c_void) -> c_int;

    pub fn LAPACKE_strttp_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_float, lda: c_int, ap: *mut c_float) -> c_int;
    pub fn LAPACKE_dtrttp_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_double, lda: c_int, ap: *mut c_double) -> c_int;
    pub fn LAPACKE_ctrttp_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ap: *mut c_void) -> c_int;
    pub fn LAPACKE_ztrttp_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *const c_void, lda: c_int, ap: *mut c_void) -> c_int;

    pub fn LAPACKE_stzrzf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, tau: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dtzrzf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, tau: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_ctzrzf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_ztzrzf_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cungbr_work(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zungbr_work(matrix_order: c_int, vect: c_char, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cunghr_work(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zunghr_work(matrix_order: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cunglq_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zunglq_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cungql_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zungql_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cungqr_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zungqr_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cungrq_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zungrq_work(matrix_order: c_int, m: c_int, n: c_int, k: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cungtr_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zungtr_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, tau: *const c_void, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cunmbr_work(matrix_order: c_int, vect: c_char, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zunmbr_work(matrix_order: c_int, vect: c_char, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cunmhr_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zunmhr_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, ilo: c_int, ihi: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cunmlq_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zunmlq_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cunmql_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zunmql_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cunmqr_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zunmqr_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cunmrq_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zunmrq_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cunmrz_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zunmrz_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cunmtr_work(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zunmtr_work(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, a: *const c_void, lda: c_int, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_cupgtr_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, tau: *const c_void, q: *mut c_void, ldq: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zupgtr_work(matrix_order: c_int, uplo: c_char, n: c_int, ap: *const c_void, tau: *const c_void, q: *mut c_void, ldq: c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_cupmtr_work(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, ap: *const c_void, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zupmtr_work(matrix_order: c_int, side: c_char, uplo: c_char, trans: c_char, m: c_int, n: c_int, ap: *const c_void, tau: *const c_void, c: *mut c_void, ldc: c_int, work: *mut c_void) -> c_int;

    // LAPACK 3.3.0
    pub fn LAPACKE_cbbcsd_work(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, m: c_int, p: c_int, q: c_int, theta: *mut c_float, phi: *mut c_float, u1: *mut c_void, ldu1: c_int, u2: *mut c_void, ldu2: c_int, v1t: *mut c_void, ldv1t: c_int, v2t: *mut c_void, ldv2t: c_int, b11d: *mut c_float, b11e: *mut c_float, b12d: *mut c_float, b12e: *mut c_float, b21d: *mut c_float, b21e: *mut c_float, b22d: *mut c_float, b22e: *mut c_float, rwork: *mut c_float, lrwork: c_int) -> c_int;
    pub fn LAPACKE_cheswapr_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_chetri2_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_chetri2x_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void, nb: c_int) -> c_int;
    pub fn LAPACKE_chetrs2_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_csyconv_work(matrix_order: c_int, uplo: c_char, way: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_csyswapr_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_csytri2_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_csytri2x_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void, nb: c_int) -> c_int;
    pub fn LAPACKE_csytrs2_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_cunbdb_work(matrix_order: c_int, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_void, ldx11: c_int, x12: *mut c_void, ldx12: c_int, x21: *mut c_void, ldx21: c_int, x22: *mut c_void, ldx22: c_int, theta: *mut c_float, phi: *mut c_float, taup1: *mut c_void, taup2: *mut c_void, tauq1: *mut c_void, tauq2: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_cuncsd_work(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_void, ldx11: c_int, x12: *mut c_void, ldx12: c_int, x21: *mut c_void, ldx21: c_int, x22: *mut c_void, ldx22: c_int, theta: *mut c_float, u1: *mut c_void, ldu1: c_int, u2: *mut c_void, ldu2: c_int, v1t: *mut c_void, ldv1t: c_int, v2t: *mut c_void, ldv2t: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_float, lrwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dbbcsd_work(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, m: c_int, p: c_int, q: c_int, theta: *mut c_double, phi: *mut c_double, u1: *mut c_double, ldu1: c_int, u2: *mut c_double, ldu2: c_int, v1t: *mut c_double, ldv1t: c_int, v2t: *mut c_double, ldv2t: c_int, b11d: *mut c_double, b11e: *mut c_double, b12d: *mut c_double, b12e: *mut c_double, b21d: *mut c_double, b21e: *mut c_double, b22d: *mut c_double, b22e: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_dorbdb_work(matrix_order: c_int, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_double, ldx11: c_int, x12: *mut c_double, ldx12: c_int, x21: *mut c_double, ldx21: c_int, x22: *mut c_double, ldx22: c_int, theta: *mut c_double, phi: *mut c_double, taup1: *mut c_double, taup2: *mut c_double, tauq1: *mut c_double, tauq2: *mut c_double, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_dorcsd_work(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_double, ldx11: c_int, x12: *mut c_double, ldx12: c_int, x21: *mut c_double, ldx21: c_int, x22: *mut c_double, ldx22: c_int, theta: *mut c_double, u1: *mut c_double, ldu1: c_int, u2: *mut c_double, ldu2: c_int, v1t: *mut c_double, ldv1t: c_int, v2t: *mut c_double, ldv2t: c_int, work: *mut c_double, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_dsyconv_work(matrix_order: c_int, uplo: c_char, way: c_char, n: c_int, a: *mut c_double, lda: c_int, ipiv: *const c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_dsyswapr_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_dsytri2_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, ipiv: *const c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_dsytri2x_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_double, lda: c_int, ipiv: *const c_int, work: *mut c_double, nb: c_int) -> c_int;
    pub fn LAPACKE_dsytrs2_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_double, lda: c_int, ipiv: *const c_int, b: *mut c_double, ldb: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_sbbcsd_work(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, m: c_int, p: c_int, q: c_int, theta: *mut c_float, phi: *mut c_float, u1: *mut c_float, ldu1: c_int, u2: *mut c_float, ldu2: c_int, v1t: *mut c_float, ldv1t: c_int, v2t: *mut c_float, ldv2t: c_int, b11d: *mut c_float, b11e: *mut c_float, b12d: *mut c_float, b12e: *mut c_float, b21d: *mut c_float, b21e: *mut c_float, b22d: *mut c_float, b22e: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_sorbdb_work(matrix_order: c_int, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_float, ldx11: c_int, x12: *mut c_float, ldx12: c_int, x21: *mut c_float, ldx21: c_int, x22: *mut c_float, ldx22: c_int, theta: *mut c_float, phi: *mut c_float, taup1: *mut c_float, taup2: *mut c_float, tauq1: *mut c_float, tauq2: *mut c_float, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_sorcsd_work(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_float, ldx11: c_int, x12: *mut c_float, ldx12: c_int, x21: *mut c_float, ldx21: c_int, x22: *mut c_float, ldx22: c_int, theta: *mut c_float, u1: *mut c_float, ldu1: c_int, u2: *mut c_float, ldu2: c_int, v1t: *mut c_float, ldv1t: c_int, v2t: *mut c_float, ldv2t: c_int, work: *mut c_float, lwork: c_int, iwork: *mut c_int) -> c_int;
    pub fn LAPACKE_ssyconv_work(matrix_order: c_int, uplo: c_char, way: c_char, n: c_int, a: *mut c_float, lda: c_int, ipiv: *const c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_ssyswapr_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_ssytri2_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, ipiv: *const c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_ssytri2x_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_float, lda: c_int, ipiv: *const c_int, work: *mut c_float, nb: c_int) -> c_int;
    pub fn LAPACKE_ssytrs2_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_float, lda: c_int, ipiv: *const c_int, b: *mut c_float, ldb: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_zbbcsd_work(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, m: c_int, p: c_int, q: c_int, theta: *mut c_double, phi: *mut c_double, u1: *mut c_void, ldu1: c_int, u2: *mut c_void, ldu2: c_int, v1t: *mut c_void, ldv1t: c_int, v2t: *mut c_void, ldv2t: c_int, b11d: *mut c_double, b11e: *mut c_double, b12d: *mut c_double, b12e: *mut c_double, b21d: *mut c_double, b21e: *mut c_double, b22d: *mut c_double, b22e: *mut c_double, rwork: *mut c_double, lrwork: c_int) -> c_int;
    pub fn LAPACKE_zheswapr_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_zhetri2_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zhetri2x_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void, nb: c_int) -> c_int;
    pub fn LAPACKE_zhetrs2_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zsyconv_work(matrix_order: c_int, uplo: c_char, way: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zsyswapr_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, i1: c_int, i2: c_int) -> c_int;
    pub fn LAPACKE_zsytri2_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zsytri2x_work(matrix_order: c_int, uplo: c_char, n: c_int, a: *mut c_void, lda: c_int, ipiv: *const c_int, work: *mut c_void, nb: c_int) -> c_int;
    pub fn LAPACKE_zsytrs2_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *const c_void, lda: c_int, ipiv: *const c_int, b: *mut c_void, ldb: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zunbdb_work(matrix_order: c_int, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_void, ldx11: c_int, x12: *mut c_void, ldx12: c_int, x21: *mut c_void, ldx21: c_int, x22: *mut c_void, ldx22: c_int, theta: *mut c_double, phi: *mut c_double, taup1: *mut c_void, taup2: *mut c_void, tauq1: *mut c_void, tauq2: *mut c_void, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zuncsd_work(matrix_order: c_int, jobu1: c_char, jobu2: c_char, jobv1t: c_char, jobv2t: c_char, trans: c_char, signs: c_char, m: c_int, p: c_int, q: c_int, x11: *mut c_void, ldx11: c_int, x12: *mut c_void, ldx12: c_int, x21: *mut c_void, ldx21: c_int, x22: *mut c_void, ldx22: c_int, theta: *mut c_double, u1: *mut c_void, ldu1: c_int, u2: *mut c_void, ldu2: c_int, v1t: *mut c_void, ldv1t: c_int, v2t: *mut c_void, ldv2t: c_int, work: *mut c_void, lwork: c_int, rwork: *mut c_double, lrwork: c_int, iwork: *mut c_int) -> c_int;

    // LAPACK 3.4.0
    pub fn LAPACKE_sgemqrt_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, nb: c_int, v: *const c_float, ldv: c_int, t: *const c_float, ldt: c_int, c: *mut c_float, ldc: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dgemqrt_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, nb: c_int, v: *const c_double, ldv: c_int, t: *const c_double, ldt: c_int, c: *mut c_double, ldc: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cgemqrt_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, nb: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, c: *mut c_void, ldc: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zgemqrt_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, nb: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, c: *mut c_void, ldc: c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_sgeqrt_work(matrix_order: c_int, m: c_int, n: c_int, nb: c_int, a: *mut c_float, lda: c_int, t: *mut c_float, ldt: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dgeqrt_work(matrix_order: c_int, m: c_int, n: c_int, nb: c_int, a: *mut c_double, lda: c_int, t: *mut c_double, ldt: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_cgeqrt_work(matrix_order: c_int, m: c_int, n: c_int, nb: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_zgeqrt_work(matrix_order: c_int, m: c_int, n: c_int, nb: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_sgeqrt2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, t: *mut c_float, ldt: c_int) -> c_int;
    pub fn LAPACKE_dgeqrt2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, t: *mut c_double, ldt: c_int) -> c_int;
    pub fn LAPACKE_cgeqrt2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int) -> c_int;
    pub fn LAPACKE_zgeqrt2_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int) -> c_int;

    pub fn LAPACKE_sgeqrt3_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_float, lda: c_int, t: *mut c_float, ldt: c_int) -> c_int;
    pub fn LAPACKE_dgeqrt3_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_double, lda: c_int, t: *mut c_double, ldt: c_int) -> c_int;
    pub fn LAPACKE_cgeqrt3_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int) -> c_int;
    pub fn LAPACKE_zgeqrt3_work(matrix_order: c_int, m: c_int, n: c_int, a: *mut c_void, lda: c_int, t: *mut c_void, ldt: c_int) -> c_int;

    pub fn LAPACKE_stpmqrt_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, nb: c_int, v: *const c_float, ldv: c_int, t: *const c_float, ldt: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dtpmqrt_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, nb: c_int, v: *const c_double, ldv: c_int, t: *const c_double, ldt: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_ctpmqrt_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, nb: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_ztpmqrt_work(matrix_order: c_int, side: c_char, trans: c_char, m: c_int, n: c_int, k: c_int, l: c_int, nb: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_stpqrt_work(matrix_order: c_int, m: c_int, n: c_int, l: c_int, nb: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, t: *mut c_float, ldt: c_int, work: *mut c_float) -> c_int;
    pub fn LAPACKE_dtpqrt_work(matrix_order: c_int, m: c_int, n: c_int, l: c_int, nb: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, t: *mut c_double, ldt: c_int, work: *mut c_double) -> c_int;
    pub fn LAPACKE_ctpqrt_work(matrix_order: c_int, m: c_int, n: c_int, l: c_int, nb: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, t: *mut c_void, ldt: c_int, work: *mut c_void) -> c_int;
    pub fn LAPACKE_ztpqrt_work(matrix_order: c_int, m: c_int, n: c_int, l: c_int, nb: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, t: *mut c_void, ldt: c_int, work: *mut c_void) -> c_int;

    pub fn LAPACKE_stpqrt2_work(matrix_order: c_int, m: c_int, n: c_int, l: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, t: *mut c_float, ldt: c_int) -> c_int;
    pub fn LAPACKE_dtpqrt2_work(matrix_order: c_int, m: c_int, n: c_int, l: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, t: *mut c_double, ldt: c_int) -> c_int;
    pub fn LAPACKE_ctpqrt2_work(matrix_order: c_int, m: c_int, n: c_int, l: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, t: *mut c_void, ldt: c_int) -> c_int;
    pub fn LAPACKE_ztpqrt2_work(matrix_order: c_int, m: c_int, n: c_int, l: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, t: *mut c_void, ldt: c_int) -> c_int;

    pub fn LAPACKE_stprfb_work(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, l: c_int, v: *const c_float, ldv: c_int, t: *const c_float, ldt: c_int, a: *mut c_float, lda: c_int, b: *mut c_float, ldb: c_int, work: *const c_float, ldwork: c_int) -> c_int;
    pub fn LAPACKE_dtprfb_work(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, l: c_int, v: *const c_double, ldv: c_int, t: *const c_double, ldt: c_int, a: *mut c_double, lda: c_int, b: *mut c_double, ldb: c_int, work: *const c_double, ldwork: c_int) -> c_int;
    pub fn LAPACKE_ctprfb_work(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, l: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, work: *const c_float, ldwork: c_int) -> c_int;
    pub fn LAPACKE_ztprfb_work(matrix_order: c_int, side: c_char, trans: c_char, direct: c_char, storev: c_char, m: c_int, n: c_int, k: c_int, l: c_int, v: *const c_void, ldv: c_int, t: *const c_void, ldt: c_int, a: *mut c_void, lda: c_int, b: *mut c_void, ldb: c_int, work: *const c_double, ldwork: c_int) -> c_int;

    // LAPACK 3.X.X
    pub fn LAPACKE_ssysv_rook_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_float, lda: c_int, ipiv: *mut c_int, b: *mut c_float, ldb: c_int, work: *mut c_float, lwork: c_int) -> c_int;
    pub fn LAPACKE_dsysv_rook_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_double, lda: c_int, ipiv: *mut c_int, b: *mut c_double, ldb: c_int, work: *mut c_double, lwork: c_int) -> c_int;
    pub fn LAPACKE_csysv_rook_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int, work: *mut c_void, lwork: c_int) -> c_int;
    pub fn LAPACKE_zsysv_rook_work(matrix_order: c_int, uplo: c_char, n: c_int, nrhs: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, b: *mut c_void, ldb: c_int, work: *mut c_void, lwork: c_int) -> c_int;

    pub fn LAPACKE_csyr_work(matrix_order: c_int, uplo: c_char, n: c_int, alpha: c_void, x: *const c_void, incx: c_int, a: *mut c_void, lda: c_int) -> c_int;
    pub fn LAPACKE_zsyr_work(matrix_order: c_int, uplo: c_char, n: c_int, alpha: c_void, x: *const c_void, incx: c_int, a: *mut c_void, lda: c_int) -> c_int;
}

// Fortran interface
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
