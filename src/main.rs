extern crate rand;

use u256::U256;

fn main () {

let n1 = U256::rand();
let n2 = U256::rand();

let r: U256;

println!("n1      : {}", n1);
println!("n1 << 12: {}", n1 << 12);

r = n1 - n2;

//println!("r:  {}", r);

}
