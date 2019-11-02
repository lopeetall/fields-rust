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
        let mut list: [u64; 4] = [0; 4];
        for i in 0..4 {
            list[i] = rng.gen();
        }
        U256::new(list)
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

    pub fn overflowing_add (left: U256, right: U256) -> (U256, bool) {
        (left + right, left > U256::max() - right)
    }

    pub fn underflowing_sub (left: U256, right: U256) -> (U256, bool) {
        (left - right, left < right)
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

    pub fn overflowing_mul_u64 (left: u64, right: U256) -> (u64, U256) {
        let mut lower = [0; 4];
        let mut upper = [0; 4];
        let mut excess = 0;
        for i in (0..4).rev() {
            let mc = U256::mul_u64_with_carry(right[i], left);
            lower[i] = mc.1;
            if i > 0 {
                upper[i-1] = mc.0;
            } else {
                excess = mc.0;
            }
        }
        let oa = U256::overflowing_add(U256::new(upper), U256::new(lower));
        (excess + oa.1 as u64, oa.0)
    }

    pub fn bit_at (self, n: usize) -> bool {
        self[n / 64] & (1 << 63 - (n % 64)) != 0
    }
}

impl ops::Add for U256 {
    type Output = Self;
    fn add (self, other: Self) -> U256 {
        if other == U256::zero() {
            self
        } else {
            let mut result = [0; 4];
            let mut carries = [0; 4];
            for i in (0..4).rev() {
                let oa = self[i].overflowing_add(other[i]);
                result[i] = oa.0;
                if oa.1 {
                    if i > 0 {
                        carries[i-1] += 1;
                    }
                }
            }
        U256::new(result) + U256::new(carries)
        }      
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
        if other == U256::zero() {
            self
        } else {
            let mut result = [0; 4];
            let mut borrows = [0; 4];
            for i in (0..4).rev() {
                let oa = self[i].overflowing_sub(other[i]);
                result[i] = oa.0;
                if oa.1 {
                    if i > 0 {
                        borrows[i-1] += 1;
                    }
                }
            }
        U256::new(result) - U256::new(borrows)
        }      
    }
}

impl ops::Sub<u64> for U256 {
    type Output = Self;
    fn sub (self, other: u64) -> U256 {
        self - U256::new([0,0,0,other])
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
        let mut d: U256;
        let mut n = self;
        if self < divisor {
            self
        } else if self == divisor {
            U256::zero()
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
                let mut rt: u64;
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

impl ops::Div for U256 {
    type Output = U256;
    fn div (self, divisor: U256) -> U256 {
        if divisor == U256::zero() {
            panic!("Attempt to divide by zero in U256");
        } else if self < divisor {
            U256::zero()
        } else if self == divisor {
            U256::one()
        } else {
            let mut d: U256;
            let mut n = self;
            let mut res = U256::zero();
            while n > divisor {
                let mut s = n.len() - divisor.len();
                if n < divisor << s {
                    s -= 1;
                }
                d = divisor << s;
                res += U256::one() << s;    
                n -= d;
            }
        res
        }
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

/*-----------------------------------------------------------------*/
#[derive(Debug, Copy, Clone)]
pub struct PrimeField {
    pub prime: U256,
}

impl PrimeField {
    pub fn new (prime: U256) -> PrimeField {
        PrimeField {
            prime,
        }
    }

    pub fn rand (self) -> FieldElement {
        FieldElement::new(U256::rand(), self)
    }

    pub fn zero (self) -> FieldElement {
        FieldElement::new(U256::zero(), self)
    }

    pub fn one (self) -> FieldElement {
        FieldElement::new(U256::one(), self)
    }

    pub fn hlr (self) -> FieldElement {
        if self.prime < U256::max() {
            FieldElement::new(U256::max(), self) + self.one()
        } else {
            FieldElement::new(U256::one(), self)
        }
    }

    pub fn overflow_reduce_bool (self, sum: (U256, bool)) -> U256 {
        if !sum.1 {
            sum.0
        } else {
            self.overflow_reduce_bool(
                U256::overflowing_add(
                    sum.0,
                    self.hlr().rep
                ),            
            )
        }
    }

    pub fn underflow_reduce_bool (self, diff: (U256, bool)) -> U256 {
        if !diff.1 {
            diff.0
        } else {
            self.underflow_reduce_bool(
                U256::underflowing_sub(
                    diff.0,
                    self.hlr().rep
                ),  
            )
        }
    }

    pub fn overflow_reduce_u64 (self, scale: (u64, U256)) -> U256 {
        if scale.0 == 0 {
            scale.1
        } else {
            let om = U256::overflowing_mul_u64(scale.0, self.hlr().rep);
            self.overflow_reduce_u64 (
                (
                    om.0,
                    self.overflow_reduce_bool(
                        U256::overflowing_add(
                            om.1,
                            scale.1,
                        )
                    )
                )
            )
        }
    }

    pub fn overflow_reduce_limb_shift (self, lower: U256, n: usize) -> U256 {
        let mut result = lower;
        for i in 0..n {
            result = self.overflow_reduce_u64((result[0], result << 64));
        }
        result
    }

    pub fn overflow_reduce_u256 (self, upper: U256, lower: U256) -> U256 {
        let mut ru = upper;
        println!("{}", ru);
        for i in 0..4 {
            ru = self.overflow_reduce_limb_shift(ru, 1);
                    println!("{}", ru);
        }
        self.overflow_reduce_bool(
            U256::overflowing_add(
                ru,
                lower,
            ),
        )
    }
}

impl fmt::Display for PrimeField {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Finite field of order {}", self.prime)
    }
}

/*-----------------------------------------------------------------*/

#[derive(Debug, Copy, Clone)]
pub struct FieldElement {
    pub rep: U256,
    pub field: PrimeField,
}

impl FieldElement {
    pub fn new (r: U256, field: PrimeField) -> FieldElement {
        FieldElement {
            rep: r % field.prime,
            field,
        }
    }

    pub fn square (self) -> FieldElement {
        self*self
    }

    pub fn pow (self, e: U256) -> FieldElement {
        let mut p = self.field.one();
        for i in 0..256 {
            if e.bit_at(i) {
                p = p.square()*self;
            } else {
                p = p.square();
            }
        }
        p
    }

    pub fn inv (self) -> FieldElement {
        self.pow(self.field.prime - 2)
    }
}

impl fmt::Display for FieldElement {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rep)
    }
}

impl ops::Add for FieldElement {
    type Output = Self;
    fn add (self, other: FieldElement) -> FieldElement {
        FieldElement::new(
            self.field.overflow_reduce_bool(
                U256::overflowing_add(
                    self.rep,
                    other.rep,
                ),
            ),
            self.field,
        )
    }
}

impl ops::Sub for FieldElement {
    type Output = Self;
    fn sub (self, other: FieldElement) -> FieldElement {
        FieldElement::new(
            self.field.underflow_reduce_bool(
                U256::underflowing_sub(
                    self.rep,
                    other.rep,
                ),
            ),
            self.field,
        )
    }
}

impl ops::Mul for FieldElement {
    type Output = Self;
    fn mul (self, other: FieldElement) -> FieldElement {
        let mut result = self.field.zero();
        for i in (0..4).rev() {
            result = result + FieldElement::new(
                self.field.overflow_reduce_limb_shift(
                    self.field.overflow_reduce_u64(
                        U256::overflowing_mul_u64 (self.rep[i], other.rep)
                    ),
                    3-i,
                ),
                self.field,
            )   
        }
        result
    }
}

impl ops::MulAssign for FieldElement {
    fn mul_assign (&mut self, other: FieldElement) {
        *self = self.clone() * other 
    }
}

/*-----------------------------------------------------------------*/
