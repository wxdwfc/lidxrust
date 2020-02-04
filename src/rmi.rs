//use crate::KVInterface;
use crate::Trainiable;

use crate::LidxKV;
use crate::LidxKVTrainwArray;
use crate::LidxKVTrainwAddr;

use std::marker::PhantomData;

struct BasicRMIIndex<K,FirstLayerLidx, SecondLayerLidx>
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
FirstLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K>,
SecondLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K> + LidxKVTrainwAddr<K,usize>,
{
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn find_second_layer(&self,k : &K) -> usize {
        assert!(self.second_layer.len() > 0);
        let mut pos = self.first_layer.predict_point(k);
        pos = std::cmp::max(0,pos) % self.total_keys;
        std::cmp::min(pos, self.second_layer.len() - 1)
    }
}

use crate::KVPair;

impl<K,FirstLayerLidx, SecondLayerLidx> LidxKVTrainwArray<K> for BasicRMIIndex<K,FirstLayerLidx,SecondLayerLidx>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable + std::cmp::Eq,
FirstLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K>,
SecondLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K> + LidxKVTrainwAddr<K,usize>,
{
    fn train<V : Copy>(&mut self, array : &Vec<KVPair<K,V>>) {
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
            self.second_layer[i].train_w_addr(&partitioned_t_set[i]);
        }
        // done
    }
}
