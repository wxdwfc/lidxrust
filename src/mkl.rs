extern crate libc;

pub use libc::{c_int,c_char,c_double};

pub type lapack_int = c_int;

// reference http://www.netlib.org/lapack/explore-html/dc/d19/a01190_source.html
pub const LAPACK_ROW_MAJOR : c_int = 101;
pub const LAPACK_COL_MAJOR : c_int = 1024;

#[link(name = "mkl_rt")]
extern {
    pub fn LAPACKE_dgels(matrix_layout : c_int,
                     trans : c_char,
                     m : lapack_int,
                     n : lapack_int,
                     nrhs : lapack_int,
                     a : *mut c_double,
                     lda : lapack_int,
                     b : *mut c_double,
                     ldb : lapack_int) -> lapack_int;
}

mod tests {
    use super::*;

    #[test]
    fn test_basic() {

        /*
        A = | 12, 15 |
            | 1,  1  |
        */
        let mut A = Vec::new();
        A.push(12 as c_double);
        A.push(1 as c_double);
        A.push(15 as c_double);
        A.push(1 as c_double);

        /*
        A = | 877, 1096 |
         */
        let mut B = Vec::new();
        B.push(877 as c_double);
        B.push(1096 as c_double);

        // w,b
        let mut P = Vec::new();
        P.push(0 as c_double);
        P.push(0 as c_double);

        let a = unsafe {
            LAPACKE_dgels(LAPACK_ROW_MAJOR, 'N' as c_char,2,2,1,A.as_mut_ptr(),2,B.as_mut_ptr(),1)
        };
        assert_eq!(a,0);
    }
}
