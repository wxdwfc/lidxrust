pub mod node;

use node::*;

pub struct BTree<K : PartialOrd,V> {
    depth : usize,
    root  : Option<Box<node::Node<K,V>>>,
}

impl<K : PartialOrd,V> BTree<K,V> {
    pub fn new() -> Self {
        BTree { depth : 0, root : None, }
    }
}

// get methods
impl <K,V> BTree<K,V>
where K : PartialOrd + Copy, V : Copy
{

}

// put method
impl <K,V> BTree<K,V>
where K : PartialOrd + Copy, V : Copy
{
    pub fn insert(&mut self, key : K, value : V) {
        // the B+Tree is empty
        match self.root {
            None => self.root = Some(Box::new(Node::Leaf(LeafNode::new()))),
            _ => {},
        }

        if self.depth == 0 {

        }
    }
}
