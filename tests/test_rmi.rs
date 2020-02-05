extern crate lidx;

use lidx::rmi::BasicRMIIndex;
use lidx::lr::LRPredictor;

use lidx::KVPair;
use lidx::LidxKVTrainwArray;
use lidx::LidxKV;

mod ycsb_generator;

fn find_value(key : u64, start : usize, end : usize, array : &Vec<KVPair<u64,u64>>) -> Option<u64>
{
    for i in start..end+1 {
        if array[i].key == key {
            return Some(array[i].value);
        }
    }
    return None;
}

#[test]
fn test_rmi_stress() {
    let first_layer = LRPredictor::new();
    let mut second_layer : Vec<LRPredictor> = Vec::new();

    for _ in 0..64 {
        second_layer.push(LRPredictor::new());
    }
    println!("rmi init before");
    let mut rmi : BasicRMIIndex<u64,LRPredictor, LRPredictor> = BasicRMIIndex::new(first_layer,second_layer);
    println!("rmi init done");

    // prepare the test data
    let mut ycsb_gen = ycsb_generator::YCSBKeyGen::new(1024 * 4);
    let mut test_data : Vec<KVPair<u64,u64>> = Vec::new();
    for k in ycsb_gen.into_iter() {
        test_data.push(KVPair::new(k, k + 73));
    }

    // now we train the rmi
    rmi.train(&test_data);

    // now we check the predict
    ycsb_gen.reset();
    let mut count = 0;
    for k in ycsb_gen.into_iter() {
        let (s,e) = rmi.predict(&k);
        let val = find_value(k,s,e,&test_data).unwrap();
        assert_eq!(val,k + 73);
        count += 1;
    }
    assert_eq!(count, test_data.len());
}
