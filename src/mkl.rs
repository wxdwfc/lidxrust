extern crate libc;

pub use libc::{c_int,c_char,c_double};

#[allow(non_camel_case_types)]
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
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_basic() {

        /*
        a = | 12, 15 |
            | 1,  1  |
        */
        let mut a = Vec::new();
        a.push(12 as c_double);
        a.push(1 as c_double);
        a.push(15 as c_double);
        a.push(1 as c_double);

        /*
        b = | 877, 1096 |
         */
        let mut b = Vec::new();
        b.push(877 as c_double);
        b.push(1096 as c_double);

        let ret = unsafe {
            LAPACKE_dgels(LAPACK_ROW_MAJOR, 'N' as c_char,2,2,1,a.as_mut_ptr(),2,b.as_mut_ptr(),1)
        };
        assert_eq!(ret,0);
    }
}
