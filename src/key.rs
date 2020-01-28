#[derive(Debug,PartialEq)]
pub struct KVPair<K,V> {
    pub key : K,
    pub value : V,
}

impl<K,V> KVPair<K,V> {
    pub fn new(k : K, v : V) -> Self {
        KVPair { key : k, value : v}
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_eq() {
        type IntKV = KVPair<usize,usize>;
        assert_eq!(IntKV::new(12,12), IntKV::new(12,12));
    }
}
