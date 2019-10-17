extern crate rand;

use bigint::Bigint;
use rand::Rng;

fn main () {
       // let mut rng = rand::thread_rng();

        let n1 = 0xE000000000000000;
        let n2 = 0x7;
        println!("{:016x}", n1);
        println!("{:016x}", n2);
        let c = Bigint::mul_u64_with_carry(n1, n2);
        println!("({:016x},{:016x})", c.0, c.1);
}
