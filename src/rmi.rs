//use crate::KVInterface;
use crate::Trainiable;

use crate::LidxKV;
use crate::LidxKVTrainwArray;

use std::marker::PhantomData;

struct BasicRMIIndex<K,FirstLayerLidx, SecondLayerLidx>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable,
FirstLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K>,
SecondLayerLidx : Sized + LidxKV<K,usize> + LidxKVTrainwArray<K>,
{
    first_layer : FirstLayerLidx,
    second_layer : Vec<SecondLayerLidx>,
    phantom: PhantomData<K>,
}
