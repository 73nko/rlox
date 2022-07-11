pub struct Scanner {}

impl Scanner {
    pub fn new(_input: &str) -> Scanner {
        Scanner {}
    }

    pub fn tokens(&self) -> Vec<u32> {
        vec![0; 5]
    }
}
