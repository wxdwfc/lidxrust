use crate::KVInterface;
use crate::Trainiable;

use crate::LidxKV;

use std::marker::PhantomData;

struct BasicRMIIndex<K,FirstLayerLidx, SecondLayerLidx>
where K : PartialOrd + Copy + std::fmt::Debug + Trainiable,
FirstLayerLidx : Sized + LidxKV<K,usize>, SecondLayerLidx : Sized + LidxKV<K,usize>
{
    first_layer : FirstLayerLidx,
    second_layer : Vec<SecondLayerLidx>,
    phantom: PhantomData<K>,
}
