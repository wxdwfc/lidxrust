pub mod basic_type;

pub mod mkl;

pub mod key;

pub mod tree;
pub mod lr;

extern crate libc;
pub use libc::{c_double};

impl basic_type::Trainiable for usize {
    fn convert_to_cdouble(&self) -> c_double {
        *self as c_double
    }
}


impl basic_type::Trainiable for f64 {
    fn convert_to_cdouble(&self) -> c_double {
        *self as c_double
    }
}
