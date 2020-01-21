// a flat vector which represents a RxC matrix
struct Mat22 {
    data : Vec<f64>,
    nrow : usize,
    ncol : usize,
}

impl Mat22 {
    pub fn new(nr : usize, nc : usize) -> Self {
        Mat22 { data : vec![0_f64;nr * nc], nrow : nr, ncol : nc}
    }

    pub fn nrows(&self) -> usize {
        self.nrow
    }
}
