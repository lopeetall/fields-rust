extern crate rand;

use bigint::Bigint;
//use rand::Rng;

fn main () {
       // let mut rng = rand::thread_rng();
//let max = std::u64::MAX;
let n1 = Bigint::new(vec![1, 0, 0]);
let n2 = Bigint::new(vec![1, 0]);

println!("{}", n1);
println!("{}", n2);
println!("{}", n1*n2);

}
