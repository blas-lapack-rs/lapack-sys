extern crate liblapack_sys as raw;

#[test]
fn linking() {
    let jobz = b'V' as i8;
    let uplo = b'U' as i8;
    let n = 1;
    let mut a = vec![0.0];
    let lda = 1;
    let mut w = vec![0.0];
    let mut work = vec![0.0, 0.0];
    let lwork = 2;

    unsafe {
        raw::LAPACKE_dsyev_work(raw::LAPACK_ROW_MAJOR, jobz, uplo, n, a.as_mut_ptr(), lda, w.as_mut_ptr(),
                    work.as_mut_ptr(), lwork);
    }
}
