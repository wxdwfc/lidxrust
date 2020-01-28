// a flat vector which represents a RxC matrix
struct Mat22<T : std::default::Default > {
    data : Vec<T>,
    nrow : usize,
    ncol : usize,
}

impl<T> Mat22<T>
where T :  std::default::Default + Copy
{
    pub fn new_default(nr : usize, nc : usize) -> Self {
        Mat22 { data : vec![T::default(); nr], nrow : nr, ncol : nc}
    }

    pub fn new_null(ncol : usize) -> Self {
        Mat22 { data : Vec::new(), nrow : 0, ncol : ncol }
    }

    pub fn nrows(&self) -> usize {
        self.nrow
    }

    pub fn ncols(&self) -> usize {
        self.ncol
    }
    pub fn at(&self,r : usize,  c : usize) -> T {
        self.data[r * self.nrow + c]
    }

    pub fn add_row(&mut self, mut row : Vec<T>) {
        assert!(self.ncol == row.len());
        self.data.append(&mut row);
        self.nrow += 1;
    }
}

impl<T> Mat22<T>
where T : std::default::Default + Copy
{
    pub fn at_ref(&self,r : usize,  c : usize) -> &T {
        &self.data[r * self.nrow + c]
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn basic() {
        let mut mat = Mat22::new_null(2);
        mat.add_row([1,2].to_vec());
        mat.add_row([3,4].to_vec());

        assert_eq!(mat.at(0,0),1);
        assert_eq!(mat.at(0,1),2);
        assert_eq!(mat.at(1,0),3);
        assert_eq!(mat.at(1,1),4);
    }
}
