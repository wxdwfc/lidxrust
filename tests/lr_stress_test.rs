extern crate lidx;

use lidx::lr::LRKV;

mod ycsb_generator;

use lidx::KVInterface;

#[test]
fn test_lr_stress() {
    let mut t = LRKV::<u64,u64>::new();

    let mut ycsb_gen = ycsb_generator::YCSBKeyGen::new(1024);

    for k in ycsb_gen.into_hash_iter() {
        t.insert(k, k + 73);
    }
    t.retrain();

    ycsb_gen.reset();

    for k in ycsb_gen.into_hash_iter() {
        let v = t.get(&k);
        assert_ne!(v,None);
        assert_eq!(v.unwrap(), k + 73);
    }
}
