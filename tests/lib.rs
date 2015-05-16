extern crate lapack_sys as raw;

#[test]
fn link() {
    let mut jobz = b'V' as i8;
    let mut uplo = b'U' as i8;
    let mut n = 1;
    let mut a = vec![0.0];
    let mut lda = 1;
    let mut w = vec![0.0];
    let mut work = vec![0.0, 0.0];
    let mut lwork = 2;
    let mut flag = 0;

    unsafe {
        raw::dsyev_(&mut jobz as *mut _, &mut uplo as *mut _, &mut n as *mut _, a.as_mut_ptr(),
                    &mut lda as *mut _, w.as_mut_ptr(), work.as_mut_ptr(), &mut lwork as *mut _,
                    &mut flag as *mut _);
    }
}
