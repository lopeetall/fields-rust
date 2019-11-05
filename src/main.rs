extern crate rand;

mod fields;

use fields::U256;
use fields::PrimeField;
use fields::FieldElement;

fn main () {

let n1 = U256::new([0xee93d8ec92be8bcb, 0xb108da0e7a99aee1, 0x651a3cc2f62dcf59, 0x309f46631f159246]);
let n2 = U256::new([0x734ab324171e6862, 0x5a31a671d3d0f58f, 0x931d7743364beec8, 0x1da9a977404d3fc4]);
let d =  U256::new([0x09a762d2d4e6dba2, 0x3fa7c65d90e7c587, 0xa16e4fd39b5ecd04, 0x476ae7bb64802287]);

let f = PrimeField::new(d);
let p1 = FieldElement::new(n1, f);
let p2 = FieldElement::new(n2, f);

println!("p1:     {}", p1);
println!("p2:     {}", p2);
println!("p1/p2:  {}", p1/p2);

}
