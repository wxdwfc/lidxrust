pub mod mkl;

pub mod key;

pub mod tree;
pub mod lr;
pub mod rmi;

pub struct KVPair<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
{
    pub key : K,
    pub value : V,
}

impl<K,V> KVPair<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable , V : Copy
{
    pub fn new(k : K, v : V) -> Self {
        KVPair { key : k, value : v }
    }
}

use std::cmp::Ordering;

impl<K,V> Ord for KVPair<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.partial_cmp(&other.key).unwrap()
    }
}

impl<K,V> Eq for KVPair<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
{
}

impl<K,V> PartialOrd for KVPair<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
{
    fn partial_cmp(&self, other: &KVPair<K,V>) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K,V> PartialEq for KVPair<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
{
    fn eq(&self, other: &KVPair<K,V>) -> bool {
        self.key.eq(&other.key)
    }
}

// KV interfaces
pub trait KVInterface<K,V>
where V : Copy
{
    fn insert(&mut self, k : K, v : V);
    fn get(&self, k : &K) -> Option<V>;

    fn get_as_ref(&self, k : &K) -> Option<&V>;
}

// Learned index trait
// a learned index model must implement predict(K) -> [start_addr, end_addr]
// the address type is Addr
pub trait LidxKV<K,Addr> {
    fn predict(&self, k : &K) -> (Addr,Addr);
}

/// Trait for training
pub trait LidxKVTrainwArray<K>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable
{
    fn train<V : Copy>(&mut self, array : &Vec<KVPair<K,V>>);
}

// some minor helpers

pub use libc::{c_double};
extern crate libc;

pub trait Trainiable {
    fn convert_to_cdouble(&self) -> c_double;
}

impl Trainiable for usize {
    fn convert_to_cdouble(&self) -> c_double {
        *self as c_double
    }
}

impl Trainiable for f64 {
    fn convert_to_cdouble(&self) -> c_double {
        *self as c_double
    }
}
