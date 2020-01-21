use std::mem::{self, MaybeUninit};
use std::rc::Rc;

pub const MAX_KEYS : usize = 16; // number of keys per node

fn range_assign<T : Copy> (src : & [T;MAX_KEYS], target : &mut [T;MAX_KEYS], start_src : usize, start_target : usize, num : usize) {
    for idx in 0 .. num {
        target[start_target + idx] = src[start_src + idx];
    }
}

#[derive(Debug)]
pub struct LeafNode <K,V> {
    num_keys : usize,
    keys : [K; MAX_KEYS],
    values : [V; MAX_KEYS],
}

// methods for leaf
impl <K,V> LeafNode<K,V>
where K : PartialOrd + Copy, V : Copy
{
    pub fn get_ref(&mut self, k :& K) -> Option<&mut V> {
        let (find,idx) = self.find(k);
        if find {
            return Some (&mut self.values[idx]);
        }
        return None;
    }
}

impl <K,V> LeafNode<K,V>
where K : PartialOrd + Copy, V : Copy
{
    pub fn new() -> Self {
        unsafe {
            LeafNode { num_keys : 0,
                       keys : MaybeUninit::uninit().assume_init(),
                       values : MaybeUninit::uninit().assume_init(),
            }
        }
    }

    pub fn empty(&self) -> bool {
        self.num_keys == 0
    }

    pub fn num_keys(&self) -> usize {
        self.num_keys
    }

    pub fn get(&self, k : &K) -> Option<V> {
        let (find, idx) = self.find(k);
        if find {
            return Some(self.values[idx]);
        }
        None
    }

    // \ret: whether find (bool), the appropriate index (usize)
    pub fn find(&self, k : & K) -> (bool,usize) {
        let mut idx = 0;
        while (idx < self.num_keys && self.keys[idx] < *k) {
            idx += 1;
        }

        // the key has already been there
        if (idx < self.num_keys && *k == self.keys[idx]) {
            return (true, idx);
        }
        return (false, idx);

    }

    // \ret: whether find (bool), the newly splitted leaf
    pub fn insert(&mut self, k : K, v : V) -> (bool, Option<Box<LeafNode<K,V>>>) {
        let (find, mut idx) = self.find(&k);
        if find {
            return (true, None);
        }

        // insert
        if self.num_keys == MAX_KEYS {
            // split
            let mut new_sib = Box::new(LeafNode::new());

            let threshold = (MAX_KEYS + 1) / 2;

            new_sib.num_keys = self.num_keys - threshold;
            range_assign(&self.keys, &mut new_sib.keys, threshold, 0, new_sib.num_keys);
            range_assign(&self.values,&mut new_sib.values, threshold, 0, new_sib.num_keys);

            self.num_keys = threshold;

            if idx >= threshold {
                idx -= threshold;
                new_sib.keys[idx] = k;
                new_sib.values[idx] = v;
                new_sib.num_keys += 1;
            } else {
                self.keys[idx] = k;
                self.values[idx] = v;
                self.num_keys += 1;
            }

            return (false, Some(new_sib));
        } else {
            // we have space to accomodate the new key
            for i in (idx .. self.num_keys).rev() {
                self.keys[i + 1] = self.keys[i];
                self.values[i + 1] = self.values[i];
            }
            self.num_keys += 1;
            self.keys[idx] = k;
            self.values[idx] = v;
        }
        return (false, None);
    }
}

// the node to store a B+Trpee node (including internal and extern)
pub struct InternalNode <K : PartialOrd,V> {
    num_keys : usize,
    keys : [K; MAX_KEYS],
    links : [Option<Box<Node<K,V>>>; MAX_KEYS],
}

impl <K,V> InternalNode<K,V>
where K : PartialOrd + Copy, V : Copy
{
    pub fn new() -> Self {
        unsafe {
            InternalNode { num_keys : 0,
                       keys : MaybeUninit::uninit().assume_init(),
                           links : MaybeUninit::uninit().assume_init(),
            }
        }
    }

    // copt num elements from myself to next
    pub fn copy_n_to(&mut self, next : *mut InternalNode<K,V>, num : usize) {
        unimplemented!();
        // TODO
        unsafe {
            (*next).num_keys = self.num_keys - num;
        }
    }
}

impl <K,V> Drop for InternalNode<K,V>
where K : PartialOrd
{
    fn drop(&mut self) {
        for i in 0..self.num_keys {
            let mut temp = self.links[i].take();
        }
    }
}


pub enum Node<K : PartialOrd,V> {
    Internal (InternalNode<K,V>),
    Leaf (LeafNode<K,V> )
}

mod tests {
    use super::*;
    type TestNode = Node<usize,usize>;
    type TestInter = InternalNode<usize,usize>;
    type TestLeaf = LeafNode<usize,usize>;

    #[test]
    fn test_basic() {
        let mut a = TestLeaf::new();

        let keys = [2,3,5,12,4,635];

        // insert
        for k in keys.iter() {
            println!("Try insert: {}",k);
            a.insert(*k,*k);
            println!("Insert {}, key: {:?}", k, a);
        }

        // check
        for k in keys.iter() {
            assert_eq!(a.get(k).unwrap(), *k);
        }

        let keys2 = [10,11,12,13,14,15,20,17,18,19,16];
        for k in keys2.iter() {
            a.insert(*k,*k);
        }

        for k in keys.iter() {
            assert_eq!(a.get(k).unwrap(), *k);
        }

        for k in keys2.iter() {
            let val = a.get(k);
            match val {
                Some (v) => assert_eq!(v,*k),
                None => assert_eq!(*k,*k + 1),
            }
        }
        assert_eq!(a.num_keys(), MAX_KEYS);
        let (_,new_leaf) = a.insert(733333,733333);
        let mut nn = new_leaf.unwrap();
        println!("{:?}", nn);
        assert_eq!(nn.get(&733333).unwrap(),733333);

        for k in keys.iter() {
            match a.get(k) {
                Some (v) => assert_eq!(*k, v) ,
                None => {
                    match nn.get(k) {
                        Some (v) => assert_eq!(*k,v),
                        None => assert_eq!(*k,*k + 1), // cannot happen
                    }
                }
            }
        }

        for k in keys2.iter() {
            match a.get(k) {
                Some (v) => assert_eq!(*k, v) ,
                None => {
                    match nn.get(k) {
                        Some (v) => assert_eq!(*k,v),
                        None => assert_eq!(*k,*k + 1), // cannot happen
                    }
                }
            }
        }

        // try modify something
        nn.get_ref(&733333).map(|v| *v += 12);
        nn.get(&733333).map(|v| assert_eq!(v,733333 + 12));
    }

    #[test]
    fn test_get_ref() {
        let mut a = TestLeaf::new();

        let mut r = {
            a.insert(12,12);
            let mut r = a.get_ref(&12).unwrap();
            *r = 12;
            r
        };
    }
}
