extern crate rand;

use bigint::Bigint;
use rand::Rng;

fn main () {
let mut rng = rand::thread_rng();
let max = std::u64::MAX;

let mut n1 = Bigint::new(vec![rng.gen()]);
let mut n2 = Bigint::new(vec![rng.gen()]);
let mut d = Bigint::new(vec![rng.gen()]);

for i in 0..4 {
    n1 = n1 << 1;
    n1 += Bigint::new(vec![rng.gen()]); 
}

for i in 0..2 {
    n2 = n2 << 1;
    n2 += Bigint::new(vec![rng.gen()]); 
}

for i in 0..2 {
    d = d << 1;
    d += Bigint::new(vec![rng.gen()]); 
}


println!("{:016x}", n1[0]);
let a = Bigint::first_digit_with_index(n1[0]);
println!("{:016x} at index {}", a.0, a.1);

}
