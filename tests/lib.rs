extern crate lapack_sys;

#[cfg(not(feature = "accelerate"))]
#[test]
fn link_c() {
    use lapack_sys::c as ffi;

    let matrix_layout = ffi::LAPACK_COL_MAJOR;
    let jobz = b'V' as i8;
    let uplo = b'U' as i8;
    let n = 1;
    let mut a = vec![0.0];
    let lda = 1;
    let mut w = vec![0.0];

    unsafe {
        ffi::LAPACKE_dsyev(matrix_layout, jobz, uplo, n, a.as_mut_ptr(), lda, w.as_mut_ptr());
    }
}

#[test]
fn link_fortran() {
    use lapack_sys::fortran as ffi;

    let jobz = b'V' as i8;
    let uplo = b'U' as i8;
    let n = 1;
    let mut a = vec![0.0];
    let lda = 1;
    let mut w = vec![0.0];
    let mut work = vec![0.0, 0.0];
    let lwork = 2;
    let mut flag = 0;

    unsafe {
        ffi::dsyev_(&jobz, &uplo, &n, a.as_mut_ptr(), &lda, w.as_mut_ptr(), work.as_mut_ptr(),
                    &lwork, &mut flag);
    }
}
