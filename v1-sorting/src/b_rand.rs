use std::sync::Mutex;
use std::sync::OnceLock;

pub fn rand(max: usize) -> usize {
    static RG: OnceLock<Mutex<RandGen>> = OnceLock::new();
    RG.get_or_init(|| Mutex::new(RandGen::new(34502)))
        .lock()
        .unwrap()
        .next_v(max)
}

pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(curr: usize) -> Self {
        RandGen {
            curr,
            mul: 56394237,
            inc: 346423491,
            modulo: 23254544563,
        }
    }

    pub fn next_v(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        self.curr % max
    }
}

#[cfg(test)]
mod tests {
    use super::RandGen;

    #[test]
    fn test_rands_printout() {
        let mut r = RandGen::new(12);
        for _ in 0..100 {
            println!("--{}", r.next_v(100));
        }
    }
}
