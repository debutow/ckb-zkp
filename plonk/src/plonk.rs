use ark_ff::{to_bytes, FftField as Field};
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::{PCUniversalParams, PolynomialCommitment};

use ark_std::marker::PhantomData;
use digest::Digest;
use rand_core::RngCore;

use crate::composer::Composer;
use crate::data_structures::*;
use crate::protocol::{PreprocessorKeys, Prover, Verifier};
use crate::rng::FiatShamirRng;
use crate::Error;

pub struct Plonk<
    F: Field,
    D: Digest,
    PC: PolynomialCommitment<F, DensePolynomial<F>>,
> {
    _field: PhantomData<F>,
    _digest: PhantomData<D>,
    _pc: PhantomData<PC>,
}

impl<
        F: Field,
        D: Digest,
        PC: PolynomialCommitment<F, DensePolynomial<F>>,
    > Plonk<F, D, PC>
{
    pub const PROTOCOL_NAME: &'static [u8] = b"PLONK";

    pub fn setup<R: RngCore>(
        max_degree: usize,
        rng: &mut R,
    ) -> Result<UniversalParams<F, PC>, Error<PC::Error>> {
        PC::setup(max_degree, None, rng).map_err(Error::from_pc_err)
    }

    pub fn keygen(
        srs: &UniversalParams<F, PC>,
        cs: &Composer<F>,
        ks: [F; 4],
    ) -> Result<(ProverKey<F, PC>, VerifierKey<F, PC>), Error<PC::Error>>
    {
        let keys = PreprocessorKeys::generate(cs, ks)?;
        if srs.max_degree() < keys.size() {
            return Err(Error::CircuitTooLarge);
        }

        let (ck, vk) = PC::trim(srs, keys.size(), 0, None)
            .map_err(Error::from_pc_err)?;
        let (comms, rands) = PC::commit(&ck, keys.iter(), None)
            .map_err(Error::from_pc_err)?;
        let comms =
            comms.into_iter().map(|c| c.commitment().clone()).collect();

        let vk = VerifierKey {
            info: keys.info(),
            comms,
            rk: vk,
        };
        let pk = ProverKey { keys, rands, ck };

        Ok((pk, vk))
    }

    pub fn prove(
        pk: &ProverKey<F, PC>,
        cs: &Composer<F>,
        zk_rng: &mut dyn RngCore,
    ) -> Result<Proof<F, PC>, Error<PC::Error>> {
        let mut p = Prover::init(cs, &pk.keys)?;
        let mut v = Verifier::init(pk.keys.info())?;
        let pi = p.public_input();

        let mut fs_rng = FiatShamirRng::<D>::from_seed(
            &to_bytes![&Self::PROTOCOL_NAME, &pi].unwrap(),
        );

        let first_oracles = p.first_round(&cs)?;
        let (first_comms, first_rands) =
            PC::commit(&pk.ck, first_oracles.iter(), Some(zk_rng))
                .map_err(Error::from_pc_err)?;
        fs_rng.absorb(&to_bytes![first_comms].unwrap());
        let first_msg = v.first_round(&mut fs_rng)?;

        let second_oracles = p.second_round(&first_msg)?;
        let (second_comms, second_rands) =
            PC::commit(&pk.ck, second_oracles.iter(), Some(zk_rng))
                .map_err(Error::from_pc_err)?;
        fs_rng.absorb(&to_bytes![second_comms].unwrap());
        let second_msg = v.second_round(&mut fs_rng)?;

        let third_oracles = p.third_round(&second_msg)?;
        let (thrid_comms, third_rands) =
            PC::commit(&pk.ck, third_oracles.iter(), Some(zk_rng))
                .map_err(Error::from_pc_err)?;
        fs_rng.absorb(&to_bytes![third_rands].unwrap());
        let third_msg = v.third_round(&mut fs_rng)?;

        Err(Error::Other)
    }
}

// #[cfg(test)]
// mod test {
//     use ark_bls12_381::Fr;
//     use ark_ff::{One, Zero};
//     use ark_std::test_rng;

//     use crate::composer::Composer;
//     use crate::Error;

//     use super::prover::Prover;
//     use super::verifier::Verifier;

//     fn run() -> Result<bool, Error> {
//         let ks = [
//             Fr::one(),
//             Fr::from(7_u64),
//             Fr::from(13_u64),
//             Fr::from(17_u64),
//         ];
//         let rng = &mut test_rng();

//         // compose
//         let mut cs = Composer::new();
//         let one = Fr::one();
//         let two = one + one;
//         let three = two + one;
//         let four = two + two;
//         let var_one = cs.alloc_and_assign(one);
//         let var_two = cs.alloc_and_assign(two);
//         let var_three = cs.alloc_and_assign(three);
//         let var_four = cs.alloc_and_assign(four);

//         cs.create_add_gate(
//             (var_one, one),
//             (var_three, one),
//             var_four,
//             None,
//             Fr::zero(),
//             Fr::zero(),
//         );
//         cs.create_add_gate(
//             (var_one, one),
//             (var_two, one),
//             var_three,
//             None,
//             Fr::zero(),
//             Fr::zero(),
//         );
//         cs.constrain_to_constant(var_four, Fr::zero(), -four);
//         println!("size of the circuit: {}", cs.size());

//         // init
//         print!("initializing prover...");
//         let mut p = Prover::init(&cs, ks)?;
//         println!("done");

//         print!("initializing verifier...");
//         let mut v = Verifier::init(&cs)?;
//         println!("done");
//         // first round
//         print!("prover: first round...");
//         let first_oracles = p.first_round(&cs)?;
//         println!("done");

//         print!("verifier: first round...");
//         let first_msg = v.first_round(rng)?;
//         println!("done");

//         // second round
//         print!("prover: second round...");
//         let second_oracles = p.second_round(&first_msg)?;
//         println!("done");

//         print!("verifier: second round...");
//         let second_msg = v.second_round(rng)?;
//         println!("done");

//         // third round
//         print!("prover: third round...");
//         let third_oracles = p.third_round(&second_msg)?;
//         println!("done");

//         print!("verifier: third round...");
//         let third_msg = v.third_round(rng)?;
//         println!("done");

//         // finalize
//         print!("prover: evaluating...");
//         let evals = p.evaluate(
//             &third_msg,
//             &first_oracles,
//             &second_oracles,
//             &third_oracles,
//         );
//         println!("done");

//         print!("verifier: equality checking...");
//         let is_equal = v.check_equality(&evals);
//         println!("done");

//         is_equal
//     }

//     #[test]
//     fn test() {
//         let result = run().unwrap();
//         assert!(result);
//     }
// }

// pub fn evaluate<'a>(
//     &self,
//     third_msg: &ThirdMsg<F>,
//     first_oracles: &FirstOracles<F>,
//     second_oracles: &SecondOracles<F>,
//     third_oracles: &ThirdOracles<F>,
// ) -> Evaluations<F> {
//     let ThirdMsg { zeta } = third_msg;

//     let mut evals = Evaluations::new();
//     // evaluation of [w_0, ..., w_3]
//     let w_zeta: Vec<_> =
//         first_oracles.iter().map(|w| w.evaluate(zeta)).collect();

//     // evaluation of z_shifted
//     let gen = get_generator(self.pk.domain_n());
//     let z_shifted_zeta = second_oracles.z.evaluate(&(gen * zeta));

//     // evaluation of t
//     let t_zeta: F = {
//         let zeta_n = zeta.pow(&[self.size() as u64]);
//         let zeta_2n = zeta_n.square();

//         third_oracles
//             .iter()
//             .zip(vec![F::one(), zeta_n, zeta_2n, zeta_n * zeta_2n])
//             .map(|(p, z)| p.evaluate(zeta) * z)
//             .sum()
//     };

//     let (q_arith_zeta, sigma_0_zeta, sigma_1_zeta, sigma_2_zeta, r_zeta) = {
//         let alpha = &self.alpha.unwrap();
//         let beta = &self.beta.unwrap();
//         let gamma = &self.gamma.unwrap();

//         let arithmetic_key = self.pk.arithmetic_key();
//         let (q_arith_zeta, arith_lin) = arithmetic_key
//             .compute_linearisation(
//                 &w_zeta[0], &w_zeta[1], &w_zeta[2], &w_zeta[3], zeta,
//             );

//         let permutation_key = self.pk.permutation_key();
//         let (sigma_0_zeta, sigma_1_zeta, sigma_2_zeta, perm_lin) =
//             permutation_key.compute_linearisation(
//                 (&w_zeta[0], &w_zeta[1], &w_zeta[2], &w_zeta[3]),
//                 &z_shifted_zeta,
//                 &second_oracles.z.polynomial(),
//                 beta,
//                 gamma,
//                 zeta,
//                 alpha,
//             );

//         (
//             q_arith_zeta,
//             sigma_0_zeta,
//             sigma_1_zeta,
//             sigma_2_zeta,
//             (arith_lin + perm_lin).evaluate(zeta),
//         )
//     };

//     evals.insert("w_0".into(), w_zeta[0]);
//     evals.insert("w_1".into(), w_zeta[1]);
//     evals.insert("w_2".into(), w_zeta[2]);
//     evals.insert("w_3".into(), w_zeta[3]);
//     evals.insert("z_shifted".into(), z_shifted_zeta);
//     evals.insert("q_arith".into(), q_arith_zeta);
//     evals.insert("sigma_0".into(), sigma_0_zeta);
//     evals.insert("sigma_1".into(), sigma_1_zeta);
//     evals.insert("sigma_2".into(), sigma_2_zeta);
//     evals.insert("t".into(), t_zeta);
//     evals.insert("r".into(), r_zeta);

//     evals
// }
