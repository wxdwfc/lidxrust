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

use crate::basic_type::Trainiable;

// sort data and data1 according to data
pub fn sort_two_array<T: PartialOrd,T2 : PartialOrd>(data: &mut [T], data1 : &mut [T2]) {
    assert!(data.len() == data1.len());
    if data.len() < 2 { return; }

    let mut lpos = 1;
    let mut rpos = data.len();
    /* Invariant: pivot is data[0]; everything with index (0,lpos) is <= pivot;
    [rpos,len) is >= pivot; lpos < rpos */
    loop {
        if lpos != rpos {
            if data[lpos] > data[0] {
                rpos -= 1;
                data.swap(lpos, rpos);
                data1.swap(lpos,rpos);
            } else {
                lpos += 1;
            }
        } else {
            break;
        }
    }

    // Once our cursors met, we need to put the pivot in the right place.
    data.swap(0, lpos-1);
    data1.swap(0, lpos-1);

    let (part1, part2) = data.split_at_mut(lpos);
    let (part11, part21) = data1.split_at_mut(lpos);
    sort_two_array(&mut part1[..lpos-1], &mut part11[..lpos-1]);                                     /*@*/
    sort_two_array(&mut part2[..], &mut part21[..]);                                                    /*@*/
}

impl Trainer {
    pub fn new() -> Self {
        Trainer { key_set : Vec::new(), value_set : Vec::new() }
    }

    pub fn add_one<K,V>(&mut self, k : K, v : V)
        where K : Trainiable, V : Trainiable
    {
        self.key_set.push(k.convert_to_cdouble());
        self.value_set.push(v.convert_to_cdouble());
    }

    pub fn clear(&mut self) {
        self.key_set.clear();
        self.value_set.clear();
    }

    pub fn sort(&mut self) {
        sort_two_array(&mut self.key_set[..], &mut self.value_set[..]);
    }

    pub fn at(&self, idx : usize) -> (c_double,c_double) // K,V
    {
        (self.key_set[idx], self.value_set[idx])
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn basic() {
        let mut t = Trainer::new();
        t.add_one(0, 0);
    }

    #[test]
    pub fn test_sort() {
        let mut t = Trainer::new();
        t.add_one(0, 0);
        t.add_one(12,12);
        t.add_one(4,4);
        t.add_one(7, 7);

        t.sort();

        let mut prev = 0_f64;
        for i in 0..4 {
            let (k0,v0) = t.at(i);
            assert_eq!(k0,v0);
            assert_eq!(true, k0 >= prev);
            prev = k0;
        }

    }
}
