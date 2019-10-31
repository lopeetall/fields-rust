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

    pub fn sqrt () -> U256 {
        U256::new([0x0,0x1,0x0,0x0])
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

    pub fn recursing_mul_mod (upper: u64, lower: U256, divisor: U256) -> (u64, U256) {
        let hlr = (U256::max() % divisor) + U256::one();
        println!("hlr: {}", hlr);
        let upp_res = hlr * upper;
        println!("up0: {}", upp_res.0);
        println!("up1: {}", upp_res.1);        
        let ra = U256::recursing_add(lower, upp_res.1);
        println!("ra.0 {}", ra.0);
        println!("ra.1 {}", ra.1);
        (ra.1[3] + upp_res.0, ra.0) 
    }

    pub fn mul_u64_with_carry (left: u64, right: u64) -> (u64, u64) {
        let sqrt = 0x100000000;
        let leftq = [left / sqrt, left % sqrt];
        let rightq = [right / sqrt, right % sqrt];
        let tmp_middle = [(leftq[0]*rightq[1]) / sqrt + (leftq[1]*rightq[0]) / sqrt, (leftq[0]*rightq[1]) % sqrt + (leftq[1]*rightq[0]) % sqrt];
        let lower_middle = [tmp_middle[1] / sqrt, tmp_middle[1] % sqrt];
        let (big, middle, little) = (leftq[0]*rightq[0] + tmp_middle[0] + lower_middle[0], lower_middle[1]*sqrt, leftq[1]*rightq[1]);
        let littles = middle.overflowing_add(little);
        if littles.1 {
            (big + 1, littles.0)
        } else {
            (big, littles.0)
        }
    }

    pub fn len (self) -> usize {
        let mut result: usize = 256;
        let mut i = 0;
        while self[i] == 0 && i < 3 {
            i += 1;
            result -= 64; 
        }
        result -= self[i].leading_zeros() as usize;
        result
    }

    pub fn mul_mod (self, other: Self, divisor: Self) -> U256 {
        let hlr = (U256::max() % divisor) + 1;
        let mut res = U256::zero();
        //for i in (0..4).rev() {
            //let uprod = self * other[i];
            //res += uprod.1 % divisor;
            //let hi_par = uprod.0
            //res += mul_mod(hlr * uprod.0)
        //}
        U256::zero()
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


impl ops::Add<u64> for U256 {
    type Output = Self;
    fn add (self, other: u64) -> U256 {
        let mut rc = U256::recursing_add(self, U256::new([0,0,0,other]));
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

impl ops::Rem for U256 {
    type Output = U256;
    fn rem(self, divisor: U256) -> U256 {
        let mut d = divisor;
        let mut n = self;
        if self < divisor {
            self
        } else {
            while n > divisor {
                d = divisor << n.len() - divisor.len();
                if n < d {
                    d >>= 1;
                }      
                n -= d;
            }
        n
        }
    }
}

impl ops::Div for U256 {
    type Output = U256;
    fn div (self, divisor: U256) -> U256 {
        if divisor == U256::zero() {
            panic!("Attempt to divide by zero in U256");
        } else if self < divisor {
            U256::zero()
        } else {
            let mut d = divisor;
            let mut n = self;
            let mut res = U256::zero();
            let mut i = 0;
            while n > divisor {
                let mut s = n.len() - divisor.len();
                if n < divisor << s {
                    s -= 1;
                }
                d = divisor << s;
                res += U256::one() << s;    
                n -= d;
                i += 1;
            }
        res
        }
    }
}

impl ops::Mul<u64> for U256 {
    type Output = (u64, Self);
    fn mul (self, other: u64) -> (u64, U256) {
        let mut lo_r: [u64; 4] = [0; 4];
        for i in (1..4).rev() {
            let mc = U256::mul_u64_with_carry(self[i], other);
            let ra = lo_r[i].overflowing_add(mc.1);
            lo_r[i] = ra.0;
            lo_r[i-1] += ra.1 as u64 + mc.0;
        }
        let mc = U256::mul_u64_with_carry(self[0], other);
        let ra = lo_r[0].overflowing_add(mc.1);
        lo_r[0] = ra.0;
        let hi_r = ra.1 as u64 + mc.0;
        (hi_r, U256::new(lo_r))
    }
}

impl ops::Shl<usize> for U256 {
    type Output = Self;
    fn shl (self, n: usize) -> U256 {
        if n > 255 {
            panic!("attempt to shift left past end of U256");
        }
        if n == 0 {
            self
        } else {
            let q = n / 64;
            let r = n % 64;
            let mut result = self.list;
            if r != 0 {
                let mut lt: u64 = 0;
                let mut rt: u64;
                for i in (0..4).rev() {
                    rt = self[i] << r;
                    result[i] = rt + lt;
                    lt = self[i] >> 64 - r;
                }
            }
            for i in 0..4 {
                if i + q < 4 {
                    result[i] = result[i+q];
                } else {
                    result[i] = 0;
                }
            }
            U256::new(result)
        }
    }
}

impl ops::ShlAssign<usize> for U256 {
    fn shl_assign (&mut self, n: usize) -> () {
        let new = self.clone();
        *self = new << n;
    }
}

impl ops::Shr<usize> for U256 {
    type Output = Self;
    fn shr (self, n: usize) -> U256 {
        if n > 255 {
            panic!("attempt to shift right past end of U256");
        }
        if n == 0 {
            self
        } else {
            let q = n / 64;
            let r = n % 64;
            let mut result = self.list;
            if r != 0 {
                let mut lt: u64 = 0;
                let mut rt: u64 = 0;
                for i in 0..4 {
                    rt = self[i] >> r;
                    result[i] = rt + lt;
                    lt = self[i] <<  64 - r;
                }
            }
            for i in (0..4).rev() {
                if i >= q {
                    result[i] = result[i-q];
                } else {
                    result[i] = 0;
                }
            }
        U256::new(result)
        }
    }
}

impl ops::ShrAssign<usize> for U256 {
    fn shr_assign (&mut self, n: usize) -> () {
        let new = self.clone();
        *self = new >> n;
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