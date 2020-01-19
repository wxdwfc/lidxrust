mod mkl;
use mkl::{LAPACKE_dgels,LAPACK_ROW_MAJOR,c_double,c_char};

fn main() {
    println!("Hello, world!");

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
    println!("{}",a);
}
