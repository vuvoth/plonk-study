use ark_ff::Field;

use ark_bn254::Fq as F;
use ark_bn254::Bn254;

fn main() {
    let a = F::from(10000);
    let b = F::from(10);

    let c = a + b;
    println!("{}", c);
}
