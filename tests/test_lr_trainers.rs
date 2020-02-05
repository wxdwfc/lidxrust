use lidx::lr::LRPredictor;

use lidx::LidxKVTrainwAddr;
use lidx::LidxKVTrainwArray;

use lidx::KVPair;

// we test whether the above two training trait, yields *the same* linear regression model
#[test]
fn basic_trainers() {
    let mut lr = LRPredictor::new();
    let mut lr1 = LRPredictor::new();

    // construct two training-set with different types
    let mut training_set0 : Vec<KVPair<usize,usize>> = Vec::new();
    let mut training_set1 : Vec<(usize,usize)> = Vec::new();

    training_set0.push(KVPair::new(0,0));
    training_set0.push(KVPair::new(12,0));
    training_set0.push(KVPair::new(73,0));
    training_set0.push(KVPair::new(1024,0));
    training_set0.push(KVPair::new(2048,0));
    training_set0.push(KVPair::new(12124,0));

    for (idx,kv) in training_set0.iter().enumerate() {
        training_set1.push((kv.key, idx));
    }

    lr.train(&training_set0);
    lr1.train_w_addr(&training_set1);

    assert_eq!(lr.get_w(),lr1.get_w());
    assert_eq!(lr.get_b(),lr1.get_b());
    assert_eq!(lr.get_error_min(),lr1.get_error_min());
    assert_eq!(lr.get_error_max(),lr1.get_error_max());
}
