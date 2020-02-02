pub mod trainer;

// the predictort
#[derive(Clone,Copy,Debug)]
struct LRPredictor {
    w : f64,
    b : f64,
    err_min : i64,
    err_max : i64,
}

use std::cmp;
use crate::KVPair;
use crate::Trainiable;

impl LRPredictor {
    pub fn new() -> Self {
        LRPredictor { w : 0_f64, b : 0_f64, err_min : std::i64::MAX, err_max : std::i64::MIN}
    }

    pub fn predict_temp(&self,x : &f64) -> f64 {
        self.w * (*x) + self.b
    }

    pub fn predict_temp_to_i64(&self, x : &f64) -> i64 {
        self.predict_temp(x).ceil() as i64
    }
}


use crate::LidxKV;

pub struct LRKV<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
{
    sorted_array : Vec<KVPair<K,V>>,
    index : LRPredictor,
}

impl<K,V> LRKV<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
{
    pub fn new() -> Self {
        LRKV { sorted_array : Vec::new(), index : LRPredictor::new() }
    }

    pub fn retrain(&mut self) {
        self.sorted_array.sort();
        self.index.train(&self.sorted_array);
    }
}

use crate::LidxKVTrainwArray;

impl<K> LidxKVTrainwArray<K> for LRPredictor
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable
{
    /// assumption: array is sorted
    fn train<V : Copy>(&mut self, array : &Vec<KVPair<K,V>>) {

        // reset
        *self = LRPredictor::new();

        let mut training_set = trainer::Trainer::new();
        for (i,kv) in array.iter().enumerate() {
            training_set.add_one(kv.key.convert_to_cdouble(), i.convert_to_cdouble());
        }

        let (w, b) = training_set.train_optimal();
        self.w = w;
        self.b = b;

        // then we calculate the min-max
        for (real_pos,kv) in array.iter().enumerate() {
            let predicted_pos = self.predict_temp_to_i64(&kv.key.convert_to_cdouble());
            self.err_min = cmp::min(self.err_min, real_pos as i64 - predicted_pos);
            self.err_max = cmp::max(self.err_max, real_pos as i64 - predicted_pos);
        }
    }
}

impl<K> LidxKV<K,usize> for LRPredictor
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable
{
    fn predict(&self, k : &K) -> (usize, usize) {
        let mid = self.predict_temp_to_i64(&k.convert_to_cdouble());
        let start = cmp::max(mid + self.err_min, 0);
        let end   = mid + self.err_max;
        (start as usize, end as usize)
    }
}

// implement a minimal KV interface for testing
use crate::KVInterface;

impl<K,V> KVInterface<K,V> for LRKV<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy + std::fmt::Debug
{
    fn get(&self, key : &K) -> Option<V> {
        self.get_as_ref(key).map(|v| *v)
    }

    fn get_as_ref(&self, key : &K) -> Option<&V> {
        let (start,end) = self.index.predict(&key.convert_to_cdouble());
        let aug_end = cmp::min(end + 1, self.sorted_array.len());

        for idx in start..aug_end {
            if self.sorted_array[idx as usize].key == *key {
                return Some(& self.sorted_array[idx as usize].value);
            }
        }
        None
    }

    fn insert(&mut self, key : K, value : V) {
        self.sorted_array.push(KVPair::new(key, value));
    }
}

// implement a minimal KV for testing
mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn basic_lr() {
        let mut t = LRKV::<usize,usize>::new();

        let test_num = 40960;
        println!("before insert");
        for i in 0..test_num {
            t.insert(i, i + 73);
        }
        println!("insert done");

        t.retrain();
        println!("retrain done {:?}", t.index);

        for i in 0..test_num {
            //println!("try get {}",i);
            let v = t.get(&i);
            assert_ne!(v,None);
            assert_eq!(v.unwrap(), i + 73);
            //println!("try get {} done",i);
        }
    }
}
