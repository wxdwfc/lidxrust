extern crate lidx;

use lidx::tree::BTree;
use lidx::KVInterface;

extern crate rand;
#[allow(unused_imports)]
use rand::{seq::SliceRandom, SeedableRng,thread_rng}; // 0.6.5

#[test]
fn test_tree_stress() {
    let mut key_set : Vec<usize> = Vec::new();      //
    let key_num = 1000000;                          //
    //
    for i in 0..key_num {                           //
        key_set.push(i);                            //
    }                                               //
    //
    //let mut rng = thread_rng();                     //
    //let seed = [0; 32];
    //let mut rng : StdRng = SeedableRng::from_seed(seed);
    let mut rng = thread_rng();
    //
    for _ in 0..5 {                                 //
        //key_set.as_mut_slice().shuffle(&mut rng);   //
        //rng.shuffle(key_set.as_mut_slice());
        key_set.shuffle(&mut rng);
        let mut t = BTree::<usize,usize>::new();    //
        //
        for k in &key_set {                         //
            t.insert(*k, k + 73);                   //
        }                                           //
        //
        for k in &key_set {                         //
            assert_eq!(t.get(k).unwrap(), k + 73); //
        }                                           //
        //
    }                                               //
}
