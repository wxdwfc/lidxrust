extern crate lidx;

use lidx::lr::LRKV;

#[test]
fn test_lr_stress() {
    let mut t = LRKV::<usize,usize>::new();
    assert_eq!(-1,0);
}
