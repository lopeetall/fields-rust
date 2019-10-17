
// TO DO

// don't use functional type shit unless required.
// I have a LOT to do before I can apply to this job
// If I have Bigint AND Finite Fields in Rust
// Or even just Bigint
// I Can supplement my Python work for the rest
// 
// Mult
// IntDiv
// Mod


extern crate hex;

use std;
use std::fmt;
use std::ops;
use std::cmp;

#[derive(Debug)]
pub struct Bigint {
    list: Vec<u64>,
}

impl Bigint {
    pub fn new (list: Vec<u64>) -> Bigint {
        Bigint {
            list: Bigint::depad(list),
        }
    }

    pub fn add_u64_with_carry (first: &u64, other: &u64) -> (u64, u64) {
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
        (c, s)        
    }

    pub fn sub_u64_with_borrow (first: &u64, other: &u64) -> (u64, u64) {
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
        (b, d)        
    }
    //create quotient by sqrt function, use instead of add with carry in saome places
    pub fn mul_u64_with_carry (left: u64, right: u64) -> (u64, u64) {
        let leftq = (left / 0x400000000, left % 0x400000000);
        println!("left: ({:016x},{:016x})", leftq.0, leftq.1);
        let rightq = (right / 0x400000000, right % 0x400000000);
        println!("right: ({:016x},{:016x})", rightq.0, rightq.1);
        let middle = Bigint::add_u64_with_carry(&(leftq.0*rightq.1), &(leftq.1*rightq.0));
        println!("middle: ({:016x},{:016x})", middle.0, middle.1);
        let far_right = Bigint::add_u64_with_carry(&(0x400000000*middle.1), &(leftq.1*rightq.1));
        println!("far_right: ({:016x},{:016x})", far_right.0, far_right.1);
        let carry = leftq.0*rightq.0 + middle.0 + far_right.0;
        println!("5");
        let sum = far_right.1;
        (carry, sum)
    }    

    pub fn pad (list: &Vec<u64>, n: usize) -> Vec<u64> {
        let mut z = vec![0; n-list.len()];
        z.extend(list);
        z
    }

    pub fn depad (list: Vec<u64>) -> Vec<u64> {
        list.into_iter().skip_while(|&x| x == 0).collect::<Vec<u64>>()
    }
}

impl fmt::Display for Bigint {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.list[1..self.list.len()].iter()
        .map(|x| format!("{:016x}", x))
        .collect::<Vec<String>>()
        .join("");
        write!(f, "{:x}{}", self.list[0], s)
    }
}

impl ops::Add for Bigint {
    type Output = Self;

    fn add (self, other: Self) -> Bigint {
        let sum_length = cmp::max(self.list.len(), other.list.len()) + 1;
        let mut sum: Vec<u64> = vec![0; sum_length];
        let l = Bigint::pad(&self.list, sum_length);
        let r = Bigint::pad(&other.list, sum_length);
        for i in (1..(sum_length)).rev() {
            let ac = Bigint::add_u64_with_carry(&l[i], &r[i]);
            sum[i] += ac.1;
            sum[i-1] += ac.0; 
        }
        Bigint::new(sum)
    }
}

impl ops::Mul for Bigint {
    type Output = Self;

    //test with 7 and 14
/*
    fn scalar_mult (self, other: u64) -> Bigint {
        let mut prod: Vec<u64> = vec![0; self.list.len()+1];

    }
*/
    fn mul (self, other: Self) -> Bigint {
        Bigint {
            list: vec![0],
        }
    }
}

impl ops::Sub for Bigint {
    type Output = Self;
    fn sub (self, other: Self) -> Bigint {
        let n = cmp::max(self.list.len(), other.list.len());
        let mut l = Bigint::pad(&self.list, n);
        let r = Bigint::pad(&other.list, n);
        println!("{:?}", l);
        println!("{:?}", r);
        let mut d: Vec<u64> = vec![0; n];
        for i in (0..n).rev() {
            if l[i] < r[i] {
                l[i-1] -= 1; //borrow
                d[i] = 0xFFFFFFFFFFFFFFFF - r[i] + 0x1 + l[i];
            } else {
                d[i] = l[i] - r[i];
            }
        }
        println!("{:?}", d);
        Bigint::new(d)
    }
}

impl cmp::Ord for Bigint {

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let n = cmp::max(self.list.len(), other.list.len());
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



