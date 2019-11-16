extern crate rand;

mod fields;

use fields::U256;
use fields::PrimeField;
use fields::PrimeFieldElement;
use fields::QuadraticExtensionField;
use fields::QuadraticExtensionFieldElement;

fn main () {

let d =  U256::new([0x09a762d2d4e6dba2, 0x3fa7c65d90e7c587, 0xa16e4fd39b5ecd04, 0x476ae7bb64802287]);
let f = PrimeField::new(d);

let q = QuadraticExtensionField::new(f, [f.one(), f.zero(), f.one()]);

let q1 = q.rand();
let q2 = q.rand();


println!("{}", q1);
println!("{}", q2);

println!("{}", q1 + q2);

let two = f.one() + f.one();
let five = two + two + f.one();

let r1 = QuadraticExtensionFieldElement::new(q, [five, two, f.one()]);
let r2 = QuadraticExtensionFieldElement::new(q, [two, f.one(), f.one()]);
let dr = QuadraticExtensionFieldElement::new(q, [two, two, two]);

let product = q.double_poly_mod(q.poly_long_mul(r1, r2), dr);

println!("{:?}", product)

}
