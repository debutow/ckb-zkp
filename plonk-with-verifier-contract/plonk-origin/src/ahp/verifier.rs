use ark_ff::FftField as Field;
// use ark_poly::{EvaluationDomain, Evaluations as EvaluationsOnDomain, Polynomial};
// use ark_poly_commit::{Evaluations, QuerySet};
// use ark_std::string::ToString;

use crate::ahp::indexer::IndexInfo;
use crate::ahp::{AHPForPLONK, Error};
// use crate::utils::{evaluate_first_lagrange_poly, evaluate_vanishing_poly, generator, pad_to_size};

pub struct VerifierState<'a, F: Field> {
    pub(crate) info: &'a IndexInfo<F>,

    pub(crate) alpha: Option<F>, // combination
    pub(crate) beta: Option<F>,  // permutation
    pub(crate) gamma: Option<F>, // permutation
    pub(crate) zeta: Option<F>,  // evaluation
}

pub struct FirstMsg<F: Field> {
    pub beta: F,
    pub gamma: F,
}

pub struct SecondMsg<F: Field> {
    pub alpha: F,
}

pub struct ThirdMsg<F: Field> {
    pub zeta: F,
}

impl<F: Field> AHPForPLONK<F> {
    pub fn verifier_init(info: &IndexInfo<F>) -> Result<VerifierState<'_, F>, Error> {
        Ok(VerifierState {
            info,

            alpha: None,
            beta: None,
            gamma: None,
            zeta: None,
        })
    }

    pub fn verifier_first_round(
        mut vs: VerifierState<'_, F>,
        beta_new: F,
        gamma_new: F,
    ) -> Result<(VerifierState<'_, F>, FirstMsg<F>), Error> {
        // let beta = F::rand(rng);
        // let gamma = F::rand(rng);
        // println!("beta:\n{}", beta);
        // println!("gamma:\n{}", gamma);
        // vs.beta = Some(beta);
        // vs.gamma = Some(gamma);

        vs.beta = Some(beta_new);
        vs.gamma = Some(gamma_new);

        Ok((vs, FirstMsg { beta: beta_new, gamma: gamma_new }))
    }

    pub fn verifier_second_round(
        mut vs: VerifierState<'_, F>,
        alpha_new: F,
    ) -> Result<(VerifierState<'_, F>, SecondMsg<F>), Error> {
        // let alpha = F::rand(rng);
        // vs.alpha = Some(alpha);

        vs.alpha = Some(alpha_new);

        Ok((vs, SecondMsg { alpha: alpha_new }))
    }

    pub fn verifier_third_round(
        mut vs: VerifierState<'_, F>,
        zeta_new: F,
    ) -> Result<(VerifierState<'_, F>, ThirdMsg<F>), Error> {
        // let zeta = F::rand(rng);
        // vs.zeta = Some(zeta);

        vs.zeta = Some(zeta_new);

        Ok((vs, ThirdMsg { zeta: zeta_new }))
    }

    // pub fn verifier_query_set(vs: &VerifierState<'_, F>) -> QuerySet<F> {
    //     let zeta = vs.zeta.unwrap();
    //     let g = generator(vs.info.domain_n);
    //
    //     let mut query_set = QuerySet::new();
    //
    //     query_set.insert(("w_0".into(), ("zeta".into(), zeta)));
    //     query_set.insert(("w_1".into(), ("zeta".into(), zeta)));
    //     query_set.insert(("w_2".into(), ("zeta".into(), zeta)));
    //     query_set.insert(("w_3".into(), ("zeta".into(), zeta)));
    //
    //     query_set.insert(("z".into(), ("shifted_zeta".into(), zeta * g)));
    //
    //     //query_set.insert(("sigma_0".into(), ("zeta".into(), zeta)));
    //     query_set.insert(("sigma_1".into(), ("zeta".into(), zeta)));
    //     query_set.insert(("sigma_2".into(), ("zeta".into(), zeta)));
    //     query_set.insert(("sigma_3".into(), ("zeta".into(), zeta)));
    //     //query_set.insert(("q_arith".into(), ("zeta".into(), zeta)));
    //
    //     query_set.insert(("t".into(), ("zeta".into(), zeta)));
    //     query_set.insert(("r".into(), ("zeta".into(), zeta)));
    //
    //     query_set
    // }

    // pub fn verifier_equality_check(
    //     vs: &VerifierState<'_, F>,
    //     evaluations: &Evaluations<F, F>,
    //     public_inputs: &[F],
    // ) -> Result<bool, Error> {
    //
    //     let alpha = vs.alpha.unwrap();
    //     let beta = vs.beta.unwrap();
    //     let gamma = vs.gamma.unwrap();
    //     let zeta = vs.zeta.unwrap();
    //
    //     let domain_n = vs.info.domain_n;
    //     let g = generator(domain_n);
    //     let v_zeta = evaluate_vanishing_poly(domain_n, zeta);
    //     let pi_zeta = {
    //         let pi_n = pad_to_size(public_inputs, domain_n.size());
    //         let pi_poly = EvaluationsOnDomain::from_vec_and_domain(pi_n, domain_n).interpolate();
    //         pi_poly.evaluate(&zeta)
    //     };
    //
    //
    //     let w_0_zeta = get_eval(&evaluations, "w_0", &zeta)?;
    //     let w_1_zeta = get_eval(&evaluations, "w_1", &zeta)?;
    //     let w_2_zeta = get_eval(&evaluations, "w_2", &zeta)?;
    //     let w_3_zeta = get_eval(&evaluations, "w_3", &zeta)?;
    //
    //     let z_shifted_zeta = get_eval(&evaluations, "z", &(zeta * g))?;
    //
    //     //let sigma_0_zeta = get_eval(&evaluations, "sigma_0", &zeta)?;
    //     let sigma_1_zeta = get_eval(&evaluations, "sigma_1", &zeta)?;
    //     let sigma_2_zeta = get_eval(&evaluations, "sigma_2", &zeta)?;
    //     let sigma_3_zeta = get_eval(&evaluations, "sigma_3", &zeta)?;
    //     //let q_arith_zeta = get_eval(&evaluations, "q_arith", &zeta)?;
    //
    //     let t_zeta = get_eval(&evaluations, "t", &zeta)?;
    //     let r_zeta = get_eval(&evaluations, "r", &zeta)?;
    //
    //     let l1_zeta = evaluate_first_lagrange_poly(vs.info.domain_n, zeta);
    //     let alpha_2 = alpha.square();
    //
    //     let lhs = t_zeta * v_zeta;
    //     let rhs = r_zeta + pi_zeta
    //         - z_shifted_zeta
    //             * (w_3_zeta + beta * sigma_3_zeta + gamma)
    //             * (w_1_zeta + beta * sigma_1_zeta + gamma)
    //             * (w_2_zeta + beta * sigma_2_zeta + gamma)
    //             * (w_0_zeta + gamma)
    //             * alpha
    //         - l1_zeta * alpha_2;
    //
    //
    //     println!("lhs\n{}", lhs);
    //     println!("rhs\n{}", rhs);
    //
    //     Ok(lhs == rhs)
    // }
}

// fn get_eval<F: Field>(evaluations: &Evaluations<F, F>, label: &str, point: &F) -> Result<F, Error> {
//     let eval = evaluations
//         .get(&(label.to_string(), *point))
//         .ok_or_else(|| Error::MissingEvaluation(label.to_string()))?;
//     Ok(*eval)
// }
