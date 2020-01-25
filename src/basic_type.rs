extern crate libc;

pub use libc::{c_double};


pub trait Trainiable {
    fn convert_to_cdouble(&self) -> c_double;
}
