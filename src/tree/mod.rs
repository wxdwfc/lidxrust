pub mod node;

use node::*;

pub struct BTree<K : PartialOrd,V> {
    root  : Option<Box<node::Node<K,V>>>,
}

impl<K : PartialOrd,V> BTree<K,V> {
    pub fn new() -> Self {
        BTree { root : None, }
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

        // insert to root
        let new_node = self.root.as_mut().unwrap().insert(key,value);

        // update root if possible
        new_node.map(|n| {
            self.root = Some(Box::new(Node::Internal(InternalNode::new_from(
                n.first_key(), self.root.take().unwrap(),n))));
        });

        // end one-layer case
        // end insert
    }


}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn basic() {
        let mut t = BTree::<usize,usize>::new();

        let test_num = 128;

        for i in 0..test_num {
            //println!("insert {} start",i);
            t.insert(i, i);
            //println!("insert {} done",i);
        }

        for i in 0..test_num {
            //println!("try get {}",i);
            let v = t.get(i);
            assert_ne!(v,None);
            assert_eq!(v.unwrap(), i);
        }

        //assert_eq!(0,-1);
    }
}
