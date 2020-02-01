pub mod trainer;

// the predictort
#[derive(Clone,Copy,Debug)]
struct LRPredictor {
    w : f64,
    b : f64,
}

impl LRPredictor {
    pub fn new(t : &mut trainer::Trainer) -> Self {
        let (w,b) = t.train_optimal();
        LRPredictor { w : w, b : b}
    }

    pub fn predict(&self,x : &f64) -> f64 {
        self.w * (*x) + self.b
    }
}

#[derive(Clone,Copy,Debug)]
struct LRIndex
{
    lr : Option<LRPredictor>,

    err_min : i64,
    err_max : i64,
}

use std::cmp;
use crate::KVPair;
use crate::Trainiable;

impl LRIndex {
    pub fn new(lr : LRPredictor) -> Self {
        LRIndex { lr : Some(lr), err_min : 0, err_max : 0 }
    }

    pub fn calculate_min_max<K,V>(&mut self, sorted_array : &Vec<KVPair<K,V>>)
    where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
    {
        self.err_min = std::i64::MAX;
        self.err_max = std::i64::MIN;

        for (real_pos,kv) in sorted_array.iter().enumerate() {
            let predicted_pos = self.lr.expect("lr should be trained to calculate min_max")
                .predict(&kv.key.convert_to_cdouble()).ceil() as i64;
            self.err_min = cmp::min(self.err_min, real_pos as i64 - predicted_pos);
            self.err_max = cmp::max(self.err_max, real_pos as i64 - predicted_pos);
        }
    }
}

use crate::LidxKV;

impl LidxKV<f64, i64> for LRIndex
{
    fn predict(&self, k : &f64) -> (i64,i64) {
        let predict_pos = self.lr.as_ref().expect("lr should be trained to use").predict(k);
        let start_pos = (predict_pos as i64) + self.err_min;
        let end_pos   = (predict_pos as i64) + self.err_max;
        (start_pos,end_pos)
    }
}

struct LRKV<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
{
    sorted_array : Vec<KVPair<K,V>>,
    index : Option<LRIndex>,
}

impl<K,V> LRKV<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
{
    pub fn new() -> Self {
        LRKV { sorted_array : Vec::new(), index : None }
    }

    pub fn retrain(&mut self) {
        let mut training_set = trainer::Trainer::new();
        self.sorted_array.sort();

        for (i,kv) in self.sorted_array.iter().enumerate() {
            training_set.add_one(kv.key.convert_to_cdouble(), i.convert_to_cdouble());
        }

        // reatrain the model, replace the old one
        self.index.take();
        self.index = Some(LRIndex::new(LRPredictor::new(&mut training_set)));

        // calculate the min-max
        self.index.unwrap().calculate_min_max(&self.sorted_array);

        println!("retrain done {:?}", self.index.unwrap());
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
        let (mut start,mut end) = self.index.expect("lr should be trained to predict").predict(&key.convert_to_cdouble());
        start = cmp::max(start, 0);
        end = cmp::min(end, self.sorted_array.len() as i64 - 1);

        //println!("search range {},{}", start,end);

        for idx in start..end+1 {
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

        for i in 0..test_num {
            //println!("try get {}",i);
            let v = t.get(&i);
            assert_ne!(v,None);
            assert_eq!(v.unwrap(), i + 73);
            //println!("try get {} done",i);
        }
    }
}
