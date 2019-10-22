// TO DO 

// MAKE IT IMPRESSIVE

// Implement:
// Shift Left **
// AddAssign **
// SubAssign **
// MulAssign **
// Make Bigint indexable and remove the .lists when necessary **
// give Bigints a len() function and remove .list.len() when necessary **
// ---MAke Bigint iterable and remove the .into_iter when necessary (maybe not?)---

// mod
// mod assign
// int div
// int div assign

// DONE?

extern crate hex;

use std;
use std::fmt;
use std::ops;
use std::cmp;

#[derive(Debug)]
pub struct Bigint {
    pub list: Vec<u64>,
}

impl Bigint {
    pub fn new (list: Vec<u64>) -> Bigint {
        Bigint {
            list: Bigint::depad(&list),
        }
    }

    pub fn add_u64_with_carry (first: &u64, other: &u64) -> Vec<u64> {
        let c: u64;
        let s: u64;
        if first > &(0xFFFFFFFFFFFFFFFF - other) {
            c = 1;
            s = (first - (0xFFFFFFFFFFFFFFFF - other)) - 0x1;
        } else {
            c = 0;
            s = first + other;
        }
        // (carry, sum mod 2^64)
        vec![c, s]        
    }

    pub fn len (&self) -> usize {
        self.list.len()
    }

    pub fn sub_u64_with_borrow (first: &u64, other: &u64) -> Vec<u64> {
        let b: u64;
        let d: u64;
        if first < other {
            b = 1;
            d = ((0xFFFFFFFFFFFFFFFF - other) + 0x1) + first;
        } else {
            b = 0;
            d = first - other;
        }
        // (borrow, difference mod 2^64)
        vec![b, d]        
    }

    pub fn mul_u64_with_carry (left: u64, right: u64) -> Vec<u64> {
        let sqrt = 0x100000000;
        let leftq = vec![left / sqrt, left % sqrt];
        let rightq = vec![right / sqrt, right % sqrt];
        let tmp_middle = [(leftq[0]*rightq[1]) / sqrt + (leftq[1]*rightq[0]) / sqrt, (leftq[0]*rightq[1]) % sqrt + (leftq[1]*rightq[0]) % sqrt];
        let (big, middle, little) = (leftq[0]*rightq[0] + tmp_middle[0], tmp_middle[1]*sqrt, leftq[1]*rightq[1]);
        let littles = Bigint::add_u64_with_carry(&middle, &little);
        vec![big + littles[0], littles[1]]
    }  

    pub fn mul_by_u64 (&self, other: u64) -> Bigint {
        let n = self.len();
        let mut r = Bigint::new(vec![]);
        for i in (0..n).rev() {
            r += Bigint::new(Bigint::mul_u64_with_carry(self[i], other)) << n-1-i;
        }
        r
    }

    pub fn pad (list: &Vec<u64>, n: usize) -> Vec<u64> {
        let mut z = vec![0; n-list.len()];
        z.extend(list);
        z
    }

    pub fn depad (list: &Vec<u64>) -> Vec<u64> {
        // remove leading zeros before creating a new bigint
        // leaves one zero element if the big int is actually 0
        // I would love to see a more functional way of doing this
        let mut r = vec![];
        let mut sigs_have_begun = false;
        for i in 0..list.len() {
            if list[i] != 0 || sigs_have_begun || i == list.len() - 1{
                sigs_have_begun = true;
                r.push(list[i]);
            }
        }
        r
    }
}

impl fmt::Display for Bigint {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.list[1..self.len()].iter()
        .map(|x| format!("{:016x}", x))
        .collect::<Vec<String>>()
        .join("");
        write!(f, "{:x}{}", self[0], s)
    }
}

impl ops::Add for Bigint {
    type Output = Self;
    fn add (self, other: Self) -> Bigint {
        let sum_length = cmp::max(self.len(), other.len()) + 1;
        let mut sum: Vec<u64> = vec![0; sum_length];
        let l = Bigint::pad(&self.list, sum_length);
        let r = Bigint::pad(&other.list, sum_length);
        for i in (1..(sum_length)).rev() {
            let ac = Bigint::add_u64_with_carry(&l[i], &r[i]);
            sum[i] += ac[1];
            sum[i-1] += ac[0]; 
        }
        Bigint::new(sum)
    }
}

impl ops::AddAssign for Bigint {
    fn add_assign (&mut self, other: Self) {
        let placeholder = &self.list;
        *self = Bigint::new(placeholder.to_vec()) + other;
    }
}

impl ops::Shl<usize> for Bigint {
    type Output = Self;
    fn shl (self, n: usize) -> Bigint {
        let mut r = self.list;
        r.extend(vec![0; n]);
        Bigint::new(r)
    }
}

impl ops::Mul for Bigint {
    type Output = Self;
    fn mul (self, other: Self) -> Bigint {
        let n = other.len();
        let mut r = Bigint::new(vec![]);
        for i in (0..n).rev() {
            r += self.mul_by_u64(other[i]) << n-1-i;
        }    
        r
    }
}

impl ops::MulAssign for Bigint {
    fn mul_assign (&mut self, other: Self) {
        let placeholder = &self.list;
        *self = Bigint::new(placeholder.to_vec()) * other;
    }
}

impl ops::Sub for Bigint {
    type Output = Self;
    fn sub (self, other: Self) -> Bigint {
        let n = cmp::max(self.len(), other.len());
        let mut l = Bigint::pad(&self.list, n);
        let r = Bigint::pad(&other.list, n);
        let mut d: Vec<u64> = vec![0; n];
        for i in (0..n).rev() {
            if l[i] < r[i] {
                l[i-1] -= 1; //borrow
                d[i] = 0xFFFFFFFFFFFFFFFF - r[i] + 0x1 + l[i];
            } else {
                d[i] = l[i] - r[i];
            }
        }
        Bigint::new(d)
    }
}

impl ops::SubAssign for Bigint {
    fn sub_assign (&mut self, other: Self) {
        let placeholder = &self.list;
        *self = Bigint::new(placeholder.to_vec()) - other;
    }
}

impl ops::Index<usize> for Bigint {
    type Output = u64;
    fn index (&self, idx: usize) -> &u64 {
        &self.list[idx]
    }
}

impl cmp::Ord for Bigint {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let n = cmp::max(self.len(), other.len());
        let l = Bigint::pad(&self.list, n);
        let r = Bigint::pad(&other.list, n);
        let mut i = 0;
        while l[i] == r[i] && i < n-1 { i += 1 }
        l[i].cmp(&r[i])
    }
}

impl cmp::PartialOrd for Bigint {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for Bigint {
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

impl cmp::Eq for Bigint {}



