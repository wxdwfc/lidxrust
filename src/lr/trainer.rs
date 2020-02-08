/*
  The basic linear regression trainer which returns w, b from the training-set,
  where target = w * feature + b
 */

extern crate libc;
pub use libc::{c_double};

pub struct Trainer {
    pub key_set : Vec<c_double>,
    pub value_set : Vec<c_double>,
}

use crate::Trainiable;
use crate::mkl::*;

impl Trainer {
    pub fn new() -> Self {
        Trainer {
            key_set : Vec::new(), value_set : Vec::new()
        }
    }

    pub fn add_one<K,V>(&mut self, k : K, v : V)
        where K : Trainiable, V : Trainiable
    {
        if self.key_set.len() > 0 {
            assert!(self.key_set[self.key_set.len() - 1] < k.convert_to_cdouble());
        }
        self.key_set.push(k.convert_to_cdouble());
        self.value_set.push(v.convert_to_cdouble());
    }

    pub fn clear(&mut self) {
        self.key_set.clear();
        self.value_set.clear();
    }

    pub fn at(&self, idx : usize) -> (c_double,c_double) // K,V
    {
        (self.key_set[idx], self.value_set[idx])
    }

    pub fn train_optimal(&mut self) -> (c_double, c_double) {

        let mut flattered_matrix = Vec::new();
        for i in 0..self.key_set.len() {
            flattered_matrix.push(self.key_set[i]);
            flattered_matrix.push(1 as c_double);
        }

        unsafe {
            let ret = LAPACKE_dgels(LAPACK_ROW_MAJOR,
                                    'N' as c_char,
                                    self.key_set.len() as i32,
                                    2,
                                    1,
                                    flattered_matrix.as_mut_ptr(),
                                    2,
                                    self.value_set.as_mut_ptr(),
                                    1);
            assert!(ret == 0);

        };
        let res = (self.value_set[0], self.value_set[1]);
        self.clear();
        res
    }
}

mod tests {

    #[cfg(test)]
    fn approx_equal(a: f64, b: f64, p : f64) -> bool {
        (a - b).abs() <= p
    }

    #[cfg(test)]
    use super::*;

    #[test]
    pub fn basic() {
        let mut t = Trainer::new();
        t.add_one(0, 0);
    }
    #[test]
    pub fn test_trainer() {
        let test_w = 73_f64;
        let test_b = 11212_f64;

        let mut t = Trainer::new();

        for i in 0..100 {
            let x = i as f64;
            t.add_one(x, x * test_w + test_b);
        }

        let (w,b) = t.train_optimal();
        assert_eq!(true,approx_equal(w, test_w, 0.005_f64));
        assert_eq!(true,approx_equal(b, test_b, 0.005_f64));
    }
}
