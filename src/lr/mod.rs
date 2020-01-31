pub mod trainer;

// the predictort
#[derive(Clone,Copy)]
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

#[derive(Clone,Copy)]
struct LRIndex
{
    lr : Option<LRPredictor>,

    error_min : i64,
    error_max : i64,
}


use std::cmp;
use crate::KVPair;
use crate::Trainiable;

impl LRIndex {
    pub fn new(lr : LRPredictor) -> Self {
        LRIndex { lr : Some(lr), error_min : 0, error_max : 0 }
    }

    pub fn calculate_min_max<K,V>(&mut self, sorted_array : &Vec<KVPair<K,V>>)
    where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
    {
        self.error_min = std::i64::MAX;
        self.error_max = std::i64::MIN;

        for (real_pos,kv) in sorted_array.iter().enumerate() {
            let predicted_pos = self.lr.expect("lr should be trained to calculate min_max")
                .predict(&kv.key.convert_to_cdouble()) as i64;
            self.error_min = cmp::min(self.error_min, real_pos as i64 - predicted_pos);
            self.error_max = cmp::max(self.error_max, real_pos as i64 - predicted_pos);
        }
    }
}

use crate::LidxKV;

impl LidxKV<f64, i64> for LRIndex
{
    fn predict(&self, k : &f64) -> (i64,i64) {
        let predict_pos = self.lr.as_ref().expect("lr should be trained to use").predict(k);
        let start_pos = (predict_pos as i64) + self.error_min;
        let end_pos   = (predict_pos as i64) + self.error_max;
        (start_pos,end_pos)
    }
}

struct LRKV<K,V>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable, V : Copy
{
    sorted_array : Vec<KVPair<K,V>>,
    index : LRIndex,
}

// implement a minimal KV for testing
mod tests {

}
