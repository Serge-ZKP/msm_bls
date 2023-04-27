pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        use ark_ec::Group;
        use ark_ff::{PrimeField, Field};
        // We'll use the BLS12-381 G1 curve for this example.
// This group has a prime order `r`, and is associated with a prime field `Fr`.
        use ark_test_curves::bls12_381::{G1Projective as G, Fr as ScalarField};
        use ark_std::{Zero, UniformRand, ops::Mul};

        let mut rng = ark_std::test_rng();
// Let's sample uniformly random group elements:
        let a = G::rand(&mut rng);
        let b = G::rand(&mut rng);

// We can add elements, ...
        let c = a + b;
// ... subtract them, ...
        let d = a - b;
        // ... and double them.
        assert_eq!(c + d, a.double());
// We can also negate elements, ...
        let e = -a;
        // ... and check that negation satisfies the basic group law
        assert_eq!(e + a, G::zero());

// We can also multiply group elements by elements of the corresponding scalar field
// (an act known as *scalar multiplication*)
        let scalar = ScalarField::rand(&mut rng);
        let e = c.mul(scalar);
        let f = e.mul(scalar.inverse().unwrap());
        assert_eq!(f, c);
    }

    #[test]
    fn test() {
        use ark_ec::{Group, VariableBaseMSM};
        use ark_ff::{PrimeField, Field};
        // We'll use the BLS12-381 G1 curve for this example.
// This group has a prime order `r`, and is associated with a prime field `Fr`.
        use ark_test_curves::bls12_381::{G1Projective as G, G1Affine as GAffine, Fr as ScalarField};
        use ark_std::{Zero,  UniformRand};

        let mut rng = ark_std::test_rng();
// Let's sample uniformly random group elements:
        let a = GAffine::rand(&mut rng);
        let b = GAffine::rand(&mut rng);

        let s1 = ScalarField::rand(&mut rng);
        let s2 = ScalarField::rand(&mut rng);

// Note that we're using the `GAffine` type here, as opposed to `G`.
// This is because MSMs are more efficient when the group elements are in affine form. (See below for why.)
//
// The `VariableBaseMSM` trait allows specializing the input group element representation to allow
// for more efficient implementations.
        let r = G::msm(&[a, b], &[s1, s2]).unwrap();
        assert_eq!(r, a * s1 + b * s2);



    }

    #[test]
    fn test_msm() {
        use std::time::Instant;
        use ark_ec::{Group, VariableBaseMSM};
        use ark_ff::{PrimeField, Field};
        // We'll use the BLS12-381 G1 curve for this example.
// This group has a prime order `r`, and is associated with a prime field `Fr`.
        use ark_test_curves::bls12_381::{G1Projective as G, G1Affine as GAffine, Fr as ScalarField};
        use ark_std::{Zero,  UniformRand};

        let mut rng = ark_std::test_rng();


        const DEG: usize = 1000000;
        let mut g = vec![];
        let mut s = vec![];
        for _ in 0..DEG {
            g.push(GAffine::rand(&mut rng));
            s.push(ScalarField::rand(&mut rng));
        }

        let now = Instant::now();
        let _r = G::msm(g.as_slice(), s.as_slice()).unwrap();
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);


    }




}
