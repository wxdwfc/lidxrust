use std::mem::{MaybeUninit};

pub const MAX_KEYS : usize = 16; // number of keys per node

fn range_assign<T : Copy> (src : & [T;MAX_KEYS], target : &mut [T;MAX_KEYS],
                           start_src : usize, start_target : usize, num : usize) {
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

    pub fn first_key(&self) -> K {
        assert!(self.num_keys >= 1);
        self.keys[0]
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
        while idx < self.num_keys && self.keys[idx] < *k {
            idx += 1;
        }

        // the key has already been there
        if idx < self.num_keys && *k == self.keys[idx] {
            return (true, idx);
        }
        return (false, idx);

    }

    // \ret: whether find (bool), the newly splitted leaf
    pub fn insert(&mut self, k : K, v : V) -> (bool, Option<Box<Node<K,V>>>) {
        let (find, mut idx) = self.find(&k);
        if find {
            return (true, None);
        }

        // insert
        if self.num_keys == MAX_KEYS {
            // split
            let mut new_sib = LeafNode::new();

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

            return (false, Some(Box::new(Node::Leaf(new_sib))));
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
//#[derive(Debug)]
pub struct InternalNode <K : PartialOrd,V> {
    num_keys : usize,
    keys : [K; MAX_KEYS],
    links : [Option<Box<Node<K,V>>>; MAX_KEYS + 1],
    up_key : K, // a temporal store for storing upper key
}

impl <K,V> InternalNode<K,V>
where K : PartialOrd + Copy, V : Copy
{
    pub fn new() -> Self {
        let res = unsafe {
            let res = InternalNode { num_keys : 0,
                                     keys : MaybeUninit::uninit().assume_init(),
                                     links : Default::default(),
                                     up_key : MaybeUninit::uninit().assume_init(),
            };
            res
        };
        res
    }

    pub fn new_from(k : K, n0 : Box<Node<K,V>>, n1 : Box<Node<K,V>>) -> Self {
        let mut res = InternalNode::new();
        res.num_keys = 1;
        res.keys[0] = k;

        res.links[0] = Some(n0);
        res.links[1] = Some(n1);
        res
    }

    pub fn num_keys(&self) -> usize {
        self.num_keys
    }

    pub fn find_link(&self, k : &K) -> &Option<Box<Node<K,V>>> {
        let mut idx = 0;
        while idx < self.num_keys() {
            if k < &self.keys[idx] {
                return &self.links[idx];
            }
            idx += 1;
        }
        &self.links[idx]
    }

    pub fn first_key(&self) -> K {
        assert!(self.num_keys >= 1);
        self.keys[0]
    }

    pub fn end_key(&self) -> K {
        self.keys[self.num_keys() - 1]
    }

    pub fn insert(&mut self, key : K, val : V) -> Option<Box<Node<K,V>>> {
        let mut pos = self.find_target_pos(&key);
        let new_node = self.links[pos].as_mut().map(|n| n.insert(key,val)).and_then(|n| n);

        new_node.map(|n| {
            // insert the new node to myself
            if self.num_keys == MAX_KEYS {
                let threshold = (MAX_KEYS + 1) / 2;
                let mut new_node = self.split_n(threshold);
                self.num_keys = threshold - 1;

                let up_key = self.keys[threshold - 1];

                if n.get_up_key() > up_key {
                    // insert to the new node
                    pos -= threshold;
                    new_node.insert_new_node(n, pos);
                } else {
                    // insert to myself
                    self.insert_new_node(n, pos);
                }
                new_node.set_up_key(up_key);
                Some(new_node)
            } else {
                self.insert_new_node(n,pos);
                None
            }
        }).and_then(|n| n)
    }

    pub fn insert_new_node(&mut self, node : Box<Node<K,V>>, pos : usize) {
        assert!(pos <= self.num_keys());
        for i in (pos+1..self.num_keys()+1).rev() {
            self.keys[i] = self.keys[i - 1];
            self.links[i + 1] = self.links[i].take();
        }
        self.keys[pos] = node.get_up_key();
        self.links[pos + 1] = Some(node);
        self.num_keys += 1;
    }

    pub fn set_first_key(&mut self, k : K) {
        self.keys[0] = k;
    }

    // copy the elements after *num* from myself to "next"
    pub fn split_n(&mut self, num : usize) -> Box<Node<K,V>> {
        assert!(num < self.num_keys);
        let mut target = InternalNode::new();
        target.num_keys = self.num_keys - num;

        for i in 0..target.num_keys {
            target.keys[i] = self.keys[num + i];
            target.links[i] = self.links[num + i].take().map(|l| l);
        }
        target.links[target.num_keys] = self.links[self.num_keys].take().map(|l| l);

        Box::new(Node::Internal(target))
    }

    pub fn find_target_pos(&self, k : &K) -> usize {
        let mut idx : usize = 0;
        while idx < self.num_keys() && self.keys[idx] <= *k {
            idx += 1;
        }
        idx
    }
}


//#[derive(Debug)]
pub enum Node<K : PartialOrd,V> {
    Internal (InternalNode<K,V>),
    Leaf (LeafNode<K,V> )
}

impl <K,V> Node <K,V>
where K : PartialOrd + Copy, V : Copy {
    pub fn first_key(&self) -> K {
        match self {
            Node::Leaf(l) => l.first_key(),
            Node::Internal(i) => i.first_key(),
        }
    }

    pub fn num_keys(&self) -> usize {
        match self {
            Node::Leaf(l) => l.num_keys(),
            Node::Internal(i) => i.num_keys(),
        }
    }

    pub fn get_up_key(&self) -> K {
        match self {
            Node::Leaf(l) => l.first_key(),
            Node::Internal(i) => i.up_key,
        }
    }

    pub fn set_up_key(&mut self, k : K) {
        match self {
            Node::Leaf(_) => unreachable!(),
            Node::Internal(i) => { i.up_key = k },
        }
    }

    pub fn set_first_key(&mut self, k : K) {
        match self {
            Node::Leaf(_) => unreachable!(),
            Node::Internal(i) => i.set_first_key(k),
        }
    }

    pub fn insert_new_node(&mut self, node : Box<Node<K,V>>, pos : usize) {
        match self {
            Node::Leaf(_) => unreachable!(),
            Node::Internal(i) => i.insert_new_node(node, pos),
        }
    }

    pub fn insert(&mut self, key : K,val : V,) -> Option<Box<Self>> {
        match self {
            Node::Internal(ref mut inner) => {
                inner.insert(key, val)
            }
            Node::Leaf(ref mut l) => {
                let (_, nl) = l.insert(key,val);
                nl
            }
        }
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_basic() {
        type TestLeaf = LeafNode<usize,usize>;

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

        // TODO: re-write tests

    }

    #[test]
    fn test_get_ref() {
        type TestLeaf = LeafNode<usize,usize>;
        let mut a = TestLeaf::new();

        a.insert(12,12);
        let mut r = a.get_ref(&12).unwrap();
        *r = 12;
        println!("{}",r);
    }

    #[test]
    #[should_panic]
    fn test_internal() {
        //type TestLeaf = LeafNode<usize,usize>;
        type TestInter = InternalNode<usize,usize>;
        let mut node = TestInter::new();
        node.split_n(1);
    }

    #[test]
    fn test_internal0() {
        type TestNode = Node<usize,usize>;
        type TestInter = InternalNode<usize,usize>;

        let node = Box::new(TestNode::Internal(TestInter::new()));
        //let mut new_root = Box::new(Node::Internal(InternalNode::<usize,usize>::new()));
    }
}
