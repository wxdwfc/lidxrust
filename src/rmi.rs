//use crate::KVInterface;
use crate::Trainiable;

use crate::LidxKV;
use crate::LidxKVTrainwArray;
use crate::LidxKVTrainwAddr;

use std::marker::PhantomData;

pub struct BasicRMIIndex<K,FirstLayerLidx, SecondLayerLidx>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable,
FirstLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K>,
SecondLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K> + LidxKVTrainwAddr<K,usize>,
{
    first_layer : FirstLayerLidx,
    second_layer : Vec<SecondLayerLidx>,
    total_keys : usize,
    phantom: PhantomData<K>,
}

impl<K,FirstLayerLidx, SecondLayerLidx> BasicRMIIndex<K,FirstLayerLidx, SecondLayerLidx>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable,
FirstLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K> + std::fmt::Debug,
SecondLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K> + LidxKVTrainwAddr<K,usize>,
{
    pub fn new(f : FirstLayerLidx, sec : Vec<SecondLayerLidx>) -> Self {
        Self { first_layer : f, second_layer : sec, total_keys : 0, phantom : PhantomData }
    }

    pub fn find_second_layer(&self,k : &K) -> usize {
        // pre-conditions
        assert!(self.second_layer.len() > 0);
        assert!(self.total_keys > 0);

        let mut pos = self.first_layer.predict_point(k);
        pos = std::cmp::min(std::cmp::max(0,pos), self.total_keys - 1);

        let ratio = (pos as f64) / ((self.total_keys - 1) as f64);
        std::cmp::min((ratio * self.second_layer.len() as f64).ceil() as usize,
                      self.second_layer.len() - 1)
    }
}

use crate::KVPair;

impl<K,FirstLayerLidx, SecondLayerLidx> LidxKVTrainwArray<K> for BasicRMIIndex<K,FirstLayerLidx,SecondLayerLidx>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable + std::cmp::Eq,
FirstLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K> + std::fmt::Debug,
SecondLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K> + LidxKVTrainwAddr<K,usize>,
{
    fn train<V : Copy>(&mut self, array : &Vec<KVPair<K,V>>) {

        self.total_keys = array.len();

        // 1. train the first layers
        self.first_layer.train::<V>(array);

        // 2. assign the second layer training-set
        let mut partitioned_t_set : Vec<Vec<(K,usize)>> = Vec::new();

        // 2.0 init the training-set of each rmi leaf node
        for _ in 0..self.second_layer.len() {
            partitioned_t_set.push(Vec::new());
        }

        // 2.1 fill the training-set
        for (i,kv) in array.iter().enumerate() {
            partitioned_t_set[self.find_second_layer(&kv.key)].push((kv.key,i));
        }

        // 2.2 train each training set
        for i in 0..self.second_layer.len() {
            if partitioned_t_set[i].len() > 0 {
                //println!("train {}, {}",i,partitioned_t_set[i].len());
            }
            self.second_layer[i].train_w_addr(&partitioned_t_set[i]);
        }
        // done
    }
}

impl<K,FirstLayerLidx, SecondLayerLidx> LidxKV<K,usize> for BasicRMIIndex<K,FirstLayerLidx,SecondLayerLidx>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable + std::cmp::Eq,
FirstLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K> + std::fmt::Debug,
SecondLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K> + LidxKVTrainwAddr<K,usize>,
{
    fn predict(&self, k : &K) -> (usize,usize) {
        let model_idx = self.find_second_layer(k);
        self.second_layer[model_idx].predict(k)
    }

    fn predict_point(&self, k : &K) -> usize {
        let model_idx = self.find_second_layer(k);
        self.second_layer[model_idx].predict_point(k)
    }
}
