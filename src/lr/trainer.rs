pub struct Trainer<K,V> {
    pub key_set : Vec<K>,
    pub value_set : Vec<V>,
}

impl <K,V> Trainer<K,V> {
    pub fn new() -> Self {
        Trainer { key_set : Vec::new(), value_set : Vec::new() }
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn basic() {
        let t = Trainer::<usize,usize>::new();
        //assert_eq!(0,-1);
    }
}
