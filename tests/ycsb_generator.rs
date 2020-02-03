pub struct YCSBKeyGen {
    total_keys : u64,
    cur_idx : u64,
}

impl YCSBKeyGen {
    pub fn new(tk : u64) -> Self {
        YCSBKeyGen { total_keys : tk, cur_idx : 0 }
    }

    pub fn reset(&mut self) {
        self.cur_idx = 0;
    }

    pub fn get_next_key(&mut self) -> Option<u64> {
        if self.cur_idx < self.total_keys {
            let res = self.cur_idx;
            self.cur_idx += 1;
            return Some(res)
        }
        None
    }
}

const kFNVOffsetBasis64 : u64 = 0xCBF29CE484222325;
const kFNVPrime64 : u64       = 1099511628211;

// hash the key to get a sparsh key space
fn hash(key : u64) -> u64 {
    let mut hash : u64 = kFNVOffsetBasis64;
    let mut hashed_val = key;

    for _ in 0..8 {
        let octet = hashed_val & 0x00ff;
        hashed_val = hashed_val >> 8;

        hash = hash ^ octet;
        hash = hash.wrapping_mul(kFNVPrime64);
    }

    hash
}

// the iterator class
pub struct YCSBHashIter<'a> {
    cur : &'a mut YCSBKeyGen,
}

impl<'a> Iterator for YCSBHashIter<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.get_next_key().map(|k| hash(k))
    }
}

impl YCSBKeyGen {
    pub fn into_hash_iter(&mut self) -> YCSBHashIter {
        YCSBHashIter { cur : &mut *self }
    }
}
