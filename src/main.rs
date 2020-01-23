mod mkl;
use mkl::{LAPACKE_dgels,LAPACK_ROW_MAJOR,c_double,c_char};

mod tree;
use tree::*;
use tree::node::*;


fn main() {
    println!("Hello, world!");


    //let mut new_root = Box::new(Node::<usize,usize>::Internal(InternalNode::<usize,usize>::new()));

    let mut t = BTree::<usize,usize>::new();
    for i in 0..17 {
        println!("insert {} start",i);
        t.insert(i, i);
        println!("insert {} done",i);
    }

    for i in 0..17 {
        println!("start get {}",i);
        let v = t.get(i).unwrap();
        println!("get {}: {}",i, v);
    }
    /*
    let mut t = BTree::<usize,usize>::new();
    for i in 0..17 {
        println!("insert {} start",i);
        t.insert(i, i);
        println!("insert {} done",i);
    }*/
}
