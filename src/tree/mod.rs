pub mod node;

use node::*;

pub struct BTree<K : PartialOrd + std::fmt::Debug ,V> {
    root  : Option<Box<node::Node<K,V>>>,
}

impl<K : PartialOrd + std::fmt::Debug ,V> BTree<K,V> {
    pub fn new() -> Self {
        BTree { root : None, }
    }
}

// trait implementations
use crate::KV;

impl<K,V> KV<K,V> for BTree<K,V>
where K : PartialOrd + Copy + std::fmt::Debug, V : Copy + std::fmt::Debug
{
    fn get(&self, key : &K) -> Option<V> {
        self.find_leaf_page(*key).map(|n| {
            match **n {
                Node::Internal(_) => unreachable!(),
                Node::Leaf(ref l) => {
                    l.get(key)
                }
            }
        }).and_then(|n| n)
    }

    fn get_as_ref(&self, key : &K) -> Option<&V> {
        self.find_leaf_page(*key).map(|n| {
            match **n {
                Node::Internal(_) => unreachable!(),
                Node::Leaf(ref l) => {
                    l.get_as_ref(key)
                }
            }
        }).and_then(|n| n)
    }

    fn insert(&mut self, key : K, value : V) {
        // the B+Tree is empty
        {
            match &self.root {
                None => { self.root = Some(Box::new(Node::Leaf(LeafNode::new()))); },
                _ => {},
            }
        }

        // insert to root
        let new_node = self.root.as_mut().unwrap().insert(key,value);

        // update root if possible
        new_node.map(|n| {
            self.root = Some(Box::new(Node::Internal(InternalNode::new_from(
                n.get_up_key(), self.root.take().unwrap(),n))));
        });

        // end one-layer case
        // end insert
    }
}

// get methods
impl <K,V> BTree<K,V>
where K : PartialOrd + Copy + std::fmt::Debug, V : Copy + std::fmt::Debug
{
    pub fn find_leaf_page(&self, key : K) -> Option<&Box<node::Node<K,V>>> {

        let mut cur_node : &Option<Box<node::Node<K,V>>> = &self.root;

        loop {
            cur_node = match cur_node.as_ref() {
                Some(n) => {
                    match **n {
                        Node::Internal(ref i) => {
                            i.find_link(&key)
                        },
                        Node::Leaf(_) => {
                            return cur_node.as_ref();
                        }
                    }
                },
                None => { break; }
            }
        }
        None
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn basic() {
        let mut t = BTree::<usize,usize>::new();

        let test_num = 40960;

        for i in 0..test_num {
            //println!("insert {} start",i);
            t.insert(i, i + 73);
            //println!("insert {} done",i);
        }

        for i in 0..test_num {
            println!("try get {}",i);
            let v = t.get(&i);
            assert_ne!(v,None);
            assert_eq!(v.unwrap(), i + 73);
            println!("try get {} done",i);
        }

        //assert_eq!(0,-1);
    }

    extern crate rand;
    #[allow(unused_imports)]
    use rand::{seq::SliceRandom, SeedableRng,thread_rng}; // 0.6.5

    #[test]
    fn stress() {

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
}
