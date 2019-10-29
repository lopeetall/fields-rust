extern crate rand;

use u256::U256;

fn main () {

//let n1 = U256::rand();
//let n2 = U256::rand();
//let d = U256::rand() >> 60;


let n1 = U256::new([0x1, 0x0, 0x0, 0x0]);
let d = U256::new([0x0, 0x0, 0x0, 0x1]);
println!("n1:       {}", n1);
println!("divisor:  {}", d);

println!("quo:      {}", n1 / d);
//r = n1 - n2;

//println!("r:  {}", r);

}
