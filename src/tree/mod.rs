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
where K : PartialOrd + Copy + std::fmt::Debug, V : Copy + std::fmt::Debug
{
    pub fn get(&self, key : K) -> Option<V> {
        self.find_leaf_page(key).map(|n| {
            match **n {
                Node::Internal(_) => unreachable!(),
                Node::Leaf(ref l) => {
                    println!("{:?}",l);
                    l.get(&key)
                }
            }
        }).and_then(|n| n)
    }

    pub fn find_leaf_page(&self, key : K) -> Option<&Box<node::Node<K,V>>> {
        let mut idx = self.depth;
        let mut cur_node : &Option<Box<node::Node<K,V>>> = &self.root;
        loop {
            if idx == 0 {
                return cur_node.as_ref().map(|n| {
                    match **n {
                        Node::Internal(_) => unreachable!(),
                        Node::Leaf(_) => cur_node,
                    } }
                ).and_then(|n| n.as_ref());
            }

            cur_node = cur_node.as_ref().map(|n| {
                match **n {
                    Node::Internal(ref i) => {
                        i.find_link(&key)
                    },
                    Node::Leaf(_) => unreachable!(),
                } }
            ).unwrap();
            idx -= 1;
        }
    }

}

// put method
impl <K,V> BTree<K,V>
where K : PartialOrd + Copy, V : Copy
{
    pub fn insert(&mut self, key : K, value : V) {
        // the B+Tree is empty
        {
            match &self.root {
                None => { self.root = Some(Box::new(Node::Leaf(LeafNode::new()))); },
                _ => {},
            }
        }

        if self.depth == 0 {
            // insert to the first layer
            //let root_ref : &mut Option<Box<Node<K,V>>> = &mut self.root;
            let new_node = self.root.as_mut().map(|node| {
                match node.as_mut() {
                    Node::Internal(_) => unreachable!(),
                    Node::Leaf(ref mut l) => {
                        let (_, nl) = l.insert(key,value);
                        nl
                    },
                }
            } ).and_then(|n| n);

            new_node.map(|n| {
                self.root = Some(Box::new(Node::Internal(InternalNode::new_from(
                    n.first_key(), self.root.take().unwrap(),n))));
                self.depth += 1;
            });

            // end one-layer case
        }
        // end insert
    }

    fn insert_to_internal(key : K, val : V, target :  &Box<node::Node<K,V>>, depth : usize) -> Option<Box<node::Node<K,V>>> {
        unimplemented!();

        match target.as_mut() {

        }
        None
    }
}


mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut t = BTree::<usize,usize>::new();
        for i in 0..17 {
            println!("insert {} start",i);
            t.insert(i, i);
            println!("insert {} done",i);
        }

        for i in 0..17 {
            println!("try get {}",i);
            let v = t.get(i);
            assert_ne!(v,None);
            assert_eq!(v.unwrap(), i);
        }
        //assert_eq!(0,-1);
    }
}
