pub mod basic_type;

pub mod mkl;

pub mod key;

pub mod tree;
pub mod lr;

// KV interfaces
pub trait KV<K,V>
where V : Copy
{

    fn insert(&mut self, k : K, v : V);
    fn get(&self, k : &K) -> Option<V>;

    fn get_as_ref(&self, k : &K) -> Option<&V>;
}

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
