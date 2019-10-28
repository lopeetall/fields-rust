extern crate hex;

use std;
use std::fmt;
use std::ops;
use std::cmp;

use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct U256 {
    pub list: [u64; 4],
}



impl U256 {
    pub fn new (list: [u64; 4]) -> U256 {
        U256 {
            list,
        }
    }

    pub fn zero () -> U256 {
        U256::new([0; 4])
    }

    pub fn one () -> U256 {
        U256::new([0,0,0,1])
    }

    pub fn max () -> U256 {
        U256::new([std::u64::MAX; 4])
    }

    pub fn rand () -> U256 {
        let mut rng = rand::thread_rng();
        let mut result: [u64; 4] = [0; 4];
        for i in 0..4 {
            result[i] = rng.gen();
        }
        U256::new(result)
    }

    pub fn recursing_add (left: U256, right: U256) -> (U256, U256) {
        let mut result = [0; 4];
        let mut carries = [0; 4];
        for i in (0..4).rev() {
            let oa = left[i].overflowing_add(right[i]);
            result[i] = oa.0;
            if oa.1 {
                if i == 0 {
                    panic!("attempt to add U256 with overflow");
                } else {
                    carries[i-1] += 1;
                }
            }
        } 
        (U256::new(result), U256::new(carries))
    }

    pub fn recursing_sub (left: U256, right: U256) -> (U256, U256) {
        let mut result = [0; 4];
        let mut borrows = [0; 4];
        for i in (0..4).rev() {
            let oa = left[i].overflowing_sub(right[i]);
            result[i] = oa.0;
            if oa.1 {
                if i == 0 {
                    panic!("attempt to subtract U256 with overflow");
                } else {
                    borrows[i-1] += 1;
                }
            }
        } 
        (U256::new(result), U256::new(borrows))
    }
}

impl ops::Add for U256 {
    type Output = Self;
    fn add (self, other: Self) -> U256 {
        let mut rc = U256::recursing_add(self, other);
        while rc.1 > U256::zero() {
            rc = U256::recursing_add(rc.0, rc.1);
        }
        rc.0
    }
}

impl ops::AddAssign for U256 {
    fn add_assign (&mut self, other: Self) {
        let sc = self.clone();
        *self = sc + other;
    }
}

impl ops::Sub for U256 {
    type Output = Self;
    fn sub (self, other: Self) -> U256 {
        let mut rb = U256::recursing_sub(self, other);
        while rb.1 > U256::zero() {
            rb = U256::recursing_sub(rb.0, rb.1);
        }
        rb.0
    }
}

impl ops::SubAssign for U256 {
    fn sub_assign (&mut self, other: Self) {
        let sc = self.clone();
        *self = sc - other;
    }
}


impl ops::Shl<usize> for U256 {
    type Output = Self;
    fn shl (self, n: usize) -> U256 {
        if n > 255 {
            panic!("attempt to shift left past end of U256");
        }
        let q = n / 64;
        let r = n % 64;
        let mut result = [0; 4];
        let mut lt: u64 = 0;
        let mut rt: u64 = 0;
        for i in (0..4).rev() {
            rt = self[i] << r;
            result[i] = rt + lt;
            lt = self[i] >> 64 - r;
        }
        U256::new(result)
    }
}

impl ops::Shl<usize> for U256 {
    type Output = Self;
    fn shr (self, n: usize) -> U256 {
        if n > 255 {
            panic!("attempt to shift right past end of U256");
        }
        let q = n / 64;
        let r = n % 64;
        let mut result = [0; 4];
        let mut lt: u64 = 0;
        let mut rt: u64 = 0;
        for i in 0..4 {
            rt = self[i] << r;
            result[i] = rt + lt;
            lt = self[i] >> 64 - r;
        }
        U256::new(result)
    }
}


impl fmt::Display for U256 {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}{:016x}{:016x}{:016x}", self[0], self[1], self[2], self[3])
    }
}

impl ops::Index<usize> for U256 {
    type Output = u64;
    fn index (&self, idx: usize) -> &u64 {
        &self.list[idx]
    }
}

impl cmp::Ord for U256 {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let mut i = 0;
        while self[i] == other[i] && i < 3 { i += 1 }
        self[i].cmp(&other[i])
    }
}

impl cmp::PartialOrd for U256 {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for U256 {
    fn eq(&self, other: &Self) -> bool {
        let r = self.list.clone();
        r.iter()
        .zip(&other.list)
        .map(|x| match x {
            (a, b) => *a == *b,
        })
        .all(|x| x)
    }
}

impl cmp::Eq for U256 {}
