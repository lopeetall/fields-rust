extern crate rand;

use u256::U256;

fn main () {

//let n1 = U256::rand();
//let n2 = U256::rand();
let d = U256::new([0x0000000000000000, 0x58c5dbf797d1ee40, 0x4dc0a95542c50eb9, 0x20388444e4e1b093]);


let n1 = U256::new([0xc86821ab51beb61b, 0xe3d0f1545c623a1e, 0x47bfa9294d0bef0b, 0x7ddfe3497ebd0462]);
let n2 = U256::new([0x356e7c704f111143, 0xef2029456397928b, 0x8d157277606a762c, 0x7d117ca61adee118]);
let u = 0x7d117ca61adee118;

println!("n1:           {}", n1);
println!(" u:           {:016x}", u);
println!(" d:           {}", d);

let p = U256::recursing_mul_mod(u, n1, d);
println!(" p:           {}", p.1);

//println!("quo:      {}", n1 / d);
//r = n1 - n2;

//println!("r:  {}", r);
//println!("lower n1 * n2 {}", U256::low_mul(n1, n2));
//println!("l_mid n1 * n2 {}", U256::left_mid_mul(n1, n2));
//println!("r_mid n1 * n2 {}", U256::right_mid_mul(n1, n2));
//println!("upper n1 * n2 {}", U256::hi_mul(n1, n2));

}
