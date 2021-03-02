use ark_ff::FftField as Field;
use ark_poly::{univariate::DensePolynomial, Polynomial};
use ark_std::{borrow::Cow, string::String};

use crate::Map;

pub type Evals<F> = Map<String, F>;

#[derive(Clone, Debug)]
pub struct LabeledPolynomial<'a, F: Field> {
    polynomial: Cow<'a, DensePolynomial<F>>,
    label: String,
}

impl<'a, F: Field> LabeledPolynomial<'a, F> {
    pub fn new_owned(
        label: String,
        polynomial: DensePolynomial<F>,
    ) -> Self {
        Self {
            label,
            polynomial: Cow::Owned(polynomial),
        }
    }
    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn polynomial(&self) -> &DensePolynomial<F> {
        &self.polynomial
    }

    pub fn evaluate(&self, loc: &F) -> F {
        self.polynomial.evaluate(loc)
    }
}
