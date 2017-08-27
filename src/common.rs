use libc::{c_char, c_double, c_float, c_int};

use {c_double_complex, c_float_complex};

pub type lapack_int = c_int;

pub type lapack_logical = lapack_int;

pub type lapack_complex_float = c_float_complex;

pub type lapack_complex_double = c_double_complex;

pub type LAPACK_S_SELECT2 = Option<extern "C" fn(*const c_float, *const c_float) -> lapack_logical>;
pub type LAPACK_S_SELECT3 = Option<
    extern "C" fn(*const c_float, *const c_float, *const c_float)
                  -> lapack_logical,
>;

pub type LAPACK_D_SELECT2 = Option<
    extern "C" fn(*const c_double, *const c_double) -> lapack_logical,
>;
pub type LAPACK_D_SELECT3 = Option<
    extern "C" fn(*const c_double,
                  *const c_double,
                  *const c_double)
                  -> lapack_logical,
>;

pub type LAPACK_C_SELECT1 = Option<extern "C" fn(*const lapack_complex_float) -> lapack_logical>;
pub type LAPACK_C_SELECT2 = Option<
    extern "C" fn(*const lapack_complex_float,
                  *const lapack_complex_float)
                  -> lapack_logical,
>;

pub type LAPACK_Z_SELECT1 = Option<extern "C" fn(*const lapack_complex_double) -> lapack_logical>;
pub type LAPACK_Z_SELECT2 = Option<
    extern "C" fn(*const lapack_complex_double,
                  *const lapack_complex_double)
                  -> lapack_logical,
>;
